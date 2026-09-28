# musl 目标支持

cel-cxx 支持以下 `*-unknown-linux-musl` 目标：

| 目标 | 状态 | 说明 |
|------|------|------|
| `x86_64-unknown-linux-musl` | ✅ 已测试 | cross-rs 下完整测试套件通过 |
| `aarch64-unknown-linux-musl` | ✅ 已测试 | 测试二进制经 qemu-user 运行 |
| `i686-unknown-linux-musl` | ✅ 已测试 | |
| `armv7-unknown-linux-musleabi` | ✅ 已测试 | 软浮点工具链；测试二进制经 qemu-user 运行 |

产物为静态 PIE 二进制（Rust 的 musl 目标默认 `crt-static`），C++ 运行时
会被静态链接进最终可执行文件。

## 前置要求

Bazel 构建会用目标平台解析出的 C++ 编译器编译整个 libcel（cel-cpp、
Abseil、Protobuf、ANTLR 运行时），因此需要一个面向 musl 的 **C++** 工具链
——仅有 `musl-gcc` 是不够的：

- `$(prefix)g++`，支持 C++17 并带齐头文件；
- sysroot 中有静态的 `libstdc++.a`；
- **GCC 10 或更新版本**。cel-cpp 依赖的 Abseil-cpp（20260526.x）会在
  `absl/base/policy_checks.h` 中对 GCC 9 及更早版本直接 `#error`。

cxx bridge/FFI 编译需要 `CXX_<target>`（下划线形式）指向该编译器，最终
链接需要 `CARGO_TARGET_<TARGET>_LINKER`。

## 推荐：cross-rs + 仓库自带镜像

仓库根目录的 `Cross.toml` 已把全部四个 musl 目标接到 `docker/cross/` 下的
Dockerfile 所构建的镜像，无需手工配置：

```bash
cargo install cross --git https://github.com/cross-rs/cross
cross test --target x86_64-unknown-linux-musl
```

