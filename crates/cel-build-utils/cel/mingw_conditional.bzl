"""
Conditional MinGW-w64 toolchain for the x86_64-pc-windows-gnu target.

Bazel's auto-configured C++ toolchain on a Windows host is MSVC, which
produces objects ABI-incompatible with the GNU toolchain rustc uses for
*-pc-windows-gnu: the cxx bridge is compiled by the MinGW g++ that cc-rs
finds on PATH and the final link is a GNU link, so libcel must be compiled
by the same g++. This extension generates a cc toolchain from the MinGW
tools on PATH whenever the Rust target being built is
x86_64-pc-windows-gnu, and registers it from the root MODULE.bazel: Bzlmod
ranks root-module registrations above the auto-configured local toolchain,
so it wins over MSVC on the platform's shared [cpu:x86_64, os:windows]
constraint set (the same ranking the rules_cross_rs registration relies on
for musl; the gnu platform in BUILD.bazel deliberately declares the same
constraints as the msvc platform for that reason).

Activation rules (read from the Bazel process environment, which cargo sets
for its build scripts and their bazel subprocesses):
  - TARGET must equal x86_64-pc-windows-gnu.
  - CROSS_TOOLCHAIN_PREFIX / CROSS_SYSROOT must be unset: under cross-rs the
    rules_cross_rs toolchain owns the build instead.
Otherwise a stub toolchain with impossible constraints is generated, so
every other target (including x86_64-pc-windows-msvc on this same host)
keeps resolving the auto-configured toolchain exactly as before.
"""

_MINGW_TRIPLE = "x86_64-pc-windows-gnu"
_MINGW_TOOL_PREFIX = "x86_64-w64-mingw32-"
_MINGW_DUMP_MACHINE = "x86_64-w64-mingw32"
_MINGW_TOOLCHAIN_IDENTIFIER = "mingw_toolchain_" + _MINGW_TRIPLE

_REQUIRED_TOOLS = ["gcc", "g++", "ar"]
_OPTIONAL_TOOLS = ["ld", "strip", "nm", "objcopy", "objdump"]

def _find_tool(repository_ctx, tool):
    """Resolve a tool under its prefixed and plain names.

    Bazel's which() retries with the platform's executable suffix, so bare
    names resolve gcc.exe on Windows; MSYS2 only ships plain binutils (the
    prefixed x86_64-w64-mingw32-ar does not exist there), while standalone
    mingw-w64 distributions and Linux cross packages ship both.
    """
    for name in [_MINGW_TOOL_PREFIX + tool, tool]:
        path = repository_ctx.which(name)
        if path != None:
            return path
    return None

def _find_mingw_compiler(repository_ctx, tool):
    """Resolve a compiler that actually targets x86_64-w64-mingw32.

    -dumpmachine rejects near-misses such as the MSYS POSIX /usr/bin/gcc or
    a host-native gcc on a Linux machine, which would otherwise silently
    miscompile libcel.
    """
    for name in [_MINGW_TOOL_PREFIX + tool, tool]:
        path = repository_ctx.which(name)
        if path == None:
            continue
        result = repository_ctx.execute([str(path), "-dumpmachine"])
        if (result.return_code == 0 and
            result.stdout.strip() == _MINGW_DUMP_MACHINE):
            return path
    return None

