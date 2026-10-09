"""Mac Catalyst ABI configuration for the registered Apple C++ toolchain.

Catalyst uses the macOS SDK and Xcode tools, but an iOS -macabi target triple.
Reuse apple_cc_configure_extension's macOS platforms/toolchains and the public
extra_enabled_features interface rather than modifying apple_support sources.
The additional ABI constraint keeps these flags out of macOS execution tools.
"""

load("@rules_cc//cc/toolchains:args.bzl", "cc_args")
load("@rules_cc//cc/toolchains:feature.bzl", "cc_feature")
load("@rules_cc//cc/toolchains:feature_set.bzl", "cc_feature_set")

def _catalyst_target_impl(ctx):
    arm64 = ctx.target_platform_has_constraint(ctx.attr._arm64[platform_common.ConstraintValueInfo])
    arch = "arm64" if arm64 else "x86_64"
    minimum = apple_common.dotted_version("14.0" if arm64 else "13.1")
    xcode = ctx.attr._xcode_config[apple_common.XcodeVersionConfig]
    version = max(minimum, xcode.minimum_os_for_platform_type(apple_common.platform_type.ios))
    return [platform_common.TemplateVariableInfo({
        "CATALYST_TARGET": "{}-apple-ios{}-macabi".format(arch, version),
    })]

_catalyst_target = rule(
    implementation = _catalyst_target_impl,
    attrs = {
        "_arm64": attr.label(default = "@platforms//cpu:arm64"),
        "_xcode_config": attr.label(default = configuration_field(
            fragment = "apple",
            name = "xcode_config_label",
        )),
    },
    fragments = ["apple"],
)

def apple_catalyst_platforms():
    """Declare the two Catalyst platforms and their compile/link features."""
    native.constraint_setting(name = "apple_abi")
    native.constraint_value(
        name = "catalyst_abi",
        constraint_setting = ":apple_abi",
    )
    native.config_setting(
        name = "catalyst_abi_enabled",
        constraint_values = [":catalyst_abi"],
    )

    for rust_arch, apple_arch in [("aarch64", "arm64"), ("x86_64", "x86_64")]:
        native.platform(
            name = rust_arch + "-apple-ios-macabi",
            parents = ["@apple_support//platforms:darwin_" + apple_arch],
            constraint_values = [":catalyst_abi"],
            flags = [
                # Replace the macOS -target emitted by the inherited toolchain.
                "--features=-default_required_flags",
                "--@apple_support//toolchain:extra_enabled_features=//:catalyst_features",
            ],
        )

    _catalyst_target(
        name = "catalyst_target",
        target_compatible_with = ["@platforms//os:macos", ":catalyst_abi"],
    )
    cc_args(
        name = "catalyst_target_flags",
        actions = [
            "@rules_cc//cc/toolchains/actions:compile_actions",
            "@rules_cc//cc/toolchains/actions:link_actions",
        ],
        args = ["-no-canonical-prefixes", "-target", "{CATALYST_TARGET}"],
        toolchains = [":catalyst_target"],
    )
    cc_args(
        name = "catalyst_headers",
        actions = ["@rules_cc//cc/toolchains/actions:compile_actions"],
        args = [
            "-isystem",
            "__BAZEL_XCODE_SDKROOT__/System/iOSSupport/usr/include",
            "-iframework",
            "__BAZEL_XCODE_SDKROOT__/System/iOSSupport/System/Library/Frameworks",
        ],
    )
    cc_args(
        name = "catalyst_libraries",
        actions = ["@rules_cc//cc/toolchains/actions:link_actions"],
        args = [
            "-L__BAZEL_XCODE_SDKROOT__/System/iOSSupport/usr/lib",
            "-F__BAZEL_XCODE_SDKROOT__/System/iOSSupport/System/Library/Frameworks",
            "-framework",
            "UIKit",
        ],
    )
    cc_feature(
        name = "catalyst_flags",
        feature_name = "cel_catalyst",
        args = [":catalyst_target_flags", ":catalyst_headers", ":catalyst_libraries"],
    )
    cc_feature_set(
        name = "catalyst_features",
        all_of = select({
            ":catalyst_abi_enabled": [":catalyst_flags"],
            "//conditions:default": [],
        }),
    )