cross 在首次使用时构建各镜像（docker 层缓存使后续构建增量进行）。镜像基于
[rust-musl-cross](https://github.com/rust-cross/rust-musl-cross)，其工具链
（GCC 12.4，前缀 `<triple>-`，位于 `/usr/local/musl` 且已在 `PATH` 上）满足
上述全部要求。镜像在其基础上补齐：

- `CROSS_TOOLCHAIN_PREFIX` —— 为 libcel 构建激活
  [rules_cross_rs](https://github.com/xjasonli/rules_cross_rs) 的 Bazel cc
  工具链接线；
- `CARGO_TARGET_<TARGET>_LINKER` / `CC_<target>` / `CXX_<target>` —— 让
  cargo 与 cxx bridge 使用同一工具链；
- aarch64 与 armv7 的 `qemu-user` 作为 cargo runner（测试二进制是静态的，
  直接 `qemu-<arch>` 即可执行）；
- 仅 x86_64 需要修复 `/lib/ld-musl-x86_64.so.1`：该目标的 exec 配置会解析
  到 musl 工具链本身，其 protoc 以工具链的 musl libc 动态链接——镜像自带的
  `ld-musl` 符号链接指向 glibc 链接器脚本，因此要重新指向真正的 musl libc
  （详见 Dockerfile 内注释）。

不能改用 cross-rs 官方 musl 镜像：其工具链由 musl-cross-make 编译，固定在
GCC 9.2，会被 Abseil 拒绝。

## 备选：原生 musl 宿主（如 Alpine）

宿主默认工具链本身就面向 musl 时，直接 `cargo build` 即可，无需任何额外
配置：Bazel 自动配置的 C++ 工具链以 musl 为目标，并匹配 musl 平台（该平台
刻意只声明 cpu + os 约束）。此路径不涉及 cross-rs。

## 备选：宿主机安装 musl 交叉工具链

安装一个 GCC ≥ 10 的 musl 交叉工具链（例如
[musl.cc](https://musl.cc/) 的 GCC 11.2.1 构建，基于
[musl-cross-make](https://github.com/richfelker/musl-cross-make)），并暴露给构建：

```bash
curl -sSL https://musl.cc/x86_64-linux-musl-cross.tgz | tar xz -C ~/.cache/muslcc
T=~/.cache/muslcc/x86_64-linux-musl-cross

env PATH="$T/bin:$PATH" \
  CROSS_TOOLCHAIN_PREFIX=x86_64-linux-musl- \
  CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc \
  CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++ \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc \
  cargo build --target x86_64-unknown-linux-musl --example basic
```

`CROSS_TOOLCHAIN_PREFIX` 会激活
[rules_cross_rs](https://github.com/xjasonli/rules_cross_rs) 的 Bazel 工具链
接线：它在 `PATH` 上发现 `$(prefix)gcc`、`$(prefix)g++`、`$(prefix)ar`、
`$(prefix)ld` 并注册匹配的 cc 工具链。在 glibc 宿主上，这正是让 musl 工具链
在 Bazel 平台解析中胜过本地 glibc 工具链的关键。

## 工作原理

- `cel-build-utils` 接受 musl triple，并以
  `--platforms=//:x86_64-unknown-linux-musl`（等）运行 Bazel 构建。musl
  平台声明的约束与其 gnu 对应平台相同——`[cpu, os:linux]`，不含 libc 专属
  约束——因此原生 musl 宿主无需 cross-rs 即可解析出自动配置的工具链。
- 在 cross-rs 下，`rules_cross_rs` 依据 `CROSS_TOOLCHAIN_PREFIX` 的工具
  生成 cc 工具链。其注册必须来自**根模块**
  （`crates/cel-build-utils/cel/MODULE.bazel`）：Bzlmod 中根模块注册的
  工具链优先级高于自动配置的本地工具链——当目标与宿主同架构（如
  x86_64 → x86_64-musl，两个工具链匹配相同的平台约束）时，正是这一点让
  musl 工具链胜出。不在 cross-rs 环境时，生成的工具链是一个永远不会匹配
  的 stub，对原生构建无影响。
- `cel-cxx` 为 `cfg(target_env = "musl")` 启用 `link-cplusplus` 的
  `libstdc++` 特性：rustc 在 gnu 目标上会自动链接 libstdc++，但在 musl
  上没有默认的 C++ 运行时。
- 32 位 ARM 需要链接 `libatomic`（musl 上为静态），供 abseil/protobuf 的
  64 位原子操作使用。rustc 通过编译器驱动的 `-print-file-name` 解析该归档
  文件，因为交叉工具链把它放在 rustc 自身搜索路径之外。
- 32 位 ARM 的预编译 libstdc++ 中包含按 pre-v6 核心编译的对象，其
  locale/字符串引用计数会调用 `__sync_*` 运行时辅助函数。musl 工具链
  把它们放在 libgcc.a 里，但 rustc 以 `-nodefaultlibs` 链接、其
  compiler_builtins 只带该家族的 `__sync_fetch_and_*` 一半——因此 bridge
  在每个链接都会拉取的 runtime 对象里，以 armv7 指令补齐缺失的
  `__sync_*_and_fetch_*` 与 `__sync_synchronize`
  （`crates/cel-cxx-ffi/src/arm_sync_shim.inc`）。

## armv7 说明

`armv7-unknown-linux-musleabi`（软浮点）需要一个软浮点的 musl C++ 工具链
且 GCC ≥ 10。rust-musl-cross 恰好提供一个（GCC 12.4，即自带镜像的基镜像），
这个目标因此变得可测。不要转而使用唾手可得的 armv7 硬浮点
（`musleabihf`）工具链：该 Rust 目标跨 FFI 边界时浮点参数走通用寄存器，
硬浮点编译器会静默破坏浮点数据。

## 注意事项

- 链接内存消耗大：每个测试/示例二进制都要静态链接完整 C++ 运行时，而
  `cargo test` 会并发执行多个链接。如果链接器被 OOM kill
  （`ld terminated with signal 9`），请降低并发数（`cargo test -j 4`），
  必要时跳过 doc 测试——rustdoc 的 doctest 编译不遵循 cargo 的 `-j`。
- cross-rs 容器中的 Bazel 动作缓存不会被持久化
  （`--output_user_root=/tmp/bazel` 位于临时容器内），因此每次 `cross`
  调用都会从头重编 libcel。若复用 `target/` 下的路径，cross 构建即可增量。
