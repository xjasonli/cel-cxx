"""
Conditional Android NDK support for build-linux.

This extension allows build-linux to support both Linux and Android platforms
by only setting up Android NDK when ANDROID_NDK_HOME is available.
"""

load("@rules_android_ndk//:rules.bzl", "android_ndk_repository")

def _stub_android_ndk_impl(repository_ctx):
    """Implementation for stub Android NDK repository."""
    build_content = '''# Stub Android NDK repository - Android NDK not available

package(default_visibility = ["//visibility:public"])

# Export target_systems.bzl for compatibility
exports_files(["target_systems.bzl"])

# Stub libraries
filegroup(name = "cpufeatures", srcs = [])
filegroup(name = "native_app_glue", srcs = [])
filegroup(name = "toolchain", srcs = [])

# NOTE: No toolchain_* targets, so register_toolchains("@androidndk//:all") matches nothing
'''
    repository_ctx.file("BUILD.bazel", build_content)
    
    target_systems_content = '''# Stub target_systems.bzl
TARGET_SYSTEM_NAMES = []
CPU_CONSTRAINT = {}
'''
    repository_ctx.file("target_systems.bzl", target_systems_content)

# Define the stub repository rule at module level
_stub_android_ndk = repository_rule(
    implementation = _stub_android_ndk_impl,
    local = True,
)

def _conditional_android_ndk_extension_impl(module_ctx):
    """
    Module extension that conditionally creates Android NDK repository.
    
    This directly mimics the logic from @rules_android_ndk//:extension.bzl
    but adds a check for ANDROID_NDK_HOME before proceeding.
    """
    
    # Check if Android NDK is available
    android_ndk_home = module_ctx.os.environ.get("ANDROID_NDK_HOME")
    
    if not android_ndk_home or android_ndk_home.strip() == "":
        # No Android NDK available - create a minimal stub repository
        print("INFO: ANDROID_NDK_HOME not set. Android builds will be unavailable.")
        
        _stub_android_ndk(name = "androidndk")
        return
    
    # Android NDK is available - use the official rules_android_ndk logic
    print("INFO: ANDROID_NDK_HOME found: {}".format(android_ndk_home))
    print("INFO: Setting up Android NDK toolchains...")
    
    # This is copied from @rules_android_ndk//:extension.bzl
    # We find the root module that has configure tags
    root_modules = [m for m in module_ctx.modules if m.is_root and m.tags.configure]
    if len(root_modules) > 1:
        fail("Expected at most one root module, found {}".format(", ".join([x.name for x in root_modules])))

    if root_modules:
        module = root_modules[0]
    else:
        module = module_ctx.modules[0]

    kwargs = {}
    if module.tags.configure:
        kwargs["api_level"] = module.tags.configure[0].api_level
        kwargs["path"] = module.tags.configure[0].path
    
    # If no explicit path is provided, use ANDROID_NDK_HOME
    if "path" not in kwargs:
        kwargs["path"] = android_ndk_home

    android_ndk_repository(
        name = "androidndk",
        **kwargs
    )

conditional_android_ndk = module_extension(
    implementation = _conditional_android_ndk_extension_impl,
    environ = ["ANDROID_NDK_HOME"],
    tag_classes = {
        "configure": tag_class(attrs = {
            "path": attr.string(),
            "api_level": attr.int(),
        }),
    },
)