def _detect_tool_paths(repository_ctx):
    gcc = _find_mingw_compiler(repository_ctx, "gcc")
    gxx = _find_mingw_compiler(repository_ctx, "g++")
    ar = _find_tool(repository_ctx, "ar")
    if gcc == None or gxx == None or ar == None:
        fail(
            "x86_64-pc-windows-gnu requires a MinGW-w64 C++ toolchain on " +
            "PATH: looked for 'x86_64-w64-mingw32-gcc'/'gcc' and " +
            "'x86_64-w64-mingw32-g++'/'g++' reporting -dumpmachine " +
            _MINGW_DUMP_MACHINE + ", plus 'ar'. Build from an MSYS2 UCRT64 " +
            "shell (or any host with a mingw-w64 toolchain on PATH), or " +
            "build via cross-rs.",
        )

    tool_paths = {
        "gcc": str(gcc),
        "g++": str(gxx),
        "ar": str(ar),
    }

    # Optional tools fall back to the compiler, as rules_cross_rs does; none
    # of them is on a critical path for building and archiving libcel.
    for tool in _OPTIONAL_TOOLS:
        path = _find_tool(repository_ctx, tool)
        tool_paths[tool] = str(path) if path != None else tool_paths["gcc"]
    tool_paths["gcov"] = tool_paths["gcc"]
    tool_paths["dwp"] = tool_paths["gcc"]

    # Bazel routes c++ link actions at the "ld" tool. Raw ld has no driver
    # semantics: it rejects the -Wl, flag prefixes (and cannot find the
    # libc.a / libdl.a mingw does not ship), while exec-config binaries
    # (protoc) need the CRT objects only the gcc driver adds. Use the C++
    # driver, which also links libstdc++ (g++ as "ld" is what
    # unix_cc_configure effectively does). The toolchain links its
    # executables statically (-static in default_link_flags): Bazel runs
    # exec tools with a minimal environment where the MSYS2 runtime DLLs
    # (/ucrt64/bin) are not on PATH, which would fail with 0xC0000135.
    tool_paths["ld"] = tool_paths["g++"]
    tool_paths["cpp"] = tool_paths["g++"]

    return tool_paths

def _normalize_path(path):
    """Canonicalize separators and dot segments of a printed include path.

    gcc prints forward-slash paths with ".." segments (for example
    C:/msys64/ucrt64/bin/../include); Bazel would resolve such relative
    parts against the exec root, so fold them textually.
    """
    path = path.replace("\\", "/")
    absolute = path.startswith("/")
    parts = [p for p in path.split("/") if p not in ("", ".")]
    out = []
    for part in parts:
        if part == "..":
            # Never pop a drive-letter or another leading "..".
            if out and out[-1] != ".." and not out[-1].endswith(":"):
                out.pop()
            else:
                out.append(part)
        else:
            out.append(part)
    result = "/".join(out)
    if absolute:
        result = "/" + result
    return result

def _detect_builtin_include_directories(repository_ctx, gcc_path):
    """Detect builtin include directories from the compiler."""
    if not gcc_path:
        return []

    # gcc cannot read /dev/null on Windows hosts; the null device is NUL.
    # os.name is the raw platform name ("windows", "windows 11", "linux"),
    # so match by prefix.
    os_name = repository_ctx.os.name.lower()
    null_input = "NUL" if os_name.startswith("windows") else "/dev/null"
    result = repository_ctx.execute([
        gcc_path,
        "-E", "-v", "-x", "c++", null_input,
    ], timeout = 10)

    if result.return_code != 0:
        # Probe failed; return nothing rather than guess. An empty list
        # makes Bazel's include validation reject the C++ standard headers
        # ("absolute path inclusion(s) found" errors), so a silent probe
        # failure surfaces quickly at compile time.
        return []

    # Parse the include search list; accept the /-rooted paths Unix gcc
    # prints and the drive-letter paths (C:/...) MinGW gcc prints.
    lines = result.stderr.split("\n")
    include_dirs = []
    in_include_section = False
    for line in lines:
        line = line.strip()
        if line == "#include <...> search starts here:":
            in_include_section = True
            continue
        elif line == "End of search list.":
            break
        elif in_include_section:
            # Drop trailing annotations like "(framework directory)".
            path = line.split(" ")[0]
            path = path.replace("\\", "/")
            if path.startswith("/") or (len(path) >= 2 and path[1] == ":"):
                include_dirs.append(_normalize_path(path))

    return include_dirs

