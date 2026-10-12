# 设计验证记录

定位：记录 [核心设计](../design.md) 的证据和验证边界，不新增规范或实施任务。整理日期：2026-10-10；原始调研与探针运行于 2026-10-08 至 2026-10-10。

[返回设计目录](../README.md) · [正式验收矩阵](../design.md#18-验证与验收矩阵)

## 基线与证据层级

| 组件 | 基线 |
|---|---|
| cel-cxx | 初始调研 `609805b`；核心设计 `7ff30ee425d391643a0abd11a85a934e70324077`；期间相关类型/FFI 代码未变 |
| cel-cpp | xjasonli/cel-cpp `c84e883df5932dcc992dd3494a771c37e85ae1fe`，包含 PR #2115 |
| cel-go | `12ea74fdf5f4806087b979ee05d6e38ab35c151f` |
| cel-java | `798c0be3a7beef6816344e0323294067bc1d8b1c` |
| prost / prost-build / prost-types | 0.14.1 |
| C++ protobuf | 35.1 |
| Rust 复核工具链 | rustc 1.96.1；不代表已经验证项目 MSRV |

本文区分源码检查、语言/接口模型探针、真实引擎调用及完整 bridge 集成。**统一 Struct 的生产实现、Native move bridge、生成器和字段槽位 builder 尚未实现**；以下结果不能代替它们的正式测试。

原始小型探针曾保存在开发机临时目录，尚未作为可移植测试纳入仓库。下文保留输入、结果和探针文件名供追溯，不将临时路径作为文档依赖。

## Rust 类型与转换探针

2026-10-09 的集中复核：9 个正例编译并运行通过，5 个负例按预期出现 E0275/E0119/E0210/E0582。其他专项探针另列如下；不是把所有历史探针都计入这 14 个。

| 主题 | 验证的事实 | 代表性探针 |
|---|---|---|
| Option/Box/Arc 组合 | 单一 Option 实现、透明指针、嵌套 optional、严格拒绝 null 可共存；non-Clone Arc 根可保留身份 | `composition.rs` |
| std 包装与 CEL 解码 | `Option<Value>::try_from(v)` 包装整个 v；与本地严格解码不同；扩大 blanket impl 会重叠 | `std-families-pass.rs`、`std-box-arc-lift-pass.rs`、`std-try-option-all-fail.rs` |
| orphan/coherence | 通用 Box 标准反向转换受限制；调用方不能替外部 Arc 实现外部 trait；Native blanket 与透明容器可能重叠 | `orphan.rs`、`coherence.rs` |
| owned GAT | `for<'a> FromValue<Output<'a> = T>` 可表达真实 owned 输出；Box<&User> 不满足 Box<User> | `owned-gat-bound-pass.rs` |
| 借用回调 | 公开 sync/async lending 原型可保留输入 owner；普通单一 Fut 不等价于高阶借用 future | `async-public-lending-trait-pass.rs`、`cel-cxx-native-formal-contract-final.rs` |
| 动态 optional 布局 | `Option<Box<Value>>` 打断递归并区分 none / some(null) / some(none)；别名不产生独立 trait 身份 | `alias.rs`、`representation.rs` |
| 普通 Option 与字段模式 | 独立字段 trait 可以表达 presence 模型，不需要按类型名称猜测 Option | `field-storage-with-option-value.rs` |
| 非 Clone owner | Parent/Child 不实现 Clone；子投影保留真实借用与 owner，不能制造 Arc<Child> | `native-owner-projection.rs` |
| 递归注册 | 先登记身份再解析依赖可终止注册；不能把递归字段完整能力再次放入 Self 的前置 bound | 递归注册/capability 正负例 |

字段 presence 模型探针只证明一种替代方案可表达；最终普通 Rust profile 保留 Option 值，不采用剥离最外层 Option 的规则。早期 owner 探针证明安全借用与保活；最终设计改为 Box<dyn Struct> 和 Clone 宿主，历史 Arc/非 Clone 投影探针不再代表首版默认合同。

## Rust struct/enum 形态与 VariantView

2026-10-10 使用 rustc 1.96.1、edition 2021，先运行 `shapes-and-views.rs`，随后用 `shapes-presence-and-views.rs` 补充直接分支访问的模型。两个正例均编译并运行通过；4 个独立负例按预期拒绝。

| 输入/模型操作 | 实际结果 | 证明边界 |
|---|---|---|
| named/single named/newtype/tuple/unit/empty record/empty tuple struct；类型与 const 泛型 | 使用各自 Rust 初始化语法编译运行 | 语言形态可表达，不代表 Struct derive 已生成这些实现 |
| unit、空 tuple/record、单 payload、named/multi tuple、Option/unit payload 与浮点 variant；整数 repr 下显式 discriminant | 10 类 enum 值与相应 match 编译运行 | Rust 语法与安全访问；CEL schema/命名规则仍是本库设计 |
| 无 Clone 的 E::B(Named) 子投影 | 丢弃原 owner 变量后，真实 Named 字段仍可访问 | Arc owner + 借用可行，未覆盖真实 C++ arena |
| E::C{x,y} 的 CView 保存 Arc<E> | 通过 match 读取 x/y；拒绝错误 case；无需另一份 owned payload record | 不依赖伪造 &E::C 或 Clone；未验证通用扁平 selector |
| C 分支 has/get/optional get 模型 | 选中时直接读 x；未选中时 has=false、get 报错、安全选择 None | 设计的条件 presence 可表达，尚未运行 CEL `e.C.x` |
| Maybe(None) 与另一分支 | 前者 has=true/get=Ok(None)/安全选择 Some(None)，后者 has=false/get 错误/安全选择 None | 保留 payload Option 与选择 optional 两层 |
| C record 构造与显式外层 enum 恢复 | 构造真实 E::C 后返回 view；匹配 tag 可复用同一 Arc，错误 tag 拒绝 | 只验证定向复用 owner；不是完整字段槽位 builder |
| 动态 Variant view 公开恢复为 E | 模型明确拒绝；普通 E holder 可返回 &E | 验证必须区分 view 身份，未覆盖运行时 definition key/FFI |
| 用 `E::C` 作 Rust 类型 | 编译拒绝 E0573 | variant 不是独立 payload 类型 |
| 对 repr(packed) 字段创建普通引用 | 编译拒绝 E0793 | 自动 getter 不能无条件生成字段引用 |
| 安全代码直接借用 union 字段 | 编译拒绝 E0133 | 不能自动猜测活动字段并安全读取 |
| 把局部借用宿主传给要求 'static 的 owner | 编译拒绝 E0597 | 不能靠 wrapper 延长借用 |

负例文件为 `variant-is-not-a-type.rs`、`packed-reference.rs`、`union-reference.rs`、`borrowed-not-static.rs`。这些是历史语言/owner 模型，不是对 cel-cxx 新 API 的测试，也没有验证 MSRV。最终拥有型 VariantView 改为持有 E 的快照，旧 Arc/非 Clone 实验不能作为新 Clone/move 路径的验证。真实 derive 的所有形态、跨 crate 属性与 bounds、合成类型注册、has/get 的引擎调用、错误传播、构造和生命周期仍须按 [核心验收矩阵](../design.md#18-验证与验收矩阵) 实现并验证。

## CEL presence/optional 组合验证

2026-10-10 的 `enum_presence_end_to_end.cc` 使用本文基线的 cel-cpp 实际 parser/checker/runtime。小型 C++ TypeIntrospector 提供具名 E/E.C 的 BasicStruct schema，CustomStructValue 模型提供固定字段表、分支 presence 和 getter；输入包含 C、Maybe(None)、Maybe(Some(8))、Deep(Some(None))、unit/null 标记，以及 0/false/空字符串/空 list payload。

启用标准库与 optional library、qualified type identifiers，并关闭 legacy null assignment。`max_recursion_depth=0` 与 `128` 两种规划配置各通过 **170 项检查**：164 次真实表达式求值及 6 项 parse/check 拒绝，共 340 项。负例包括 has(e)、has(e.?C)、未知字段选择/has/optional 选择及对普通 record 调用 hasValue。

| 检查内容 | 实际结果 |
|---|---|
| 已知 variant 的 has、kind 与直接 payload 选择 | 与模型当前分支一致；0/false/空字符串/空 list/null 仍 present |
| 非 C 分支上的 e.C.x、has(e.C.x)、e.C.?x | 均得到 CEL Error；末级 has/? 不保护中间普通选择 |
| e.?C.x 与 e.?C.?x | C 选中时取得 x；未选中时返回 none |
| has(e.?C.x) | 非 C 时 false；通过 optional receiver 查询最后字段 |
| Maybe(None) 与 Maybe(Some(8)) | has=true；e.?Maybe 保留外层选择 optional 和内层真实 Option |
| Deep(Some(None)) | e.?Deep 保留三层状态，不 flatten |
| unit/null 标记 | has(e.Unit)=true，e.Unit=null，e.?Unit 有内容且内容为 null |
| o: optional<E> 为 Some(E)/None | o.C、o.?C、has(o.C) 与 o.C.x 遵守 optional receiver 的链式规则 |
| 普通 record 的 opt=None | has(e.C.opt)=true；e.?C.opt 为 some(none) |
| map 的普通 none 值与条件初始化 | 普通条目保留 optional.none；条件 none 省略条目；条件 some(none) 保留条目和值 |

这是实际 C++ 语言与 CustomStruct 字段接入的组合验证，**不是 Rust Native bridge 或生成器的端到端验证**。模型没有实现 enum constructor、宿主 owner/downcast、Rust PartialEq、完整 ForEachField 或错误分类。构造器的 tag/slot/default 规则仍需实现验收；条件条目只解包一次的结论另核对了 create_struct_step 的真实代码。选中字段的 getter 失败/Unknown 与 optional 包装仍须落实核心 §10.2 已列出的 guard 修补，不能从本次正常 payload 路径通过推导这些修补已完成。

## CEL 零字段对象与 Empty 验证

2026-10-10 的 `unit_empty_end_to_end.cc` 使用相同 cel-cpp 基线、标准库/optional library 与 null assignment 配置。输入包括两个不同全名的零字段 BasicStruct/CustomStruct 模型、动态 descriptor 定义的普通零字段 message、google.protobuf.Empty，以及包含这两种 message 的 singular/oneof 字段。

`max_recursion_depth=0` 与 `128` 各通过 **44 项检查**：35 次真实求值（其中 2 项预期 CEL Error）、2 项 Native 空 literal checker 验证、7 项静态拒绝，共 88 项。

| 检查内容 | 实际结果与边界 |
|---|---|
| 零字段 CustomStruct 输入的 type | 相同全名一致，不同全名不同；与 null/map 的类型不同，dyn(s) 不等于 null |
| example.S{} / example.T{} | checker 接受零条目 literal；未验证 Native runtime builder，既有 provider 接入缺口仍待实现 |
| 普通 message M{} 与 google.protobuf.Empty{} | 真实 runtime 构造成功，各自保留类型；不等于 null，同类型空 message 比较相等 |
| singular message 的 unset / set-empty | has 分别 false/true；直接读取都得到对应具名空 message，且比较相等；optional select 分别无内容/有内容 |
| oneof 的普通空 message / Empty 成员 | 设置为空仍 present；未选中时 absent，普通读取仍按 protobuf default 返回对应空 message |
| 空类型的未知字段 get/has/literal | typed 输入在 checker 拒绝；动态 CustomStruct 的 get/has 得到 CEL Error |
| optional 包装 | some(空 CustomStruct) 与 some(null) 都有内容；不能与 none 合并 |

这是 cel-cpp 类型/值协议和 protobuf 实际表达式验证，不是 Rust unit struct derive、prost Empty adapter 或 Rust owner/downcast 的集成验收。Native 模型的比较与 ForEachField 未在此探针中验证；“不同类型不合并身份”也不能替代注册定义和 Rust TypeId 检查。初次探针中 `type(s) != type({})` 因 checker 的参数化 map type 比较规则拒绝，改用 dyn(type(...)) 比较实际类型值后通过；没有把该静态限制解释为零字段对象不受支持。

## Go native presence 与 optional

在上述 cel-go 基线上运行：

```text
go test ./ext -run '^TestNativeTypes(WithOptional|WithCELTypedFields)?$' -count=1
结果：通过
```

`go-presence-probe.go` 和 `go-optional-nesting-review-probe.go` 的结果：

| 输入/操作 | 结果 |
|---|---|
| 普通整数 0 | has=false |
| nil 整数指针 | has=false，读取 0 |
| 指向 0 的指针 | has=true |
| nil / 非 nil 空切片 | 都读取空列表，has 分别为 false / true |
| 已设置的 OptionalNone 字段 | has=true，内容 hasValue=false |
| 对上述字段做 optional select | 保留外层 optional，不压平内部 none |
| 用 optional.none 构造 *types.Optional 字段 | 本地基线报解引用错误 |

最后一项是所测 Go 路径的限制，不是 cel-cxx 应复制的合同。Java native presence/Optional 的对照来自源码及已有测试断言，未运行 Java Native Types 测试套件。

## required 构造与默认对象

### 各实现是否接受缺 required

使用 proto2 `Parent`，其中 `child` 是 required message：

| 验证路径 | 结果 | 验证层级 |
|---|---|---|
| cel-go `Registry.NewValue` 与 CEL `Parent{}` | 成功，has_child=false；`proto.CheckInitialized` 报缺失 | 完整 Go compile/evaluate 加 protobuf 检查 |
| cel-cpp `NewValueBuilder(...).Build()` | 成功，has_child=false，IsInitialized=false | 真实 C++ ValueBuilder |
| Java protobuf DynamicMessage `build()` | 报缺 required | protobuf builder；未运行 Java CEL 端到端 |
| Java protobuf DynamicMessage `buildPartial()` | 成功，保留不完整状态 | protobuf builder |

Java CEL 的调用点使用 `build()`，这是源码结论。Java 探针使用 protobuf 3.25.5 / JDK 25.0.3；不能把它标成完整 cel-java 集成测试。

因此，core 中“Build 拒绝缺 required”是适应 stock prost 表示能力的明确选择，不是声称所有 CEL 引擎都会拒绝。原始探针：`required_go.go`、`required_cpp.cc`、`RequiredProbe.java`。

### 无环 required DAG 也会展开

`prost_required_default.rs` 定义 18 个 message 类型：前 17 层各含两个指向下一层的 required Box message，末层为 required int64。schema 无环。

调用顶层 `M0::default()` 实测产生 **262,142 次分配，3,145,696 bytes**。这证明仅拒绝 required 环不足以控制 builder 初始化；字段槽位必须在完整值组装前保留 Unset，不能先 Default 整个对象。此探针验证旧构造方式的成本，没有验证新 builder。

## owner 与投影

| 探针 | 结果 | 对设计的约束 |
|---|---|---|
| 安全 Rust root/child 投影 | non-Clone 子对象保活，根 Arc 可恢复，普通 child 没有独立 Arc | 保留 owner 与真实引用；不伪造 Arc<&T> |
| parent-source 链的 20,000 次 identity 投影 | 1 MiB 栈线程释放时栈溢出 | 不把历史投影保存成强引用 parent 链 |
| C++ protobuf 子 message 选择/Clone | 同 arena 使用原 child；跨 arena Clone 复制 | 不能将 C++ 指针/arena 合同直接当作 Rust Arc owner 合同 |
| Java protobuf 子 message 选择 | 所测 builder/message 返回同一 child 引用 | GC 下的 child 引用可直接保活；不说明所有 CEL 转换零复制 |

parent-source 反例来自 `projection_chain.rs`；C++/Java 结果来自上一节的 required/owner 探针。这是早期 Arc/投影方案的证据。最终设计采用拥有型字段副本，不再要求持久 selector 路径；parent 链反例仍用于防止内部 view/token 重新形成历史链，不能把旧探针算作 Box 方案已通过。

## protobuf 相等与 PartialEq

`prost_eq_cpp.cc` 与 `prost_eq_rust.rs` 对照真实 cel-cpp `ParsedMessageValue::Equal`、protobuf `MessageDifferencer::Equals` 和标准 prost 派生 PartialEq，覆盖三类字段各三组值，共 9 组。

```proto
syntax = "proto3";
message Implicit { double x = 1; }
message Explicit { optional double x = 1; }
message Repeated { repeated double x = 1; }
```

| 值对 | 隐式字段：Rust / C++ | 显式 optional：Rust / C++ | repeated 元素：Rust / C++ |
|---|---|---|---|
| +0.0 / -0.0 | true / **false** | true / true | true / true |
| NaN / NaN | false / false | false / false | false / false |
| 1.25 / 1.25 | true / true | true / true | true / true |

C++ 的两个比较入口结果一致。隐式 +0/-0 的差异来自 presence：分别为 absent / present；不是浮点数值比较本身采用位相等。核心设计因此优先复用兼容的派生 PartialEq，仅补已确认的 presence 与 default-view 处理。

`builtin_equal_error.cc` 还比较了含 repeated Any 的真实 C++ 消息：已知 Int64Value 的有效 payload 比较成功，坏 payload 返回外层 `INVALID_ARGUMENT: failed to unpack protocol buffer message`。它证明引擎存在可失败的比较路径，但 Any 不在首版范围，不能据此要求普通 Native 采用可失败字段比较。

早期人工 getter-error 探针则发现 list Contains 可把自定义 Equal 返回的 CEL Error 忽略为 false。它是旧的可失败比较方案的反例；新方案使用 bool comparator，不把该反例自动升级为全面改写 C++ equality 的要求。

## overload ID 与空集合

`overload_end_to_end.cc` 使用 PR #2115 对应基线，覆盖 parser → checker → runtime；checker 候选顺序变化时，runtime 注册顺序保持不变。list/map、两种声明顺序、matcher 开关及 typed/裸空值共 24 次求值均完成。

| 调用 | 观察结果 |
|---|---|
| `xs: list<int>`，绑定 [] 后调用 f(xs) | 根据 checked ID 选择 int 重载，不受候选声明顺序影响 |
| typed string list 与两类 typed 空 map | 根据声明类型选择对应重载 |
| f([]) / f({}) | checker 可能按候选顺序绑定自由类型变量，产生相应唯一 ID |
| matcher 开启/关闭 | 上述区别相同 |

该探针没有证明动态多候选已实现 ambiguity Error；这仍是核心设计的补齐项。optional.none 的候选顺序风险来自 checker/matcher 源码分析，不能将空 list/map 的端到端记录冒充 optional 的同类测试。

## 统一 Box dyn Struct trait 模型

**历史模型：以下记录保留当时的 NativeType + sealed Struct 方案，当前接口以核心设计的 Struct/TypedStruct 分层为准。** 原探针说明对应模型在语言层可行，不是当前扩展入口的定义。

2026-10-10 使用 rustc 1.96.1、edition 2021 与 dyn-clone 1.0.20，运行独立的 `struct-model` 库和 `consumer` 下游 crate。模型定义 `StructValue = Box<dyn Struct>`、sealed 动态接口、typed definition/helper、拥有 T 的 NativeInstance、schema-only 实现与拥有宿主快照的合成 view；未改动生产代码。

| 模型检查 | 实际结果 |
|---|---|
| 下游手写 NativeType/typed definition，通过安全 factory 得到 StructValue | 编译运行通过；无需下游实现私有 trait，宿主无需 Debug/Display |
| DynClone 与 dyn Struct 的 PartialEq；通用 Sized Box<T> 与 StructValue/OpaqueValue 专用类型描述 | 编译通过，Clone 调用宿主一次，消费该副本不再 Clone；浮点 NaN 与自身不相等 |
| 消费真实 root 恢复 T | Clone 计数为 0，最终 T 只析构一次 |
| wrong-TypeId 消费恢复 | 返回原 box；真实宿主地址不变，随后仍可成功消费恢复 |
| 从 &Parent getter 返回拥有型 Child | 复制 Child 一次；释放 parent 后可继续读取与恢复 Child，析构计数符合实际副本数 |
| 合成 view 内部保存真实宿主 | 公有借用/消费 downcast 均拒绝；失败后 view 仍可访问、比较并独立存活 |
| schema-only 值 | 没有虚构的 () host；同定义与不同定义的比较符合模型规则 |
| Struct 直接继承 Clone / 在 vtable 放泛型方法 | 两个独立编译负例均按预期 E0038 |
| 下游直接实现 sealed Struct | 编译负例按预期 E0277；该旧模型只开放 typed definition/helper 入口 |

正例运行通过，三个负例符合预期。探针文件为 `model/src/lib.rs`、`consumer/src/main.rs`、`clone_supertrait.rs`、`generic_vtable.rs` 与 `forge_sealed.rs`。这是 Rust 接口与所有权模型，**未验证真实 CEL/FFI、arena/async 结果提取、derive/codegen、protobuf profile 或 MSRV 1.80**，也没有 Box/Arc 性能基准。最终验收仍须覆盖 §5.4 的完整消费通路与 §18 的集成矩阵。

## 单一 Struct trait 与直接宿主模型

**历史备选模型：单一 trait 在语言层可行；最终选择 Struct/TypedStruct 分层以保持现有 Opaque/TypedOpaque 的接口组织方式。** 以下结果保留原模型和实际运行范围，不作为当前接口定义。

2026-10-10 使用 rustc 1.96.1、edition 2021 与 dyn-clone 1.0.20，另运行 `single-struct-model` 库和 `consumer` 下游 crate。模型删除 NativeType 和 NativeInstance；Struct 同时包含带 Self: Sized 的 static_definition 与动态 definition/get/equality，内部 blanket 提供实际 Self 的 Any 擦除。下游直接为 User 实现 Struct，Box 直接保存 User；另有保存 User 的私有 View。

| 模型检查 | 实际结果 |
|---|---|
| 无实例调用 T::static_definition 注册，与 Box<dyn Struct> 共存 | 编译运行通过；User 无 Default/Debug/Display 要求 |
| 下游直接 impl Struct for User，内部 Any trait 自动满足 | 编译运行通过；不需要另一宿主 trait 或包装层 |
| 消费完整 User | Clone 计数为 0 |
| 错误类型的消费 downcast | 返回原 Box，User 地址不变，随后可成功取回 |
| 动态 Clone / PartialEq | Clone 计数增加 1，副本比较符合模型的宿主规则 |
| View 内部持有 User | 借用/消费 downcast::<User> 均拒绝，失败后的 view 仍可读取字段 |
| `<dyn Struct>::static_definition()` | 编译负例按预期 E0277；方法的 Self: Sized 不影响 Struct 的 dyn compatibility |

正例以 `cargo run --offline -p consumer --bin consumer` 通过，`static_on_dyn` 编译负例符合预期。文件为 `model/src/lib.rs`、`consumer/src/main.rs` 与 `consumer/src/bin/static_on_dyn.rs`。该模型只验证单一 trait、直接存储与恢复的语言/所有权结构；**未验证实际 derive、完整 StructContext/字段错误、registry/profile、CEL/FFI、arena/async 或 MSRV 1.80**。

## Struct/TypedStruct 分层模型

2026-10-10 使用 rustc 1.96.1、edition 2021 与 dyn-clone 1.0.20，运行独立的 `typed-struct-model` 库和 `consumer` 下游 crate。模型使用 TypedStruct: Struct + Clone + PartialEq，由库统一 `impl<T: TypedStruct> Struct for T`。下游 User 只实现 TypedStruct，另有只实现动态 Struct 的 DefaultView；定义与字段值采用简化模型，未修改生产代码。

| 模型检查 | 实际结果 |
|---|---|
| 下游 TypedStruct 与库 blanket 共存 | 编译运行通过；User 无 Default/Debug/Display 要求，Box 直接保存 User |
| `<User as TypedStruct>::definition()` 无实例注册 | 与动态 definition 一致；具体类型的同名方法可通过 UFCS 调用 |
| 动态 get/has/equality/zero 转发 | 结果正确；getter 的错误保留；NaN 与自身不相等 |
| 完整 User 的消费恢复与动态 Clone | 原值移出时 Clone 计数为 0；动态 Clone 增加 1 次 |
| 错误类型的消费恢复 | 返回原 Box，User 地址不变，随后仍可成功取回 |
| DefaultView 只实现 Struct | 无 TypedStruct/PartialEq 要求，仍可 get/has/比较/zero；借用及消费 downcast::<User> 均拒绝 |
| 模型中 User 与 DefaultView 的自定义比较 | 双向比较一致；blanket 保留实现提供的 comparator，不强制只做同 Rust 类型 PartialEq |
| `Box<dyn TypedStruct>` | 编译负例按预期 E0038；动态对象使用 dyn Struct |
| 将仅实现 Struct 的 DefaultView 作为静态注册根 | 编译负例按预期 E0277，缺少 TypedStruct |

正例以 `cargo run --offline -p consumer --bin consumer` 通过，`dyn_typed_struct` 与 `dynamic_root` 两个编译负例符合预期。文件为 `model/src/lib.rs`、`consumer/src/lib.rs`、`consumer/src/main.rs` 与两个负例 bin。该验证覆盖跨 crate 的分层、blanket、动态转发和所有权结构；**未验证实际 derive、完整 registry/profile、protobuf comparator、CEL/FFI、arena/async 或 MSRV 1.80**。当前接口以核心设计 §4–5 为准，完整完成条件仍是 §18。

## Provider 安装协议的源码核对

2026-10-10 按本地 cel-cpp `c84e883` 和 cel-cxx 当前源码复核调用链，细化 [核心设计 §9.2](../design.md#92-provider-安装)。本轮没有修改或编译 provider 生产代码；以下全部为**源码核对**，不能记作新接口的测试通过。

| 核对对象 | 实际观察 | 确定的设计处理 |
|---|---|---|
| TypeCheckEnv 的 provider 列表 | 逐个查询；已知类型的字段 miss 仍会继续 | 安装已含 pool 的组合 provider；Native 类型名 miss 可回退，命中后字段 miss 终止 |
| checker 库配置/context/type mask | 库配置在 Build 展开；context 字段另走 proto provider | 最终配置统一校验，首版拒绝这些旁路 |
| TypeRegistry/RuntimeTypeProvider | 保存 final 具体对象；现有 RegisterType 仅接受 Opaque；没有 reflector 注入口 | 增加一次性 InstallTypeProvider，保留稳定 facade 与 RuntimeEnv |
| planner 与 program | 分别经过 GetRuntimeTypeProvider 与 GetComposedTypeProvider | 两入口继续共享同一个带 delegate 的对象 |
| RuntimeTypeProvider 的默认分派 | WKT → pool → Opaque；builder 直接使用 pool | 普通 struct 增加 Native 优先分支，保留 Opaque 与 protobuf fallback；字段/构造与类型选择一致 |
| enum 与 Resolver | enum 从独立缓存读取；FindType 非 OK 当前被忽略 | 合并显式/FDS enum，缓存 miss 后查询 FindEnumConstant；FindConstant 增加 StatusOr 传播 |
| pool enum 常量/字段 | introspector 与 Type::Field 可能返回 nominal EnumType，包括复合字段中的元素 | 组合层统一 int；pool 子 message 若被 Native 覆盖，拒绝父闭包，不能仅投影为 BasicStruct |
| FDS helper 与 pool | 已遍历 FDS 顶层/嵌套 enum；pool 另含 generated underlay | 保留收集并校验完整重复定义，underlay enum 不依赖全量扫描才能查询 |
| TypeSpec 恢复 | 全局 WKT 优先；未知 abstract name 变成 Opaque；命中时未使用输入参数；planner 另有忽略解析失败的调用点 | provider-only 模式贯穿全 type_map、planner 与返回类型，恢复实际参数并传播失败；新参数使用独立解析 arena |
| checker/planner literal 与 create-struct evaluator | checker 不查询 constructor 能力；planner 已检查类型和字段；求值 builder nullptr 为 CEL Error | checked 遍历按来源校验；只读 Native 不回退，Native miss 的 pool literal 仍可构造 |
| TypeIntrospector::ListFields | 声明区分 nullopt 与空列表；WKT helper 的空初始化不保留已知状态 | 已编译 provider 显式产生 Some(empty) |
| Rust reflector/value-builder FFI | metadata 与 builder 共用 'a；setter 结果缺少 CEL Error 分支 | 独立 metadata owner、短借用输入、owned Rust state 与专用三分支 bridge |

具体文件与固定版本链接集中在 [provider 调用链依据](native-interop.md#provider-必须覆盖完整调用链)。本轮已按 Native 优先/pool 回退修订方案；这些是源码推导，**没有运行新的 provider 集成测试**。文档检查仅覆盖相对链接/锚点、代码围栏与 diff 格式；SetTypeProvider/InstallTypeProvider 的实际 ABI、enum 缓存与按名查询的一致性、pool 投影并发、arena 释放及跨 carrier 负例仍须实现验证。

## 组合 provider 与值表示验证

2026-10-11 在 cel-cpp `c84e883df5932dcc992dd3494a771c37e85ae1fe`、C++ protobuf 35.1 上编译并运行独立 C++ probe。使用真实 checker、runtime、ParsedMessage 和 CustomStructValueInterface；**没有使用尚未实现的 cel-cxx EnvTypeProvider 或 Rust Native bridge**。

夹具包含 pool `probe.Parent.child: probe.Child`，Child.x 为 string，Child.secret 默认值为 `classified`；另有零字段 probe.Blank 与 value 为 google.protobuf.NullValue 的 probe.NullHost。为检验“只投影 metadata”的假设，Parent/Child 的 checker 使用独立 pool 和单一自定义 introspector：向 checker 声明 Parent.child 为 BasicStruct Child、Child.x 为 int，且不公开 secret；runtime 使用实际 descriptor pool。NullHost 用真实 pool 的标准 checker。StrictNativeEmpty 是真实 C++ Custom 接口，声明 probe.Blank，只接受自身 NativeTypeId 的 Equal。

| 输入/路径 | 实际输出 | 证明的边界 |
|---|---|---|
| `probe.Parent{}.child.x + 1` | `UNKNOWN: No matching overloads found : _+_(string, int64)` | checker 的 int 投影不能改变运行时 Child.x 的 string |
| `type(probe.Parent{}.child.x)` | `string` | 实际 ParsedMessage 仍按自身 descriptor 读取 |
| `dyn(probe.Parent{}).child.secret` | `"classified"` | 运行时 dyn 选择能读到 checker 未公开的字段 |
| `probe.NullHost{value:0}.value + 1` | `UNKNOWN: No matching overloads found : _+_(null_type, int64)` | stock pool 普通 NullValue 字段的读取与 int 声明不一致 |
| `type(probe.NullHost{value:0}.value)` | `null_type` | 只修 checker 不能修实际 C++ enum accessor |
| StrictNativeEmpty == Parsed Blank | `false` | Custom 覆写 comparator 的定义检查生效 |
| Parsed Blank == StrictNativeEmpty | `true` | Parsed 侧字段 helper 绕过同一个 comparator，产生顺序差异 |

输出中的 UNKNOWN 是 CEL ErrorValue 所包装的 Status code 文本，不是 CEL UnknownValue；这些表达式的 Evaluate 调用成功返回了错误值。Parent 的 metadata 投影是为了隔离反例而手写的模型；它证明此投影方案不足以保证实际值一致性，**不证明新增 provider 安装接口已经实现或有生产回归**。NullHost 与混合相等则直接复现基线的实际值路径。

由此修订的合同集中在核心设计：[pool 表示闭包准入](../design.md#923-五个查询构造方法的合同)、[双向相等分派](../design.md#122-protobuf-相等与-partialeq-的复用)、[普通 NullValue 的 C++ 修补](../design.md#117-enum)。实现后的通过条件分别是：Parent 在该覆盖环境的查询/规划阶段拒绝；普通 NullValue 全部字段路径得到 int/raw number；Parsed/Custom 两个顺序调用相同 Custom comparator。既有反例输出是基线证据，不能记成修补已通过。

无 arena predicate 的预算边界与字段副本成本属于设计合同/成本推导，未测生产性能。prost-types 0.14.1 FileDescriptorProto 无 edition 的判断来自本地 cargo registry 与[该版本源码](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-types/src/protobuf.rs)的字段核对。同一源码中 Syntax 只有 Proto2=0/Proto3=1，而本地 C++ protobuf 35.1 的 `google/protobuf/type.proto` 还定义 SYNTAX_EDITIONS=2；内建常量必须绑定完整 enum schema，不能直接注册 Rust 旧子集。以上为版本源码核对，不代表整个 prost-types adapter 已生成验证。

### checked AST 重新检查验证

同日扩展上述 C++ probe，源 checker 的 Child.x 为 int，目标 checker 使用真实 pool 的 Child.x:string。先取得源 checked Ast，再以其 expression/source info 创建新 `Ast(expr, source_info)`；断言它没有 checked 标志、type_map 或 reference_map，调用目标 `TypeChecker::Check`。

| 输入 | 实际重新检查结果 |
|---|---|
| `probe.Parent{}.child.x + 1` | REJECT；源返回 int，目标字段 string 无法加 int |
| `probe.Parent{}.child.x` | ACCEPT；新 Ast 返回类型 string，不沿用源 int |

probe 编译与运行退出码均为 0。该结果验证基线 checker 能重查已经展开的简单表达式，支持核心 §9.2.7 的接口选择；**未验证 cel-cxx 的序列化导入、旧 metadata 清理实现、函数/宏全矩阵、ValidateStructConstructions 或 strict TypeSpec 的新增 glue**。生产导入仍必须完整实现并测试。

### ValueContext 接口的 Rust 编译验证

同日从核心 §6.2 原文提取 FromValue/FromOwnedValue traits，以 rustc 1.96.1、edition 2021 编译独立模型。模型用简单 Value 与私有预算字段替代生产 registry/arena/profile，Option/Vec 的 borrowed/owned 递归调用统一的带 context 方法。

`rustc --edition=2021 --test context.rs` 与 `context-tests --test-threads=1` 的实际结果为 **6 passed，0 failed，0 ignored**：最外层 borrowed/owned 便利入口各只创建一次 context；递归 borrowed/owned 共用预算；借用输出在局部 context 析构后仍可引用输入；嵌套 some(none) 保持层次。模型另编译为 rlib，独立 consumer 实现借用输出 decoder 并调用默认便利方法，编译/运行退出码均为 0，确认默认方法调用库内私有构造器不妨碍下游使用。

负例尝试在带 context 的 decoder 返回 `ctx.label.as_str()`；编译退出码 1，诊断 `lifetime may not live long enough`，符合输出寿命只能绑定输入 Value 的合同。此模型验证接口的 GAT/跨 crate/基本传递结构；**没有验证实际 NativeOperationContext、RAII depth、profile、FFI、async、并发或 MSRV 1.80**，不能替代核心 §18 的正式验收。

## 仍需完整实现验证的内容

正式完成条件集中在核心设计的 [验收矩阵](../design.md#18-验证与验收矩阵)：真实 FFI/arena 生命周期、统一 registry、生成器的跨 crate 使用、字段槽位构造、消费型所有权恢复、错误归一化、bool 分派、feature/MSRV/平台组合及性能测量。

必须新增真实集成覆盖：pool 覆盖依赖闭包（含回边/空容器/未选中 oneof/dyn）、两个操作数顺序的相等与 membership、无 Native Env 的 NullValue accessor/setter、同 operation 的递归解码与纯 has、跨 Env 旧字段/函数 metadata 的重新检查，以及 root/重复输出/深拷贝字段/async 取消的最小副本与释放计数。执行顺序和证据记录见[实施计划](../plan-native-types.md)，不能仅复跑上面的基线反例或旧 Rust 模型就宣称完成。

新增结果应在这里记录版本、输入和实际运行层级；语义变化回写核心设计。核心文档中的未来 v4、Any 等边界不因一个局部探针通过就视为已支持。
