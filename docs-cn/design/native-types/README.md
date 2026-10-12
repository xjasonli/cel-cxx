# Native Types 扩展与 Prost 设计

本目录集中维护 cel-cxx 的 Native Types 扩展、Prost 接入与通用值转换设计。**[核心设计](design.md) 是唯一规范性文档**；专题补充解释依据与取舍，验证记录区分已验证事实和待实现能力。

状态：设计定稿候选，新增 API 尚未实现。最近整理：2026-10-11。

## 设计概要

Native Types 是 cel-cxx 的宿主类型扩展，通过 `EnvBuilder::with_native::<T>()` 注册根类型并安装所需支持，不另设启用开关。普通 Rust struct/enum 与 prost 普通 message 共用 `BasicStructType` 和 Rust `CustomStructValue` bridge，但采用不同的语义 profile：

- **Rust profile**：derive 覆盖 named/tuple/newtype/unit struct 与非空 enum；实际字段始终存在，variant 选择字段按分支 presence，Option payload 保留 CEL optional，整体比较使用宿主 PartialEq。
- **Protobuf profile**：prost-build 扩展默认生成；字段、presence/default、enum、oneof 与 WKT 遵守 protobuf 到 CEL 的规则，相等优先复用兼容的派生 PartialEq。
- **共同机制**：Env 显式注册 Native 根类型并展开字段依赖、冻结的类型 registry、Box<dyn Struct> 拥有型对象与受控消费型恢复、字段槽位 builder、统一解码与安全函数边界。