def _get_host_exec_constraints(repository_ctx):
    """Map the host Bazel runs on to @platforms exec constraints.

    The tools referenced by the generated toolchain only run on the machine
    this repository rule executes on. Returns an empty list when the host
    cannot be mapped; a toolchain without exec constraints matches any
    execution platform, which is acceptable because the repository is
    generated per-host anyway. os.name is the raw platform name
    ("windows", "windows 11", "linux", ...), so match by prefix.
    """
    os_name = repository_ctx.os.name.lower()
    arch = repository_ctx.os.arch

    os_constraint = None
    if os_name.startswith("windows"):
        os_constraint = "windows"
    elif os_name.startswith("linux"):
        os_constraint = "linux"
    elif os_name.startswith("mac") or os_name.startswith("darwin"):
        os_constraint = "osx"
    elif os_name.startswith("freebsd"):
        os_constraint = "freebsd"
    elif os_name.startswith("openbsd"):
        os_constraint = "openbsd"
    elif os_name.startswith("netbsd"):
        os_constraint = "netbsd"

    arch_map = {
        "x86_64": "x86_64",
        "amd64": "x86_64",
        "aarch64": "aarch64",
        "arm64": "aarch64",
        "i386": "x86_32",
        "i486": "x86_32",
        "i586": "x86_32",
        "i686": "x86_32",
    }
    arch_constraint = arch_map.get(arch)

    if not os_constraint or not arch_constraint:
        return []
    return [
        "@platforms//os:" + os_constraint,
        "@platforms//cpu:" + arch_constraint,
    ]

