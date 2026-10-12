# 官方 Rust Protobuf 能力快照与接入依据

定位：[核心设计 §13](../design.md#13-future-官方-rust-protobuf-v4) 的未来后端补充。调研时点：**2026-10-08**；本文的“当前”“发布版”“main”均指这一时点，不是持续更新的支持承诺。文档于 2026-10-10 归入设计目录，未重新刷新外部版本。

本文区分发布能力、未发布进展和早期方案中需要修正的判断，并在各节列出原判断、修正结论与源码依据。当前产品边界是 prost → Native adapter、官方 Rust Protobuf → future message adapter，公有值统一为 Box<dyn Struct>；接口遵循 [核心 §5.1](../design.md#51-公有形态)，不改变本文记录的外部版本能力。

[返回设计目录](../README.md) · [验证层级](validation.md#基线与证据层级)

## 调研基线与当前版本

早期 protobuf 方案由提交 `72c102b` 于 2026-03-04 加入仓库，引用官方 `protobuf 4.34.0-release`。它把 prost 设为序列化通路，以 protobuf-rust v3 提供反射，并等待官方 v4 的 C++ kernel；本文分别核对这些判断。版本比较区分三份源码：

| 基线 | 版本或提交 | 用途 |
|---|---|---|
| 旧文对应的发布版 | protobuf 4.34.0-release / upstream v34.0 | 判断某能力当时是否已经存在 |
| 当前发布版 | upstream v36.2，2026-09-17 发布 | 判断用户现在能从发布包获得什么 |
| 当前 main | cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2，2026-10-08 | 判断尚未发布的进展 |

已核对 GitHub release、crates.io 元数据，以及下载后的真实发布包源码。当前包名和版本为：

| crate | 当前发布版本 | 角色 |
|---|---|---|
| google-protobuf | 0.36.2-release | 官方 Rust runtime 的新开发主线 |
| protobuf | 4.36.2-release | 重导出 google-protobuf 的兼容包 |
| google-protobuf-codegen | 0.36.2-release | 新 codegen 包 |
| protobuf-codegen | 4.36.2-release | 对应的兼容包 |

所以本文的“v4”指官方 Rust 实现这条技术路线，不意味着它现在仍以 `protobuf 4.x` 作为主要开发包名。当前 README 仍标明 beta；尽管 release notes 曾列出去掉 `-release` 后缀的改动，实际发布版本仍带该后缀。版本表以 crates.io 实际包为准。[v36.2 发布记录](https://github.com/protocolbuffers/protobuf/releases/tag/v36.2)、[兼容包说明](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/release_crates/protobuf/README.md)、[google-protobuf 包元数据](https://crates.io/api/v1/crates/google-protobuf)、[protobuf 包元数据](https://crates.io/api/v1/crates/protobuf)

本地 cel-cpp 依赖仍固定在 protobuf 35.1。这与当前发布的 36.2 是不同基线；未来采用共享 C++ kernel 时需要统一生成器与 runtime，不能因同属官方实现而假设 ABI 兼容。[本地固定版本](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/MODULE.bazel#L36)、[官方 Rust 版本要求](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/release_crates/google_protobuf/README.md)

## 已发布的新增能力

| 能力 | 从 4.34 至当前的变化 | 对 cel-cxx 的意义 |
|---|---|---|
| presence getter | v35 将 `_opt()` 从库自己的 Optional 改为 `std::Option` | Option 表达字段有无，不应据此映射为 CEL optional |
| 泛型 message 操作 | v35 改进 Message bounds，引入 Singular，并整理 Map traits | 更容易写通用 adapter；不等于可以按名称反射任意字段 |
| view 使用 | v35 让 `&T` 在 T 实现 AsView 时也实现 AsView | 借用式 API 更自然；仍需区分 owned M 与 MessageView |
| repeated message 修改 | v35 有 push_default；v36 增加可变 message view 的遍历 | builder 的基础操作更完整；CEL 中仍应保持已发布值只读 |
| 包结构 | v36 系列转向 google-protobuf / google-protobuf-codegen | 新接入应面向新包，不能把旧 crate 名写死在公有类型中 |
| 生成代码路径 | v36.2 公开每个 proto 文件的模块，修正 canonical type path 和跨文件名称冲突 | 外部生成适配代码更容易可靠引用具体类型 |

这些改动已经在相应发布记录与发布源码中核对，不是从 main 推测未来版本。[v35.0 Rust 改动](https://github.com/protocolbuffers/protobuf/releases/tag/v35.0)、[v36.0 Rust 改动](https://github.com/protocolbuffers/protobuf/releases/tag/v36.0)、[v36.2 生成代码改动](https://github.com/protocolbuffers/protobuf/releases/tag/v36.2)

其中 `_opt()` 的变化说明：protobuf 的显式 presence 属于 schema；CEL `optional_type<T>` 是另一种值。prost 的 `Option<T>` 字段与 v4 的 `_opt()` 都不能仅因使用 Rust Option 就改成 CEL optional；protobuf 的隐式 presence、oneof、wrapper 等继续由 protobuf profile 决定。2026-10-09 的通用 Rust API 设计则建议变量、结果、函数、集合与普通 native 字段中的 Option 统一映射 CEL optional；prost 使用 native 后端，但字段仍属于 protobuf profile。六位置判断见 [Option 与转换说明](option-semantics.md)。这不改变本报告截至 10 月 8 日的 v4 源码调研结论。

这里必须区分存储与访问结果：v4 生成 message 保存 OwnedMessageInner，不像 prost 逐字段保存 Option<T>；std::Option 出现在当前 `_opt()` getter 的返回值中。因此“message 字段是 std::Option 映射的例外”并不准确：需要特殊适配的是 prost-native 的生成字段；v4-message 本来就按底层 schema/message 访问，不经过普通 Rust Option 成员推导。若独立暴露 `_opt()` 的返回值，它仍使用普通 Option 值映射。[v36.2 message 生成](https://github.com/protocolbuffers/protobuf/blob/v36.2/src/google/protobuf/compiler/rust/message.cc#L534)、[v36.2 getter 生成](https://github.com/protocolbuffers/protobuf/blob/v36.2/src/google/protobuf/compiler/rust/accessors/with_presence.cc#L90)

## main 新增的反射基础

v36.2 发布之后，官方确实开始推进 reflection，旧文“没有 reflection”不能直接用于描述当前 main。

| 日期 | main 新增内容 | 当前边界 |
|---|---|---|
| 2026-09-23 | WithReflection marker、CPP reflection 支持与 TextFormat 输出 | 标识能够反射的 message；尚非通用字段读写 API |
| 2026-09-24 | CPP TextFormat 解析 | 属于 CPP kernel runtime 内的能力 |
| 2026-10-05 | upb reflection、进程全局 DefPool、MessageDef 缓存和 TextFormat 输出 | 为 generated message 加载 descriptor；MessageDef wrapper 尚无字段枚举/读写 facade |

对应提交：[CPP reflection](https://github.com/protocolbuffers/protobuf/commit/117554dea4f17e6e4e0c082859182b4d60ec2851)、[CPP 文本解析](https://github.com/protocolbuffers/protobuf/commit/511b1c34f0f5fa6406084c8afd6da841dd785183)、[upb reflection](https://github.com/protocolbuffers/protobuf/commit/c817c51f46b9657faea46c3034c65f2dba462cd5)。以上均晚于 2026-09-17 发布的 v36.2。

截至所检查的 main：

- 根 API 已重导出 WithReflection，但 marker 本身没有按名称查询字段的方法。
- CPP runtime 有 `print_to_text_format` / `parse_from_text_format`；upb runtime 有文本输出。这些函数尚未由根 API 重导出，不能按稳定应用接口使用。
- upb 的 MessageDef 目前主要封装 raw descriptor pointer，未提供通用 FieldDescriptor、按字段名称/编号读写和 presence 查询 facade。
- 未发现官方 Rust `DynamicMessage` API；底层 C++/upb 自己已有的动态与反射能力不能算作 Rust 公有 API。
- upb reflection codegen 还排除了当前文件或 imports 含 extensions 的情形，不能把 WithReflection 视为所有 message 的统一能力。[生成限制](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/src/google/protobuf/compiler/rust/message.cc#L294)

因此，准确判断是：**main 已有反射基础建设，发布版尚无足以直接写通用 CEL 字段 adapter 的 Rust 反射接口。** 这是一项实质进展，但尚不能据此宣布 v4 已具备 v3 MessageFull 那套应用能力。[main 公有重导出](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/rust/shared.rs)、[WithReflection 定义](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/rust/codegen_traits.rs)、[upb MessageDef](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/rust/upb/reflection.rs)、[CPP reflection](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/rust/cpp_kernel/reflection.rs)

## 旧文需要修正的判断

### upb 与 C++ 的链接限制

旧文把限制写成了 upb 与 protobuf-cpp 不能同处一个二进制，范围过大。

实际限制是：**同一个二进制不能同时包含官方 Rust Protobuf 的 upb kernel runtime 和 cpp kernel runtime。** Rust shared runtime 会定义 `__Disallow_Upb_And_Cpp_In_Same_Binary`；两种 Rust kernel runtime 各带一份，故意产生重复符号。该机制在 4.34 发布包已经存在，36.2 仍保留。[4.34 shared runtime](https://github.com/protocolbuffers/protobuf/blob/v34.0/rust/shared.rs)、[36.2 shared runtime](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/shared.rs)

这不能推导出“Rust upb 不能与 cel-cpp 使用的原生 C++ protobuf runtime 共存”。官方自己有同时使用原生 C++ full runtime 与 upb reflection 的测试。对象 ABI 仍不能互换；本次也未编译验证 cel-cxx 与最新 upb 的完整链接组合。[官方共存测试](https://github.com/protocolbuffers/protobuf/blob/v35.1/upb/reflection/reflection_test.cc)、[测试依赖](https://github.com/protocolbuffers/protobuf/blob/v35.1/upb/reflection/BUILD)

### CPP raw interop 并非最近新增

4.34 源码已提供 OwnedMessageInterop、MessageViewInterop、MessageMutInterop，以及 CPP full 的 MessageDescriptorInterop。它们允许借用底层指针、包装 borrowed view，或接管符合条件的独立 owned 消息。35/36 将实现位置整理到 kernel interop 模块。[4.34 interop 定义](https://github.com/protocolbuffers/protobuf/blob/v34.0/rust/codegen_traits.rs)、[36.2 CPP interop](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/cpp_kernel/interop.rs)

所以 CPP full 路径可以借助 C++ 已有 reflection 研究接入，未必需要等 Rust 通用反射 API。当前 stock crates.io 发布包仍构建 upb，不能仅打开某个 Cargo feature 就获得 CPP kernel；CPP 代码及 Bazel 构建通路已经存在，与开箱可用的 Cargo 集成是两个问题。[当前发布包构建](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/release_crates/google_protobuf/build.rs)、[官方 Rust kernels 说明](https://protobuf.dev/reference/rust/rust-design-decisions/)

CPP 还必须区分 full 与 lite。main 的 interop 文档明确 raw pointer 一般仅保证为 MessageLite，具有 WithReflection 的类型才保证为 full Message；旧版本也将 DescriptorInterop 限于非 lite。不能将任意 CPP kernel 指针当作 cel-cpp 所需的 `google::protobuf::Message*`。[main CPP 指针合同](https://github.com/protocolbuffers/protobuf/blob/cefb2a661dadbb96b50b7b92fe29a466ddd3a2e2/rust/cpp_kernel/interop.rs)

### CEL 构造结果不必总是序列化

旧文“所有 Rust protobuf 库的 C++ 返回路径只能序列化”不成立。CPP kernel 至少有两种无需 wire encode/decode 的路径：

1. 保留 C++ arena、factory、pool 等 owner，以 borrowed MessageView 暴露结果。
2. 从匹配的 view 执行 to_owned / CopyFrom，得到独立 owned Rust message；这里是深复制。

第二条路径的生成代码调用 C++ copy thunk。本地 protobuf 35.1 可以直接核对：[生成的 IntoProxied](https://github.com/protocolbuffers/protobuf/blob/v35.1/src/google/protobuf/compiler/rust/message.cc#L108)、[C++ copy 实现](https://github.com/protocolbuffers/protobuf/blob/v35.1/rust/cpp_kernel/message.cc#L40)。

两条路径都要求实际 C++ concrete message 与生成的 Rust M 匹配。cel-cpp 的 custom-pool 分支明确关闭 generated factory delegation，通常构造 DynamicMessage；同全名、同 descriptor schema 不能让它安全变成 generated Rust MView。typed 返回路径应使用实际 generated descriptor/factory，并验证 concrete type。arena 内对象也不能交给会 delete 的 owned take-ownership API。[cel-cpp factory 选择](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/internal/runtime_env.cc#L45)、[CPP ownership 合同](https://github.com/protocolbuffers/protobuf/blob/v36.2/rust/cpp_kernel/interop.rs)

按新设计，prost-native 将由 Rust builder 直接完成 CEL 对象构造，并通过 Rust CustomStruct bridge 返回 M，无需经过 C++ protobuf message 或 bytes。旧文的“prost 只能序列化”和“所有 CEL 构造结果统一成为 Bytes”都应被这次新设计替代。

## 对核心设计的影响

这些事实支持当前 [核心设计 §13](../design.md#13-future-官方-rust-protobuf-v4) 保留两条未来路径：匹配版本与具体类型的 CPP full message 通路，以及具备公有字段访问能力后的 upb CustomStruct callbacks。Native/Message 是具体 Struct 实现的来源，不再是公有 enum；CPP full/upb 的 owner 与通路在各自 adapter 内封装，不机械对应 C++ CustomStruct/ParsedMessage。prost 使用拥有实际 M 的 Native adapter 与 Protobuf profile。

v4 结果恢复需要保留 owner 的 typed view/handle；不普遍承诺可取得 Rust owned M 的 &M，也不把 raw pointer 自动当作安全的 full Message。完整生成/构建链、链接实例、descriptor/factory、线程与 ownership 验证均属于接入门槛。

本报告没有实现 v4 bridge，也未运行跨 kernel 集成测试。主设计中的 prost-native 不依赖 v4 完成，首版不提供空壳 v4 功能。