**引擎接入点是组合 type provider：普通 struct 先 Native，类型名未命中再查 C++ descriptor pool。** Native 命中后的字段缺失、空字段集与只读 builder 不回退；同名 metadata 可共存，但同名 Parsed 值不可准入。pool 消息的实际 message 依赖闭包若指向被 Native 覆盖的类型，则拒绝整个 pool 根，不能只重写 checker 字段类型。Opaque 保留独立类型头与参数；protobuf enum 使用显式表和 pool 查询，字段/常量统一为 int，planner 缓存未命中时也查询 provider。checker/runtime 共享同一组合实例，保留现有 RuntimeEnv。checked AST 在目标 Env 清除旧元数据并重新检查。接口、冲突规则、构建顺序与所有权见 [§9.2](design.md#92-provider-安装)；这些接口仍待实现。

TypedStruct 要求宿主 Clone + PartialEq，宿主直接作为 T 参数与 typed evaluate/owned decode 结果。FromValue/FromOwnedValue 与 getter 共用显式 ValueContext，递归不重置预算；has/zero 等无 arena 的纯查询不计转换预算。借用解码复制，消费型解码移出；拥有型字段 getter 可复制子对象，深拷贝链的逐层选择可达 O(n²)。完整 root 的零次 Clone 与重复输出的最小副本数必须由真实 activation/FFI/输出路径验证，端到端边界见核心 §5.3–5.4、§18.7。

**StructValue 统一为 `Box<dyn Struct>`，接口采用与 `Opaque / TypedOpaque` 一致的 `Struct / TypedStruct` 分层。** Struct 提供动态 definition/get/has/bool equality/zero 与 DynClone；TypedStruct 提供静态定义、具体操作及 Clone/PartialEq 约束，库自动为它实现 Struct。`#[derive(Struct)]` 生成 TypedStruct，`with_native::<T>()` 要求 T: TypedStruct 并支持无实例注册，Box 直接拥有 T。纯 schema/view 可以只实现 Struct。泛型恢复放在 impl dyn Struct，TypedValue 仍只描述类型。C++ CustomStruct 是通用接入接口；cel-cpp legacy API/值及任意 foreign CustomStruct 仍明确拒绝，首版不交付 v4 占位实现。

newtype 保留具名对象与 value 字段，多字段 tuple struct 使用 _0/_1。普通 enum 使用同一具名类型、kind 标签及每个 variant 的选择字段：支持 `e.A`、`e.B.x`、`e.C.x`，用 `has(e.C)` 判断当前分支；无 payload 的 Unit 等也有 null 标记字段，支持 `has(e.Unit)`。未选中直接读取为 CEL Error，`e.?C` 则返回 optional。record/多字段 tuple variant 使用拥有真实 enum 快照的合成字段视图。手写 definition 或显式 scalar adapter 仍可选择；union、空 enum、packed、非静态借用和未适配字段等明确限制集中在核心设计 §4.9。

**固定决策：variant 是否选中使用 presence；payload 的 Option 表示 optional 内容。** 选中 Maybe(None) 时 has(e.Maybe)=true，e.Maybe 为 none，e.?Maybe 为 some(none)；其他分支的 e.?Maybe 才是 none。optional chaining、条件构造、unit/null 和 protobuf 的组合边界统一见核心设计 §4.10，不提供按 Env 切换这套语义的选项。

裸 `()` 映射 null；unit struct、普通零字段 message 与 `google.protobuf.Empty` 保留具名对象。enum 的 null 标记只属于无 payload 选择字段，完整 enum 仍有自己的类型。prost 将 Empty 存为 () 不改变其 protobuf/CEL 身份；详细边界见核心设计 §4.11。

prost-build 扩展自动生成 protobuf enum 的 int 转换/具名常量，以及 oneof 的父成员访问和构造；数字 enum 字段保存 raw i32，oneof 保存 Option<带 payload 的 enum>。这些默认规则不采用普通 Rust enum 映射；本地 oneof enum 可另外显式选择独立的 Rust 对象视图。protobuf enum 与转换为 CEL 内建值的 WKT 不强行包装成 struct。官方 Rust Protobuf v4 保留未来接入边界；首版不实现 Any codec、完整 partial message 或 unknown-field 保留。

## 文件与阅读顺序

| 文档 | 定位 | 适合回答的问题 |
|---|---|---|
| [design.md](design.md) | 核心规范 | 支持什么、接口是什么、各边界怎样处理、如何验收 |
| [plan-native-types.md](plan-native-types.md) | 从属于核心设计的实施计划 | Rust/C++ 如何按依赖落地、各阶段如何验证、如何记录问题与执行状态 |
| [supplements/native-interop.md](supplements/native-interop.md) | 跨语言实现与桥接依据 | Go/Java/C++ 实际怎样做，哪些机制可复用，为什么不用 map/bytes/opaque 替代 |
| [supplements/option-semantics.md](supplements/option-semantics.md) | Option 与转换的取舍说明 | 为什么统一 std Option，为什么不用 std TryFrom 作为全部解码入口 |
| [supplements/validation.md](supplements/validation.md) | 验证记录 | 哪些结论有源码或实际探针支持，哪些仍需完整 bridge 验证 |
| [supplements/protobuf-v4-status.md](supplements/protobuf-v4-status.md) | 未来后端的定时点调研 | 截至 2026-10-08，官方 Rust Protobuf 发布版/main 的能力和接入门槛 |

先读核心设计的目标、固定决策与两类 profile；需要确认某项取舍时再进入对应补充。

## 核心设计速查

| 关注点 | 核心章节 |
|---|---|
| 范围、两类 profile 与配置位置 | [§1–3](design.md#1-目标与交付范围) |
| Native Types 扩展、通用核心与 Cargo feature 的分工 | [§3.3](design.md#33-native-types-扩展的边界) |
| struct/enum 准入、类型身份与 Env 注册 | [§4](design.md#4-类型定义身份与-env-注册) |
| newtype/tuple/unit struct、全部 variant 形态与不支持项 | [§4.7–4.9](design.md#47-struct-的逐形态映射) |
| variant presence 与 optional 的固定决策、状态和路径矩阵 | [§4.10](design.md#410-presence-与-optional-的固定决策) |
| unit、零字段 struct、enum 标记与 google.protobuf.Empty | [§4.11](design.md#411-unitnull-与零字段对象) |
| Struct trait、所有权、downcast 与物化 | [§5](design.md#5-rust-值表示与所有权) |
| Struct/TypedStruct 分层、blanket 转发与 Box<dyn Struct> | [§5.1](design.md#51-公有形态) |
| enum 的拥有型 child 与合成 record view | [§5.6](design.md#56-enum-真实-payload-与合成-variant-record) |
| Native/prost/future Message 的统一接口与后端边界 | [§5.7](design.md#57-统一接口与后端边界) |
| Option、Box/Arc、解码与函数生命周期 | [§6–7](design.md#6-optionboxarc-与统一解码) |
| 普通字段、构造、checker/runtime 与错误 | [§8–10](design.md#8-普通-native-字段与构造) |
| Native 优先/pool 回退、Opaque/enum、组合 provider 与生命周期 | [§9.2](design.md#92-provider-安装) |
| 独占 builder、CEL Error 分支与 arena 边界 | [§9.4](design.md#94-builder-ffi) |
| 普通 Native struct 中的 enum 字段及混合 profile | [§8.4](design.md#84-普通-native-struct-中的-enum-字段) |
| enum 分支与 payload 构造校验 | [§8.5](design.md#85-enum-与合成-variant-record-的构造) |
| Prost、prost-types、enum、WKT 与 required | [§11](design.md#11-prost-生成式-protobuf-profile) |
| PartialEq、protobuf 相等与资源边界 | [§12](design.md#12-相等zero展示与资源边界) |
| API 示例 | [§14](design.md#14-公共-api-场景) |
| e.A / e.B.x / e.C.x 的完整例子 | [§14.6](design.md#146-rust-enum-的直接-payload-访问) |
| 模块责任、不变量与验收要求 | [§15–18](design.md#15-crate模块与现有代码迁移) |

## 文档维护规则

- 接口、支持范围或语义变化先修改核心设计，并同步其示例与验收矩阵。
- 补充文档保存理由、实现对照和反例，不另维护一份相互竞争的 API 或实施要求。
- 验证记录注明版本、输入、结果及验证层级；小型探针通过不等于 Native bridge 已实现。
- v4 报告中的“当前”限于所记录的调研时点，升级版本时重新核对，不改写旧结论为持续有效的承诺。
- 中英文版本分别维护，采用对应的目录结构；内部文档链接只指向同一语言目录，不以跨语言引用替代缺少的译文。
- 仓库内链接使用相对路径；外部源码优先固定版本/提交，不依赖作者机器目录或临时文件。

[返回中文文档](../../README.md)