# The cc_toolchain_config rule, written into the generated repository. Tool
# paths and include directories arrive as attributes so this file is static.
# The feature set mirrors rules_cross_rs (proven on cross builds) plus the
# pieces a Windows host needs:
#   - archive_param_file: cc_static_library passes ~600 object paths to the
#     archiver inline otherwise, exceeding CreateProcessW's 32767-character
#     limit (binutils ar accepts @response files, as does llvm-ar).
#   - "ld" is the gcc driver (see _detect_tool_paths).
#   - default_link_flags carries no -lc/-ldl/--build-id: mingw has no
#     libc.a/libdl.a and --build-id is ELF-only.
_TOOLCHAIN_CONFIG_BZL = '''# Generated by mingw_toolchain_repository; do not edit.
load("@rules_cc//cc:defs.bzl", "cc_common")
load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "ACTION_NAMES")
load(
    "@bazel_tools//tools/cpp:cc_toolchain_config_lib.bzl",
    "artifact_name_pattern",
    "feature",
    flag_set_ = "flag_set",
    "flag_group",
    "tool_path",
    "with_feature_set",
)

_ALL_COMPILE_ACTIONS = [
    ACTION_NAMES.c_compile,
    ACTION_NAMES.cpp_compile,
    ACTION_NAMES.linkstamp_compile,
    ACTION_NAMES.assemble,
    ACTION_NAMES.preprocess_assemble,
    ACTION_NAMES.cpp_header_parsing,
    ACTION_NAMES.cpp_module_compile,
    ACTION_NAMES.cpp_module_codegen,
    ACTION_NAMES.lto_backend,
    ACTION_NAMES.clif_match,
]

_ALL_CXX_COMPILE_ACTIONS = [
    ACTION_NAMES.cpp_compile,
    ACTION_NAMES.cpp_header_parsing,
    ACTION_NAMES.cpp_module_compile,
    ACTION_NAMES.cpp_module_codegen,
]

_ALL_LINK_ACTIONS = [
    ACTION_NAMES.cpp_link_executable,
    ACTION_NAMES.cpp_link_dynamic_library,
    ACTION_NAMES.cpp_link_nodeps_dynamic_library,
]

# flag_set wrapper allowing the "simple" flags-list form.
def flag_set(flags = None, features = None, not_features = None, **kwargs):
    if flags:
        if kwargs.get("flag_groups"):
            fail("Cannot set flags and flag_groups")
        kwargs["flag_groups"] = [flag_group(flags = flags)]
    if features or not_features:
        if kwargs.get("with_features"):
            fail("Cannot set features/not_feature and with_features")
        kwargs["with_features"] = [with_feature_set(
            features = features or [],
            not_features = not_features or [],
        )]
    return flag_set_(**kwargs)

def _mingw_toolchain_config_impl(ctx):
    tool_paths = [
        tool_path(name = "gcc", path = ctx.attr.gcc_path),
        tool_path(name = "g++", path = ctx.attr.gxx_path),
        tool_path(name = "cpp", path = ctx.attr.gxx_path),
        # The C++ driver, not raw ld: Bazel routes link actions at "ld", and
        # raw ld rejects the -Wl, flag prefixes and lacks driver semantics
        # (CRT objects, C++ runtime). See the repository rule for details.
        tool_path(name = "ld", path = ctx.attr.gxx_path),
        tool_path(name = "ar", path = ctx.attr.ar_path),
        tool_path(name = "strip", path = ctx.attr.strip_path),
        tool_path(name = "nm", path = ctx.attr.nm_path),
        tool_path(name = "objcopy", path = ctx.attr.objcopy_path),
        tool_path(name = "objdump", path = ctx.attr.objdump_path),
        tool_path(name = "gcov", path = ctx.attr.gcc_path),
        tool_path(name = "dwp", path = ctx.attr.gcc_path),
    ]

    features = []

    features.append(
        feature(
            name = "default_compile_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_COMPILE_ACTIONS,
                    flags = [
                        "-no-canonical-prefixes",
                        "-fdata-sections",
                        "-ffunction-sections",
                        # No -g: unlike MSVC's side-by-side .pdb, gcc embeds
                        # debug info in the objects, which would inflate
                        # libcel.a ~6x (1.2GB with -g vs ~200MB without).
                        # rustc packs the archive into the crate rlib, and
                        # every test/example binary link re-reads it, so the
                        # size directly turns into disk I/O. Add
                        # --cxxopt=-g to the bazelrc mingw section if C++
                        # symbols are ever needed.
                        "-fPIC",
                    ],
                ),
                flag_set(
                    actions = _ALL_CXX_COMPILE_ACTIONS,
                    flags = [
                        "-std=c++17",
                    ],
                ),
            ],
        ),
    )

    # MinGW-safe link flags. Link actions run through the "gcc" tool (the C
    # driver), so the C++ runtime is explicit: -lstdc++ and -lwinpthread
    # (std::thread on MSYS2's posix-threads gcc). -static resolves them and
    # the CRT to the .a variants, which makes exec-config binaries
    # self-contained: Bazel runs them with a minimal environment where the
    # MSYS2 runtime DLLs (/ucrt64/bin) are not on PATH (0xC0000135
    # otherwise). No -lc/-ldl (mingw ships no libc.a/libdl.a) and no
    # --build-id (ELF-only). -ldbghelp mirrors the MSVC-only
    # "#pragma comment(lib, ...)" in abseil's symbolize_win32.inc, which gcc
    # ignores (-Wunknown-pragmas).
    features.append(
        feature(
            name = "default_link_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_LINK_ACTIONS,
                    flags = [
                        "-no-canonical-prefixes",
                        "-static",
                        "-Wl,--gc-sections",
                        "-lm",
                        "-lstdc++",
                        "-lwinpthread",
                        "-ldbghelp",
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "user_compile_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_COMPILE_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["%{user_compile_flags}"],
                            iterate_over = "user_compile_flags",
                            expand_if_available = "user_compile_flags",
                        ),
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "user_link_flags",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_LINK_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["%{user_link_flags}"],
                            iterate_over = "user_link_flags",
                            expand_if_available = "user_link_flags",
                        ),
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "preprocessor_defines",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_COMPILE_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["-D%{preprocessor_defines}"],
                            iterate_over = "preprocessor_defines",
                        ),
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "include_paths",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_COMPILE_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["-include", "%{includes}"],
                            iterate_over = "includes",
                            expand_if_available = "includes",
                        ),
                    ],
                ),
                flag_set(
                    actions = _ALL_COMPILE_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["-iquote", "%{quote_include_paths}"],
                            iterate_over = "quote_include_paths",
                        ),
                        flag_group(
                            flags = ["-I%{include_paths}"],
                            iterate_over = "include_paths",
                        ),
                        flag_group(
                            flags = ["-isystem", "%{system_include_paths}"],
                            iterate_over = "system_include_paths",
                        ),
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "library_search_directories",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_LINK_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["-L%{library_search_directories}"],
                            iterate_over = "library_search_directories",
                            expand_if_available = "library_search_directories",
                        ),
                    ],
                ),
            ],
        ),
    )

    features.append(
        feature(
            name = "linkstamp_paths",
            enabled = True,
            flag_sets = [
                flag_set(
                    actions = _ALL_LINK_ACTIONS,
                    flag_groups = [
                        flag_group(
                            flags = ["%{linkstamp_paths}"],
                            iterate_over = "linkstamp_paths",
                            expand_if_available = "linkstamp_paths",
                        ),
                    ],
                ),
            ],
        ),
    )

    # Bazel spills archiver arguments into an @response file when this
    # feature is enabled (binutils ar and llvm-ar both accept @response
    # files). Without it, archiving a large cc_static_library exceeds
    # CreateProcessW's 32767-character command line limit on Windows hosts.
    features.append(
        feature(
            name = "archive_param_file",
            enabled = True,
        ),
    )

    return cc_common.create_cc_toolchain_config_info(
        ctx = ctx,
        features = features,
        # The PE linker auto-appends .exe when the -o name lacks an
        # extension, so executable artifacts must be declared with the
        # suffix or cc_binary actions fail with "output was not created".
        # Static libraries keep gcc's default lib<name>.a naming, which the
        # GNU linker's -lcel resolves.
        artifact_name_patterns = [
            artifact_name_pattern(
                category_name = "executable",
                prefix = "",
                extension = ".exe",
            ),
        ],
        toolchain_identifier = "mingw_toolchain_x86_64-pc-windows-gnu",
        host_system_name = "local",
        target_system_name = "x86_64-pc-windows-gnu",
        target_cpu = "x86_64",
        target_libc = "mingw",
        compiler = "gcc",
        abi_version = "unknown",
        abi_libc_version = "unknown",
        tool_paths = tool_paths,
        cxx_builtin_include_directories = ctx.attr.builtin_include_directories,
    )

mingw_toolchain_config = rule(
    implementation = _mingw_toolchain_config_impl,
    attrs = {
        "gcc_path": attr.string(mandatory = True),
        "gxx_path": attr.string(mandatory = True),
        "ar_path": attr.string(mandatory = True),
        "strip_path": attr.string(),
        "nm_path": attr.string(),
        "objcopy_path": attr.string(),
        "objdump_path": attr.string(),
        "builtin_include_directories": attr.string_list(),
    },
    provides = [CcToolchainConfigInfo],
)
'''

