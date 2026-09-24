# musl 目标支持

cel-cxx 支持以下 `*-unknown-linux-musl` 目标：

| 目标 | 状态 | 说明 |
|------|------|------|
| `x86_64-unknown-linux-musl` | ✅ 已测试 | 完整测试套件通过 |
| `aarch64-unknown-linux-musl` | ✅ 已测试 | |
| `i686-unknown-linux-musl` | 🟡 应该可用 | 机制相同，未在 CI 中验证 |
| `armv7-unknown-linux-musleabi` | 🟡 工具链缺口 | 没有 GCC ≥ 10 的软浮点预编译工具链；见 [armv7 说明](#armv7-说明) |

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

## 工具链方案

### 方案一：原生 musl 宿主（如 Alpine）

宿主默认工具链本身就面向 musl 时，直接 `cargo build` 即可，无需任何额外
配置：Bazel 自动配置的 C++ 工具链以 musl 为目标，并匹配 musl 平台（该平台
刻意只声明 cpu + os 约束）。此路径不涉及 cross-rs。

### 方案二：宿主机安装 musl 交叉工具链

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

### 方案三：cross-rs + 自定义镜像

**cross-rs 官方 musl 镜像无法构建 cel-cxx**：其工具链由 musl-cross-make
编译，固定在 GCC 9.2，会被 Abseil 拒绝（`#error "This package requires
GCC 10 or higher."`）。

在 cross 镜像之上叠加一个更新的工具链，使其在 `PATH` 上遮蔽 GCC 9.2，
并让 musl 动态加载器能找到它的运行时库（官方 sysroot 在 `libstdc++.so` 的
位置放的是链接器脚本，动态加载器无法使用）：

```dockerfile
# Dockerfile.cross-musl-x86_64
FROM ghcr.io/cross-rs/x86_64-unknown-linux-musl:main
COPY x86_64-linux-musl-cross /opt/musl-gcc11/x86_64-linux-musl-cross
ENV PATH=/opt/musl-gcc11/x86_64-linux-musl-cross/bin:$PATH
RUN sed -i '1i /opt/musl-gcc11/x86_64-linux-musl-cross/x86_64-linux-musl/lib' \
    /etc/ld-musl-x86_64.path
```

```bash
curl -sSL https://musl.cc/x86_64-linux-musl-cross.tgz | tar xz
docker build -t cel-cxx-musl:x86_64 -f Dockerfile.cross-musl-x86_64 .
```

然后在项目根目录放置 `Cross.toml`，把 cross 指向该镜像：

```toml
[target.x86_64-unknown-linux-musl]
image = "cel-cxx-musl:x86_64"
```

之后正常构建即可：

```bash
cross build --target x86_64-unknown-linux-musl --example basic
```

各架构同理（musl.cc 的 `aarch64-linux-musl-cross.tgz`、
`i686-linux-musl-cross.tgz`；相应调整 `FROM` 镜像和
`ld-musl-<arch>.path` 文件名）。

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

## armv7 说明

`armv7-unknown-linux-musleabi`（软浮点）需要一个软浮点的 musl C++ 工具链
且 GCC ≥ 10。目前没有公开的预编译产物——musl.cc 和 Bootlin 只提供 armv7
硬浮点（`musleabihf`）构建——因此可选方案：

- 自行用 [musl-cross-make](https://github.com/richfelker/musl-cross-make)
  以更新的 `GCC_VER` 和 `--with-float=soft` 构建，或
- 改用 `armv7-unknown-linux-musleabihf`：musl.cc 工具链和 cross-rs 镜像
  都存在（向 `crates/cel-build-utils/src/lib.rs` 的 `SUPPORTED_TARGETS`
  和 `crates/cel-build-utils/cel/BUILD.bazel` 的平台各加一条即可）。

## 注意事项

- 链接内存消耗大：每个测试/示例二进制都要静态链接完整 C++ 运行时，而
  `cargo test` 会并发执行多个链接。如果链接器被 OOM kill
  （`ld terminated with signal 9`），请降低并发数（`cargo test -j 4`），
  必要时跳过 doc 测试——rustdoc 的 doctest 编译不遵循 cargo 的 `-j`。
- cross-rs 容器中的 Bazel 动作缓存不会被持久化
  （`--output_user_root=/tmp/bazel` 位于临时容器内），因此每次 `cross`
  调用都会从头重编 libcel。若复用 `target/` 下的路径，cross 构建即可增量。