_BUILD_TEMPLATE = '''# Generated by mingw_toolchain_repository; do not edit.
load("@rules_cc//cc:defs.bzl", "cc_toolchain")
load(":toolchain_config.bzl", "mingw_toolchain_config")

package(default_visibility = ["//visibility:public"])

mingw_toolchain_config(
    name = "toolchain_config",
    gcc_path = "{gcc_path}",
    gxx_path = "{gxx_path}",
    ar_path = "{ar_path}",
    strip_path = "{strip_path}",
    nm_path = "{nm_path}",
    objcopy_path = "{objcopy_path}",
    objdump_path = "{objdump_path}",
    builtin_include_directories = {builtin_include_directories},
)

cc_toolchain(
    name = "toolchain",
    toolchain_identifier = "{toolchain_identifier}",
    toolchain_config = ":toolchain_config",
    all_files = ":empty",
    compiler_files = ":empty",
    linker_files = ":empty",
    dwp_files = ":empty",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 1,
)

filegroup(name = "empty")

toolchain(
    name = "toolchain_definition",
    toolchain = ":toolchain",
    toolchain_type = "@bazel_tools//tools/cpp:toolchain_type",
    exec_compatible_with = {exec_constraints},
    target_compatible_with = [
        "@platforms//cpu:x86_64",
        "@platforms//os:windows",
    ],
)
'''

_STUB_BUILD_TEMPLATE = '''# Stub repository for mingw_toolchain: the target is not
# x86_64-pc-windows-gnu, or cross-rs owns toolchain selection.

package(default_visibility = ["//visibility:public"])

filegroup(name = "empty")

# Stub toolchain that will never be selected due to impossible constraints.
toolchain(
    name = "toolchain_definition",
    toolchain = ":empty",
    toolchain_type = "@bazel_tools//tools/cpp:toolchain_type",
    exec_compatible_with = [
        "@platforms//os:none",  # Impossible constraint
    ],
    target_compatible_with = [
        "@platforms//os:none",  # Impossible constraint
    ],
)
'''

def _mingw_toolchain_repository_impl(repository_ctx):
    target = repository_ctx.os.environ.get("TARGET")
    cross_prefix = repository_ctx.os.environ.get("CROSS_TOOLCHAIN_PREFIX")
    cross_sysroot = repository_ctx.os.environ.get("CROSS_SYSROOT")

    # Only take charge of the gnu triple, and never when cross-rs (or a
    # manually prefixed host toolchain) drives the build: rules_cross_rs
    # generates the toolchain for those, exactly as on musl/wasm.
    if (target != _MINGW_TRIPLE or cross_prefix != None or
        cross_sysroot != None):
        repository_ctx.file("BUILD.bazel", _STUB_BUILD_TEMPLATE)
        return

    tool_paths = _detect_tool_paths(repository_ctx)
    include_dirs = _detect_builtin_include_directories(
        repository_ctx,
        tool_paths["gcc"],
    )
    exec_constraints = _get_host_exec_constraints(repository_ctx)

    repository_ctx.file("toolchain_config.bzl", _TOOLCHAIN_CONFIG_BZL)
    repository_ctx.file("BUILD.bazel", _BUILD_TEMPLATE.format(
        gcc_path = tool_paths["gcc"],
        gxx_path = tool_paths["g++"],
        ar_path = tool_paths["ar"],
        strip_path = tool_paths["strip"],
        nm_path = tool_paths["nm"],
        objcopy_path = tool_paths["objcopy"],
        objdump_path = tool_paths["objdump"],
        builtin_include_directories = repr(include_dirs),
        toolchain_identifier = _MINGW_TOOLCHAIN_IDENTIFIER,
        target_triple = _MINGW_TRIPLE,
        exec_constraints = repr(exec_constraints),
    ))

mingw_toolchain_repository = repository_rule(
    implementation = _mingw_toolchain_repository_impl,
    local = True,
    environ = [
        "TARGET",
        "CROSS_TOOLCHAIN_PREFIX",
        "CROSS_SYSROOT",
        "PATH",
    ],
)

def _conditional_mingw_extension_impl(_):
    """Module extension implementation for the conditional MinGW toolchain.

    Always create the repository; it stubs itself out unless cargo is
    building x86_64-pc-windows-gnu outside cross-rs, so use_repo() always
    succeeds and other targets are unaffected.
    """
    mingw_toolchain_repository(
        name = "mingw_toolchain",
    )

conditional_mingw_toolchain = module_extension(
    implementation = _conditional_mingw_extension_impl,
    environ = [
        "TARGET",
        "CROSS_TOOLCHAIN_PREFIX",
        "CROSS_SYSROOT",
        "PATH",
    ],
)
