# Native 接入的跨语言实现与源码依据

定位：[核心设计](../design.md) 的补充材料，说明 Go/Java/C++ 的实际行为、cel-cxx 的接入缺口和未采用方案。类型准入、API、profile 与验收要求只在核心设计维护。

调研基线为 2026-10-09，按 2026-10-10 的设计修订整理。下文的“当前实现”均指该基线，不表示目标 Native bridge 已实现。仓库提交与实际运行结果见 [验证记录](validation.md#基线与证据层级)。

[返回设计目录](../README.md) · [两类 profile](../design.md#31-两种语义-profile)

## 如何使用这些参考

Go native、Java Native Types 与 C++ CustomStruct 共同支持“类型描述、presence、读取分开提供”，但并不规定同一套宿主语义。本文保留它们的差异，避免把某个语言的默认值、指针或 GC 规则当作 Rust 的要求。

| 关注点 | 参考实现提供的依据 | 核心设计的对应选择 |
|---|---|---|
| 普通字段 | Go 的零值 presence；Java 的引用/Optional 规则；C++ 独立 get/has | 实际 struct/record 字段始终存在，Option 保存 CEL optional；Rust enum 选择字段另按分支 presence |
| protobuf 字段 | descriptor 决定 presence、default、enum、oneof、WKT | prost 使用 Protobuf profile，覆盖存储类型推导 |
| 整体相等 | 宿主比较与 protobuf 比较是不同能力 | 普通 Rust 用 PartialEq；prost 复用兼容的派生 PartialEq 并补必要差异 |
| 构造 | 从空状态逐字段赋值，完成时检查的严格程度不同 | 字段槽位 builder，Build 只发布完整 Rust 对象 |
| 子对象 | GC 引用或 C++ owner/view 分离 | Box 拥有实际宿主；字段返回独立值，求值内部 view 另受寿命约束 |

## Native Types Library 与 Env 扩展

Native Types 的 library 入口负责把宿主类型能力安装到环境中，安装内容包括 type/value provider，不限于函数或宏。

这里的 registry 与 provider 不是两个互斥方案：Go 的 registry 同时承担 provider/adapter，NativeTypes 组合后调用 CustomTypeProvider/CustomTypeAdapter；Java 的 NativeTypeRegistry 实现 CelTypeProvider/CelValueProvider，再装到 checker/runtime。Rust 设计据此把 provider 定为引擎扩展边界，把冻结 registry 定为其内部定义来源。依据：[Go 的组合入口](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/provider.go)、[Java NativeTypeRegistry](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java)。

| 实现 | 入口与安装内容 | 依据 |
|---|---|---|
| cel-go | `ext.NativeTypes(...)` 返回 EnvOption，组合已有 provider/adapter，再通过 Env option 安装 | [扩展入口](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/ext/native.go#L105) |
| cel-java | `CelExtensions.nativeTypes(...)` 返回同时实现 CelCompilerLibrary/CelRuntimeLibrary 的库；checker/runtime 使用同一个 NativeTypeRegistry | [工厂入口](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelExtensions.java#L362)、[两侧安装](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L143) |
| cel-cpp | 尚无 Native Types library；已有通用 BasicStruct/CustomStruct/provider/builder 接口 | [官方支持矩阵](https://cel.dev/reference/api-reference#native_types_library)、下文的 C++ 接口对照 |
| cel-cxx 基线 | `with_ext_*` 在 EnvBuilder 上配置扩展，Env 构建时分别安装 compiler/runtime 能力；尚无 Native Types 入口 | [Env 配置](../../../../src/env/mod.rs#L457)、[compiler 安装](../../../../src/env/inner.rs#L124)、[runtime 安装](../../../../src/env/inner.rs#L305) |

据此，核心设计将 Rust 宿主适配作为 cel-cxx 自身的 Native Types 扩展，通过 `EnvBuilder::with_native::<T>()` 注册类型并安装所需支持。Go/Java 的入口本身就接收要公开的类型，不要求先开启空库；扩展的组织方式不要求沿用 cel-cxx 既有的布尔开关。底层通用 struct、类型解析和值转换仍由核心设施承载；不能把“C++ 没有现成库”理解为缺少所有接入接口，也不能把 extension 等同于必须新增 Cargo feature 或独立 crate。具体安装与注册规则只在 [核心设计 §3.3](../design.md#33-native-types-扩展的边界) 维护。

## 各实现的实际约定

### Go

Go native 字段的类型描述使用 `FieldType{Type, IsSet, GetFrom}`，类型、presence 和读取是三个独立部分。普通指针字段的 CEL 类型通常递归取指向类型，不自动变成 CEL optional。

当前 native presence 主要使用反射的 `IsZero()`。这意味着：

| Go 字段和值 | 读取结果 | has |
|---|---|---|
| int64 为 0 | 0 | false |
| *int64 为 nil | 0 | false |
| *int64 指向 0 | 0 | true |
| []string 为 nil | 空列表 | false |
| []string 为非 nil 空切片 | 空列表 | true |
| *types.Optional 保存 OptionalNone | CEL optional.none | true |

不能把 Go 的行为概括成“集合非空才算存在”。nil 切片与非 nil 空切片的 presence 不同。

缺失的指针标量读取为默认值；普通嵌套 struct 指针的 nil 读取通常适配一个默认对象。顶层 nil 指针的适配又可能得到 CEL null，所以顶层适配和字段读取不是同一规则。

Go 的 CEL optional 是 `types.Optional`，与普通指针 presence 分开。当前源码对实现了 CEL `ref.Val` 的字段也有特殊类型映射。

构造 native struct 时，Go 创建一个零初始化对象，转换并设置提供的字段；未提供字段保留零初始化状态。比较使用 `reflect.DeepEqual`，因此可能包含未暴露给 CEL 的宿主字段。

依据：[FieldType](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/provider.go#L80)、[字段类型与构造](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L304)、[presence 和比较](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L453)、[指针读取默认值](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L1443)、[类型映射](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L1508)。

### Java

Java Native Types 扩展对普通对象使用 `PojoStructValue.select(field)` 和 `find(field)`：

- select 读取字段；Java null 会替换为该字段类型的默认值。
- find 使用原始 getter 结果的 `Optional.ofNullable(value)`，供 presence 检查使用。
- 因此，primitive 的 0、false，以及非 null 的空字符串、空集合，都算已设置；null 引用不算已设置。
- 普通嵌套对象为 null 时尝试创建默认实例；没有可用构造能力时，字段读取可能失败。

Java 的 `java.util.Optional<T>` 明确映射成 CEL optional。一个非 null 的 `Optional.empty()` 对象仍然算字段存在，和字段引用为 null 是不同状态。这个结论可以直接从 find 与 Optional 类型映射推导；本次没有单独运行 Java 探针。

构造 POJO 使用无参数构造函数，再设置字段。没有构造函数、没有 setter 或字段只读时，构造可能在运行时失败。Native Types 当前要求 planner runtime。

POJO 包装本身没有提供按可见字段递归比较的实现；普通对象的比较最终受宿主 `equals()` 等运行时规则影响，不应假定与 Go 的 DeepEqual 相同。

依据：[Optional 类型映射](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L334)、[构造](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L713)、[select 和 find](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L1050)、[默认读取与 presence 测试](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/test/java/dev/cel/extensions/CelNativeTypesExtensionsTest.java#L846)、[RuntimeEquality](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/runtime/src/main/java/dev/cel/runtime/RuntimeEquality.java#L139)。

### Native enum 的实现差异

下表区分源码实际支持的适配方式，不把 Java enum、Go 命名标量和 protobuf enum 当作同一种类型。Go/Java 均以本文固定提交为依据；这里没有新增 Java 运行探针。

| 实现/来源 | 实际行为 | 不能据此承诺的能力 |
|---|---|---|
| Go 自定义整数/字符串类型 | 字段类型和读取按 reflect.Kind 映射到 int/uint/string；宿主对象注册只接受 struct | 自动发现 Go const、保留名义 enum 类型，或推导 Rust payload variant |
| Java 普通 enum | scanner 可把 enum class 注册成 StructType，converter 把已注册 enum 实例包装为 PojoStructValue；过滤 Enum 标准方法与 static 字段 | 自动使用 ordinal/name、导出 FOO/BAR 常量、或用无参 constructor 创建 enum |
| protobuf enum | descriptor 提供全名、数字、aliases 与字段语义；属于 protobuf 适配 | 用普通宿主 enum 的内存 discriminant 代替 schema |

Go 的 `convertToCelType` 和 `getFieldValue` 按 kind 处理命名标量；`Registry.EnumValue/FindIdent` 的内建 enum 查询来自 protobuf database，并非扫描 Go const。可确认 `type State int` 字段按 CEL int 适配；这不意味着 `NativeTypes(State(...))` 会注册名义 enum。

依据：[字段类型](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L1508)、[字段读取](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L1393)、[struct 准入](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/native.go#L618)、[enum 查询](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/provider.go#L290)。

Java 的 `nativeTypes_enumSafelyIgnored` 测试定义 FOO/BAR，检查 `PojoWithEnum{}.enumVal` 可以返回非 null；不能因测试名称中的 ignored 而解释为完全拒绝 enum，也不能把这个测试扩大为常量、比较或构造均已验证。沿 scanner 和 converter 可推导该值走宿主 StructValue 路径，isGetter 过滤标准 Enum 方法，没有 ordinal/name 标量转换分支。

依据：[class 扫描与 StructType](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L181)、[过滤 static/Enum 方法](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L596)、[实例适配](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/main/java/dev/cel/extensions/CelNativeTypesExtensions.java#L838)、[enum 测试](https://github.com/cel-expr/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/extensions/src/test/java/dev/cel/extensions/CelNativeTypesExtensionsTest.java#L1451)。

因此 Rust 的 scalar 与对象视图都可参考现有宿主适配机制，但 Rust 各形态仍需自己的映射合同。核心设计的 Struct derive 覆盖普通 struct 与非空 enum，以 tag 和有条件 presence 的 payload 字段提供 e.A/e.B.x/e.C.x 访问；不由 Go/Java 的实现推导 Rust 内存布局或 enum 数值。准入与 profile 边界统一见 [核心设计 §4.6](../design.md#46-普通-rust-类型的准入条件与-enum-边界)。

### Rust 语法形态与映射依据

本次按 Rust Reference 的 item/variant 语法重新梳理，而不是只把 Rust struct 当作具名字段 record。语言事实与本库选定的 CEL 映射分开：

| Rust 语言事实 | 对本设计的影响 | 依据 |
|---|---|---|
| struct 包含 named、tuple、unit 形态；单字段 tuple 常称 newtype | 分别生成正确读取/初始化语法；保留名义类型，不因一个字段而自动透明 | [Structs](https://doc.rust-lang.org/reference/items/structs.html) |
| variant 可以无字段、含 unnamed fields 或 named fields；variant 不是可直接命名使用的独立类型 | 单 payload T 使用真实 T；record/多字段 payload 通过真实 E owner 的合成 CEL view 访问 | [Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html) |
| 裸 tuple/unit 是类型语法，与 tuple/unit struct 的 nominal item 不同 | `(T,U)` 不自动获得 struct schema；裸 () 的 null 映射不套到 S; 或 S() | [Tuple types](https://doc.rust-lang.org/reference/types/tuple.html) |
| repr 规定布局/ABI，不定义业务字段语义；packed 不允许普通未对齐字段引用 | transparent/整数 discriminant 不决定 CEL 映射；自动 derive 拒绝 packed | [Type layout](https://doc.rust-lang.org/reference/type-layout.html) |
| 泛型、const 参数、where 和 non_exhaustive 会影响实例及可构造范围 | 具体实例满足能力与稳定命名；在定义 crate 内展开当前形态，不反射外部未来字段 | [Generics](https://doc.rust-lang.org/reference/items/generics.html)、[Non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) |
| union 无安全的自动活动字段判断；alias 不创建新的名义类型 | union 不自动映射；alias 沿用原类型身份 | [Unions](https://doc.rust-lang.org/reference/items/unions.html)、[Type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html) |

对于 C{x,y}，合成 view 让 CEL 保持 `e.C.x` 的自然访问，同时在 Rust 内仅持有真实 E 并安全 match；无需复制到另一份 record。选择字段 has(e.C) 反映当前 variant，未选中直接访问为 CEL Error，安全选择使用 e.?C。此规则不是 Rust 自带的 CEL 行为，也不是 protobuf 的默认值规则，而是本库的明确选择。

规范性矩阵见 [struct](../design.md#47-struct-的逐形态映射)、[enum variant](../design.md#48-enum-variant-的逐形态映射) 和 [限制](../design.md#49-修饰组合与明确不支持项)；owner/downcast 约束见 [§5.6](../design.md#56-enum-真实-payload-与合成-variant-record)。语言及 owner 模型的运行结果见 [形态验证](validation.md#rust-structenum-形态与-variantview)。

### C++

cel-cpp 当前没有同等的 native 扩展库，但已有 `BasicStructType`、`TypeIntrospector`、`CustomStructValueInterface`、`TypeReflector` 和 `ValueBuilder`。

CustomStructValue 的读取和 presence 接口独立。普通 select 调用 GetFieldByName；optional select 先调用 HasFieldByName，再读取并包装成 OptionalValue。因此 native 对象可以自行提供 presence 规则。

protobuf 的规则另由 schema 与反射决定：

- 显式 presence 字段区分未设置和已设置的默认值。
- 隐式 presence 标量按默认值判断。
- repeated 和 map 字段按是否非空判断。
- 普通未设置嵌套 message 可以读取默认对象；wrapper 字段在相应 unboxing 规则下读取为 null。

OptionalValue 是独立的 CEL optional 表示。C++ 的 `std::optional` 在 provider 查询、查找结果等 API 中广泛用于表达“没有结果”，这些位置不能据此认定是在表示 CEL optional。

C++ checker 有 primitive wrapper 类型，以及默认开启的历史 null 可赋值规则，包括某些 struct 和 opaque 类型。**不能简单声称 C++ 完全不允许 struct 为 null；也不能把这些历史兼容规则当作任意 Nullable 类型的完整设计。**

wrapper 的未设置读取还受配置影响：当前 RuntimeOptions 的 enable_empty_wrapper_null_unboxing 默认 false，普通表达式 select 可以沿历史行为读取标量默认值。规范规定的 null unboxing 需要启用对应选项；下文区分这个实现默认值与推荐的统一语义。

依据：[CustomStructValueInterface](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/custom_struct_value.h#L179)、[optional select](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/select_step.cc#L165)、[protobuf presence](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.cc#L395)、[null 类型兼容规则](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_inference_context.cc#L54)。

### cel-cxx

当前 `Option<T>` 和 `Optional<T>` 都实现了到 CEL optional 的类型与值转换。该设计有明确的 Java Optional 类比，本身不能判定为错误。

但 prost 消息内部的 Option 字段并不经过这套普通值转换。当前消息整体序列化后交给 C++ protobuf 反射，所以它们遵循 protobuf presence。cel-cxx 已经同时存在“Option 作为一个 CEL 值”和“Option 作为消息字段存储方式”这两种上下文。

当前 Rust StructValue 保存类型名和消息字节；高层类型转换要求 protobuf descriptor；结果转换对所有 CEL struct 调用 GetMessage。这些通路需要泛化，native 对象才能完整往返。

依据：[Option 与 Optional 的转换](../../../../src/values/impls/optional.rs#L4)、[prost presence 示例](../../../../examples/protobuf-prost-derive.rs#L101)、[StructValue](../../../../src/values/mod.rs#L205)、[类型转换](../../../../src/ffi/types.rs#L16)、[值转换](../../../../src/ffi/values.rs#L155)。

**判断：几套实现一致支持将类型、presence 和读取分开，但不存在所有 native 字段都按同一种规则判定 presence 的约定。cel-cxx 应选择明确的 Rust 语义，并说明与 Go、Java、protobuf 的对应关系。**

Native Type Library 的职责是注册宿主类型，并提供字段描述、对象适配和构造能力；底层 checker 与 runtime 则通过通用 struct 接口使用这些能力。cel-cpp 缺少的是完整的 native 库与接入通路，已有的基础接口足以作为实现起点。[Native Types Library 参考](https://cel.dev/reference/api-reference#native_types_library)

## protobuf 字段与 wrapper 的实现依据

同一个 `Option<i64>` 存储，可能是普通 Rust optional、protobuf optional scalar，或 primitive wrapper。描述符身份决定后两种含义；仅观察 Rust 类型不能区分。完整映射表见 [核心设计 §11](../design.md#11-prost-生成式-protobuf-profile)，组合性理由见 [Option 专题](option-semantics.md)。

| 核对项 | 源码依据 |
|---|---|
| Go protobuf presence、默认值与 wrapper | [pb/type.go](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/pb/type.go#L255) |
| Java protobuf presence | [ProtoMessageValue](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/common/src/main/java/dev/cel/common/values/ProtoMessageValue.java#L59) |
| Java 字段值到 CEL 的转换 | [ProtoCelValueConverter](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/common/src/main/java/dev/cel/common/values/ProtoCelValueConverter.java#L117) |
| C++ wrapper unboxing 的历史默认值 | [RuntimeOptions](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/runtime_options.h#L130) |
| cel-cxx 表达式与程序化读取配置 | [Env runtime 配置](../../../../src/env/inner.rs#L268)、[FFI 字段访问](../../../../crates/cel-cxx-ffi/include/values.h#L384) |

基线 cel-cxx 没有为表达式启用 wrapper null unboxing，而程序化字段读取使用 kUnsetNull。旧 wrapper 测试只覆盖后者，不能证明二者已经一致。核心设计统一配置；这是需要实现的调整，不是已具备能力。

NullValue 的普通 enum 字段、独立 Rust WKT carrier、JSON Value null kind 也属于不同上下文。具体选择和当前 C++ 的字段类型/返回值差异集中在 [核心设计 §11.7](../design.md#117-enum)，不在这里再维护一份枚举规则。

## C++ 类型、provider 与值接口

### C++ struct variants 与 Rust 表示

cel-cpp 的 StructValueVariant 确实包含 CustomStructValue、ParsedMessageValue、LegacyStructValue 三项；这描述 C++ 库支持的实现，不要求 Rust API 原样镜像。以下职责来自基线源码；Rust 统一 Struct trait 与 adapter 边界是本库设计决定。

| cel-cpp 实现 | 源码合同 | 对 Rust 接入的含义 |
|---|---|---|
| CustomStructValue | 通过 interface/dispatcher 提供类型、get/has、比较与生命周期等操作 | 通用扩展接口，不限制真实对象属于 C++；本库 Rust Native 与未来 upb Message 都可实现这些回调 |
| ParsedMessageValue | 包装 const google::protobuf::Message 与 arena，提供反射访问；不限于从 bytes 解析而来的对象 | 对应 future Message 的 CPP full 路径，但必须验证 owner、注册与具体类型 |
| LegacyStructValue | 兼容旧 google::api::expr::runtime::CelValue 中的 message 表示与 LegacyTypeInfoApis | cel-cxx 不支持；不增加 Rust Legacy variant，也不提供隐式转换通路 |

来源：[三项 variant 定义](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/struct_value_variant.h)、[Custom 接口](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/custom_struct_value.h#L179)、[ParsedMessage 存储](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.h#L59)、[Legacy 兼容对象](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/legacy_struct_value.h#L56)。

Rust 侧统一采用 `StructValue = Box<dyn Struct>`。prost 的实际 Rust M 使用 Native adapter 和 Protobuf profile，future CPP full/upb 使用对应 kernel/view adapter；这些是实现差异，不要求公开 Native/Message enum。任意外部 C++ CustomStruct 不在接入范围内；统一 trait 不改变注册和来源校验。

ParsedMessage 的默认实例还具有 google.protobuf.Empty 的特殊空状态，不能直接解引用 message 指针。已验证的内建 Empty 使用专用 schema adapter，没有真实 Rust () 的对象恢复能力。共同接口见 [核心 §5.1](../design.md#51-公有形态)，实现矩阵见 [§5.7](../design.md#57-统一接口与后端边界)，转换分派见 [§9.5](../design.md#95-值转换-context)。这些接入合同尚未成为生产 bridge。

### ParsedMessage 的复制与 Rust 返回边界

ParsedMessageValue 的普通复制构造/赋值是 default，成员为 message 指针与管理 arena 指针；这类复制不会复制消息内容。显式 Clone(target_arena) 会先比较管理 arena：相同时直接返回当前句柄，不同时调用 Message::New(target_arena) 与 CopyFrom，生成目标 arena 中的消息副本。因此不能将“ParsedMessage 可复制”概括成“每次值复制都深拷贝”，也不能由其内部共享推导 Rust 返回普通 T 时不应复制。

| 操作 | 基线行为 |
|---|---|
| ParsedMessageValue 的 copy constructor / copy assignment | 复制 message/arena 指针 |
| Clone 到相同 arena | 返回共享原 message 的句柄 |
| Clone 到不同 arena | New + CopyFrom，消息内容进入目标 arena |

依据：[复制构造与赋值](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.h#L83)、[Clone 实现](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.cc#L160)、[Value::Clone 的 arena 寿命合同](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/value.h#L468)。本项为源码检查，没有新增运行探针。

Rust 内部用 Arc 保活与输出普通 T 可以同时成立：已有真实 &T 且支持 Clone 时，owned decode 可以复制它；读取真实 child 时复制 Child 即可，不需要抽取 Arc<Child>。消费真正的 Arc<T> 时，标准库还提供 unwrap_or_clone，在不能唯一取出时退回 Clone；这不是持久 projection 的通用恢复方法，也不应使返回的成功与否取决于引用计数。Clone 本身可能保留宿主内部共享字段，不保证递归复制所有对象。[Clone 合同](https://doc.rust-lang.org/std/clone/trait.Clone.html)、[Arc::unwrap_or_clone](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.unwrap_or_clone)

因此普通 T 的输出应由本库解码入口封装。最终设计默认要求 Native T: Clone，采用 Box 拥有实际 T，不再以支持非 Clone 投影为前提；借用时复制，消费时移出。规范边界见 [核心 §5.3](../design.md#53-downcast-与物化)。

### 消费型输入输出与现有桥接

Box 可以消费后移出 T；Arc::new(T) 同样直接移入 T，取回独占的真实 Arc<T> 时也可用 try_unwrap 移出。两者均不能从一个共享 &Value 偷走拥有者的 T。由此必须区分“解码结果是 owned T”和“转换消费了输入 Value”；只有后者可能保留完整 move 路径。[Box 所有权](https://doc.rust-lang.org/std/boxed/index.html)、[Arc::new](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.new)、[Arc::try_unwrap](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.try_unwrap)

基线 cel-cxx 存在以下实际复制/借用边界，不能仅将 Native 容器换成 Box 后便声称零次宿主 Clone：

| 位置 | 已核对行为 |
|---|---|
| [Program::evaluate](../../../../src/program/mod.rs#L283) / [eval_sync](../../../../src/program/inner.rs#L69) | 泛型 activation 虽按值传入，但后续只向桥接提供其变量/函数的借用；A 本身也可以是 &Activation |
| [FfiActivationImpl::new](../../../../src/ffi/activation.rs#L120) | 收集变量时调用 binding.clone()；按值传入 activation 并没有自动转移其绑定 |
| [value_from_rust](../../../../src/ffi/values.rs#L3) / [Opaque 输入](../../../../src/ffi/values.rs#L280) | 输入是 &Value；Opaque 路径 clone 被装箱值，再交给 C++ wrapper |
| [Opaque 输出 downcast](../../../../crates/cel-cxx-ffi/src/common/values.rs#L846) | 从 &self 克隆 ffi payload，再做类型恢复；不是从 C++ holder 消费取回原 Box |
| [FromValue](../../../../src/values/traits.rs#L181) | 以 &Value 为输入；Output 是 T 也不表示消费了输入 |

cel-cpp 的 [Activation::FindVariable](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/activation.cc#L39) 复制已绑定的 cel::Value；[CustomStructValue 的普通复制](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/custom_struct_value.h#L300) 复制表示本身，不要求每次复制 Rust 宿主。变量能够重复读取，输出还可能多次引用同一宿主；拥有一个 C++ Value 外壳不等于独占其宿主。

所以移动通路需要消费型 activation/值转换与求值后的受控 owner 恢复。读取期间维持借用；恢复前结束相关 C++ 视图的使用；对保留输入、重复输出或没有消费式 getter 的子字段使用明确的复制/共享规则。它可以利用 Box，也可以利用能完整恢复的独占 Arc，不能将任一指针类型等同于整条链必然零复制或必然复制。核心 §5.3–5.4 规定 Box 方案的目标合同；本节为源码核对，尚未实现或验证 Native move bridge，也没有新增运行基准。

### BasicStruct 是类型，CustomStruct 承载值

`BasicStructType` 保存名义类型；`BasicStructTypeField` 描述 name、number、type；get/has/Equal/Clone 属于 struct 值接口。存在编号接口不意味着普通 Rust 字段必须具备 protobuf wire tag。

| 路径 | 基线行为与接入含义 | 源码 |
|---|---|---|
| 普通 select / has | 按名称访问 | [select_step](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/select_step.cc#L120) |
| struct literal | 按字段名赋值，条件条目由 evaluator 解包/跳过 | [create_struct_step](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/create_struct_step.cc#L92) |
| 编号接口 | 可返回 Unimplemented；number=0 可表达无编号 | [BasicStructTypeField](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type.h#L1078)、[dispatcher](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/custom_struct_value.cc#L276) |
| select optimization | Qualify 的 Unimplemented 可回退到名称；错误通路与普通 select 并非完全一致 | [fallback](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/extensions/select_optimization.cc#L336)、[测试](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/extensions/select_optimization_test.cc#L226) |
| protobuf activation 绑定 | 依赖 descriptor，不能直接作为通用 Native 绑定 | [BindProtoToActivation](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/bind_proto_to_activation.cc#L36) |

这支持普通 Native 采用名称访问，但不证明 select optimization 可直接开启。首版禁用与拒绝策略见核心设计 §10。

### Opaque 的 Box dyn 与统一 Struct

当前 [OpaqueValue](../../../../src/values/mod.rs#L248) 是 Box<dyn Opaque>；[Opaque/TypedOpaque](../../../../src/values/opaque.rs#L105) 使用 DynClone 与具体类型 Clone，后者还要求宿主 Debug/Display/PartialEq。Rust→C++ 的 [opaque FFI](../../../../src/ffi/values.rs#L281) 调用 value.clone()。这提供动态复制，但不是消费式 move bridge。

Cargo.lock 固定的 dyn-clone 1.0.20 对 Clone 类型调用其 Clone，并提供 clone_trait_object! 为 Box<dyn Trait> 实现 Clone。具体 Clone 可能复制业务数据，也可能只复制内部共享句柄。[dyn-clone 文档](https://docs.rs/dyn-clone/1.0.20/dyn_clone/)

本设计同时沿用现有接口分层：[TypedOpaque](../../../../src/values/opaque.rs#L152) 提供静态类型与 Clone/PartialEq 约束，库通过 [blanket impl](../../../../src/values/opaque.rs#L209) 提供动态 Opaque。对应地，TypedStruct 提供静态 definition 和具体类型操作，`impl<T: TypedStruct> Struct for T` 提供动态转发；derive 仍叫 Struct，生成 TypedStruct 实现。纯 view/kernel adapter 可只实现 Struct。StructValue 是 Box<dyn Struct> 别名，直接拥有实际 T；泛型 downcast 放在 impl dyn Struct，Clone/PartialEq 等采用 dyn Struct 与 Box 的合法实现位置。[Rust 类型别名](https://doc.rust-lang.org/reference/items/type-aliases.html)、[dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)

Struct 不能直接继承要求 Self: Sized 的 Clone，也不能把带类型参数的动态方法放入 vtable；静态定义、Clone 与 PartialEq 留在 TypedStruct。消费型恢复使用 self: Box<Self> 的对象安全内部方法，由泛型辅助方法先检查 TypeId；失败保留原 box。内部 blanket 擦除实际 Self，因此 T 可恢复为 T，VariantView<E, Case> 则不能恢复为它内部的 E。该内部 trait 不封闭 Struct；下游 TypedStruct 的动态接口由库提供，纯动态实现也可以单独存在，Env 另行验证定义与实际实现身份。

Box 中也可以装持有 Arc 的 adapter，但首版默认直接拥有 Native T，允许移出；不把公有 Clone 固定为共享同一 owner。也不照搬 Opaque 的 Eq、宿主 Debug/Display 或可变 downcast。完整接口与所有权取舍见 [核心 §5](../design.md#5-rust-值表示与所有权)，模型验证见 [Struct/TypedStruct 验证记录](validation.md#structtypedstruct-分层模型)；没有分派性能基准或已完成 Native bridge 的承诺。

### unit、空对象与 protobuf Empty

Rust `()` 是零字段 tuple 类型；`struct S;` 是另一个具名类型。Rust enum variant 也不是独立的 Rust 类型，因此“完整 E 保留身份，无 payload 选择字段只返回 null 标记”不等于把 unit struct 转为 null。cel-cxx 现有 [null 转换](../../../../src/values/impls/null.rs#L4) 只覆盖裸 ()；扩大到零字段对象不是现有代码所隐含的规则。

| 已核对事实 | 设计含义 | 依据 |
|---|---|---|
| CEL 明确允许空 message literal M{}，每种普通 message 有其自己的类型 | 零字段仍可保留名义身份；不是 map {} 或 null | [CEL Aggregate Values](https://github.com/cel-expr/cel-spec/blob/master/doc/langdef.md#aggregate-values) |
| protobuf 的 messageBody 允许没有字段；google.protobuf.Empty 是标准空 message | 普通空 message 与 Empty 都合法；protobuf JSON 的 {} 表示也不把 CEL 类型变成 map | [Proto3 message 语法](https://protobuf.dev/reference/protobuf/proto3-spec/#message_definition)、[Empty 定义](https://protobuf.dev/reference/protobuf/google.protobuf/#empty) |
| BasicStructType 只保存类型名；字段通过独立 introspector 查询 | 类型存在与字段表为空是两个状态，不需要伪造一个字段才可注册 | [BasicStructType](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/types/basic_struct_type.h#L53) |
| prost-build 0.14.1 的 stock extern_paths 将 Empty 映射为 () | protobuf adapter 必须按 descriptor 恢复 Empty 身份，不能调用普通 () 的 null 转换 | [extern_paths](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/extern_paths.rs#L46) |

普通空 message 字段的 unset 与 set-empty 可读取到相等的空 message，但父字段 has 不同；同样适用于 Empty 与 oneof 的空 message 成员。Rust profile 的 Option<()> 则保留 optional<null_type>，不能与 protobuf-context Option<()> 混用。规范性矩阵见 [核心 §4.11](../design.md#411-unitnull-与零字段对象)，实际 checker/runtime 覆盖与限制见 [验证记录](validation.md#cel-零字段对象与-empty-验证)。

### provider 必须覆盖完整调用链

| 已核对缺口 | 为什么仅增加一个 adapter 不够 | 源码 |
|---|---|---|
| 默认 checker provider 的字段回退 | Native skip/miss 可能被同名 protobuf descriptor 再暴露 | [type_check_env](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_check_env.cc#L98) |
| runtime 类型与构造查询 | planner 实际使用固定 runtime provider | [runtime_type_provider](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/internal/runtime_type_provider.cc#L45) |
| BasicStruct → TypeSpec | 当前反向转换仍限制 BasicStruct | [type_spec_resolver](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type_spec_resolver.cc#L291) |
| Rust Struct 类型与 AST 返回类型恢复 | 基线代码按 descriptor 查找 | [Rust type glue](../../../../src/ffi/types.rs#L16)、[AST helper](../../../../crates/cel-cxx-ffi/include/ast.h#L100) |
| enum 常量 | checker 声明与 runtime resolver 需要一致 | [resolver](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/resolver.cc#L103) |
| Any 解包 | 可直接查 C++ pool，不自动经过 Native registry | [well_known_types](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/internal/well_known_types.cc#L1968) |

这些缺口要求 checker/runtime 使用一致的组合规则：普通 struct 先 Native，类型名未命中才查 pool；Native 的字段/构造 miss 不继续回退。名称优先级还不能改变 ParsedMessage 的实际布局：pool 父消息若可达已被 Native 覆盖的子消息，必须按核心 §9.2.3 拒绝该 pool 闭包，不能仅把字段类型改写为 BasicStruct。冻结 Rust registry 不意味着删除 protobuf provider；独立且准入的 pool 类型继续可用，实现责任集中在核心设计 §9、§15。

2026-10-10 按本地 cel-cpp 基线复核 RuntimeEnv 的耦合：TypeRegistry 直接保存 final RuntimeTypeProvider，RegisterType 只接受 OpaqueType，当前公开 API 不足以注入自定义 Struct reflector；但其使用方最终依赖 TypeReflector 接口。FlatExprBuilder 的 modern 分支通过 GetRuntimeTypeProvider 取接口，而 RuntimeImpl/program 使用 GetComposedTypeProvider；因此可以保留 RuntimeEnv，通过统一委派修改这两个入口共同指向的 provider。这是有待实现的定向 cel-cpp 改动，不是已有 setter，也未证明补丁已通过集成测试。

具体依据：[TypeRegistry 的存储与两个入口](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/type_registry.h#L102)、[planner 选择 provider](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/flat_expr_builder.cc#L2792)、[RuntimeEnv 的资源与 registry](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/internal/runtime_env.h#L42)、[program 的 provider](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/internal/runtime_impl.cc#L69)。RuntimeEnv 的 descriptor pool 同时保留 WKT reflection 和普通 protobuf fallback；无需替换 RuntimeEnv，也无需把全部 descriptor 转录进 Rust registry。

常量与扩展不能遗漏：Resolver 通过 [GetEnumValueTable](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/resolver.cc#L67) 取得 enum 缓存；[OptionalTypes](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/optional_types.cc#L455) 仍向 TypeRegistry 注册 OptionalType，并使用 RuntimeImpl 的 planner 配置。Opaque 保留独立类型头；enum 保留缓存并补上按名查询，才能让 pool 中未预装进缓存的常量在 parse-only 路径也可见。

细化接口时还核对了这些实际旁路和返回合同：

| 源码事实 | 对设计的约束 |
|---|---|
| checker 在 Build 时应用库配置；context declaration 直接查询 proto introspector 的字段列表 | 唯一 provider 模式必须检查全部最终配置；仅替换普通 lookup 不足以支持 context/type mask。[配置与 context 路径](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_checker_builder_impl.cc#L130) |
| TypeIntrospector 没有构造能力方法；checker 对 struct literal 主要检查类型类别与字段 | 只读类型需要额外的静态能力遍历，不能假定 FindType 已表达 constructor 可用性。[literal 检查](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_checker_impl.cc#L674) |
| create-struct 在求值时取得 builder，nullptr 转成 CEL Error | parse-only 的只读 literal 可以保持值错误，不必把“没有 constructor”提升为基础设施失败。[stack evaluator](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/create_struct_step.cc#L69) |
| planner 的 ResolveCreateStructFields 已查询完整类型名和每个 literal 字段 | parse-only 的未知类型/字段在创建 Program 时失败，不能把全部 literal 问题都描述为求值 Error。[规划检查](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/flat_expr_builder.cc#L2046) |
| RuntimeTypeProvider 先查 WKT，再查 pool，最后查 Opaque 注册表；builder 直接用 pool | Native 要在普通 protobuf 之前选定来源，同时保留 Opaque/WKT；不能仅改 FindType 而让字段和构造继续直接查 pool。[runtime provider](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/internal/runtime_type_provider.cc#L45) |
| DescriptorPoolTypeIntrospector 的 enum 常量与普通 enum 字段可能带 EnumType | 组合层须把常量及嵌套字段类型规范化为 int；普通 message 边保持实际 pool 表示，若目标名选中 Native 则拒绝父闭包，不改写 carrier。[pool 查询](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/descriptor_pool_type_introspector.cc#L111)、[字段类型](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type.cc#L632) |
| Resolver::FindConstant 只查 enum 缓存和类型，返回 optional，FindType 的非 OK 被忽略 | 缓存 miss 后补 FindEnumConstant，并以 StatusOr 在两个调用点传播 enum/type 错误。[resolver](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/resolver.cc#L105) |
| cel-cxx 的 FDS helper 已收集顶层/嵌套 enum，pool 有 generated underlay | 保留 FDS 收集但统一校验冲突；underlay 中未枚举的 enum 可直接查询，不以缓存是否含有它决定可见性。[enum helper](../../../../crates/cel-cxx-ffi/include/runtime.h#L568)、[pool owner](../../../../crates/cel-cxx-ffi/include/protobuf.h#L30) |
| TypeSpec resolver 先查全局 WKT；未知 abstract name 回退为 Opaque，已命中的 abstract type 直接返回类型头 | 恢复入口需要 provider-only 模式，并实际恢复传入的类型参数；仅换 introspector 不足以落实 authority。[名称入口](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type_spec_resolver.cc#L39)、[abstract 分支](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type_spec_resolver.cc#L173) |
| planner 的 MaybeResolveType 也调用 TypeSpec resolver，当前忽略解析失败 | strict 模式必须传到实际 planner，并传播错误；只修 Ast::return_type 不足够。[planner 调用](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/compiler/flat_expr_builder.cc#L2344) |
| ListFields 声明区分未知与已知空列表；当前 WKT helper 的 return {} 是 disengaged optional | 已编译表必须显式返回 Some(empty)，不能从空字段数反推类型未知。[接口合同](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type_introspector.h#L77)、[helper 实现](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/type_introspector.cc#L266) |
| Rust FfiTypeReflector 的 builder 与 metadata 共用一个 'a，arena 参数却是独立短借用；FfiValueBuilder 也没有 setter 的 CEL Error 分支 | 新通路使用拥有 metadata 的 provider handle，以及不保存 arena 借用的 Rust builder state；不能仅给旧 wrapper 标 Send/Sync 就宣称合同成立。[reflector](../../../../crates/cel-cxx-ffi/src/common/type_provider.rs#L198)、[builder](../../../../crates/cel-cxx-ffi/src/common/values.rs#L1016) |

据此，[核心设计 §9.2](../design.md#92-provider-安装) 使用 Native 优先、pool 回退的共享 EnvTypeProvider；checker 的 SetTypeProvider 与 runtime 的 InstallTypeProvider 安装同一组合实例。Opaque/enum 保留独立处理，enum 缓存与 provider 查询结合；构造能力按 Native/WKT/pool 来源检查。[§9.4](../design.md#94-builder-ffi) 的 owned bridge 适用于 Native/WKT，普通 pool builder 保留 C++ 生命周期。这里记录的是源码依据与设计结论，尚未编译这些新增接口或运行完整 Native 集成。

2026-10-11 的实际 C++ 反例还确认了三个不能由 metadata 修补代替的边界：

- ParsedMessage 直接按自身 descriptor 读取字段；pool Parent 返回的 Child 不会因为 checker 投影成 Native 就取得 Native getter/skip 规则。来源见 [ParsedMessage 字段路径](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.cc)。因此 pool 准入要在产生值之前验证实际依赖闭包。
- ParsedMessage::Equal 对 Custom 使用字段 helper，而 Custom 可以覆写 Equal，导致两个顺序采用不同语义。来源见 [Parsed Equal](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.cc)、[Custom Equal](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/custom_struct_value.cc)。核心 §12.2 让 Parsed/Custom 两种顺序都调用 Custom comparator；默认结构性 helper 保留为未覆写 comparator 的实现。
- 普通 NullValue 字段的 Value::Enum/repeated/map accessor 会返回 null，setter 还可能把 int 强制写成 0。来源见 [值适配](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/value.cc)、[构造适配](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/struct_value_builder.cc)。核心 §11.7 将这些普通字段路径列为首版通用 C++ 修补；JSON WKT null kind 仍走专用 adapter。

TypeSpec 名称恢复也不能证明旧 checked AST 的字段/函数签名仍有效；[TypeChecker::Check](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/type_checker.h) 接受 Ast，核心 §9.2.7 据此要求导入先丢弃旧 type/reference map，再在目标 Env 重新检查。反例、实际输出与证据边界见 [组合 provider 与值表示验证](validation.md#组合-provider-与值表示验证)；上述修补仍是待实现合同。

### cel-cxx 的类型描述与注册分工

现有 `TypedValue` 只有 `value_type()`；变量 registry 保存这个返回值，函数也通过参数/返回的 ValueType 描述签名。实际 C++ 类型注册由 Env 构建流程完成，例如基线的 Opaque 注册在 `src/env/inner.rs` 中。当前 StructType 只保存 CEL 全名，并没有可供泛型声明取出的 Native definition。

这支持核心设计 §4 保持类型描述与环境注册分离：`<T as TypedStruct>::definition()` 提供无实例定义，EnvBuilder 显式接收根并展开字段依赖，泛型变量/函数继续通过 TypedValue 推导签名。Struct/TypedStruct 采用现有 Opaque/TypedOpaque 的动态/具体类型分工；不另设 NativeType 名称，不给所有 TypedValue 增加注册方法，也不引入全局宿主类型表。

这些具名对象统一使用已有的 ValueType::Struct(StructType)。StructDefinition 是适配 metadata，C++ NativeTypeId 用于实际 C++ 表示身份，都不是新增 CEL 类型类别。

依据：[TypedValue](../../../../src/values/traits.rs#L42)、[变量声明](../../../../src/variable/registry.rs#L140)、[函数签名](../../../../src/function/mod.rs#L853)、[Env 中的 Opaque 注册](../../../../src/env/inner.rs#L400)、[StructType](../../../../src/types/mod.rs#L603)。这些是现有职责划分的源码依据，不表示目标 Native 注册接口已经实现。

## Prost 生成器依据

FDS 提供 protobuf 语义，生成后的 Rust AST/prost attrs 提供真实布局。构建 helper 组合 stock prost-build 的 load_fds/compile_fds 与属性注入即可；ServiceGenerator 面向 RPC，不是 message adapter 的替代品。

| 需要核对的行为 | 依据 |
|---|---|
| 生成前注入属性 | [prost-build Config](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/config.rs#L876) |
| 字段布局与命名 | [code_generator](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/code_generator.rs#L389) |
| extern_path 跳过本地类型生成 | [extern 路径](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/code_generator.rs#L154) |
| wrapper/Empty 等 carrier 映射 | [extern_paths](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/extern_paths.rs#L35) |
| required 编码与 unknown fields | [scalar derive](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-derive/src/field/scalar.rs#L129)、[Message derive](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-derive/src/lib.rs#L194) |
| cel-cxx 旧 ProstValue 只生成 codec | [现有宏](../../../../crates/cel-cxx-macros/src/prost_value.rs#L91) |

无运行时反射不等于 prost 只能走序列化；生成访问器可以直接读取真实对象。对 extern 类型，仍需解决 trait 所属 crate 与配套 adapter，不能由下游随意实现外部 trait。首版的确切 schema/extern/WKT 范围见核心设计 §11。

prost 会为 protobuf enum 生成无 payload 的 Rust enum，但 message 字段实际保存 raw i32；便捷 getter 遇到未知数字会返回默认 variant，因此 CEL 字段读取不能通过它。oneof 则生成带 payload 的 Rust enum，并在 parent 中保存 Option<OneofEnum>；其中 protobuf enum 成员的 payload 仍是 i32。enum 在 message 内嵌套声明只影响 protobuf 全名和 Rust module，不等于 oneof。

`Enumeration` 与 `Oneof` 是 prost 导出的 derive 宏，并非通用 trait。以上两类 enum 的默认适配需要生成器区分 schema 角色，不能用 `T: prost::Enumeration` blanket impl，或默认套用普通 Struct derive 的映射。本地 oneof enum 额外派生普通对象视图不改变其 protobuf parent 的 dispatch；数字 enum 的 int canonical 规则则不能在同一 E 上叠加第二套。核心设计的 [完整对照例子](../design.md#145-protobuf-enum嵌套声明与-oneof) 同时覆盖数字 enum、oneof 和 oneof 中的 enum 成员。

依据：[prost enum 字段与 getter](https://github.com/tokio-rs/prost/blob/v0.14.1/README.md#enumerations)、[oneof 存储](https://github.com/tokio-rs/prost/blob/v0.14.1/README.md#oneof-fields)、[oneof 代码生成](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/code_generator.rs#L588)、[enum payload 类型解析](https://github.com/tokio-rs/prost/blob/v0.14.1/prost-build/src/code_generator.rs#L939)、[derive 宏导出](https://github.com/tokio-rs/prost/blob/v0.14.1/src/lib.rs#L32)。

## 构造、比较与 owner 的对照

Go protobuf 从空 message 逐字段赋值；Java CEL 使用 protobuf builder 并调用 build；C++ ValueBuilder 也从空 message 开始。Go/C++ 能保留 partial message，Java build 的完整性检查更严格。stock prost 的 required 字段无法表达 unset，因此核心设计使用字段槽位并在 Build 检查；不能照搬“先 Default 整个 Rust M”来模拟空 protobuf message。

参考：[Go NewValue](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/provider.go#L461)、[Java value provider](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/common/src/main/java/dev/cel/common/values/ProtoMessageValueProvider.java#L49)、[C++ builder](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/struct_value_builder.cc#L884)。实际结果见 [required 验证](validation.md#required-构造与默认对象)。

普通宿主比较与 protobuf message 比较也应分开观察。Go native 使用 reflect.DeepEqual；Go protobuf 使用 CEL 自己的 pb.Equal；Java 有 ProtoEquality 适配 CEL protobuf 规则；C++ message 使用 MessageDifferencer。它们并不推出“普通 Rust 必须按 CEL getter 比较”或“所有 prost PartialEq 都不可复用”。

参考：[Go pb.Equal](https://github.com/google/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/common/types/pb/equal.go)、[Java ProtoEquality](https://github.com/google/cel-java/blob/798c0be3a7beef6816344e0323294067bc1d8b1c/common/src/main/java/dev/cel/common/internal/ProtoEquality.java)、[CEL protobuf equality](https://github.com/cel-expr/cel-spec/blob/master/doc/langdef.md#protocol-buffers)。PartialEq/signed-zero 的实测范围见 [相等验证](validation.md#protobuf-相等与-partialeq)。

Go/Java 的 GC 引用可以保活 child；C++ 则将 owner 与 view 分开，arena 是否存活仍依赖调用方合同。Rust 的具体承载选择见核心设计 §5；旧投影 parent 链的析构反例见 [owner 验证](validation.md#owner-与投影)。参考：[C++ Owner/Owned](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/memory.h)、[ParsedMessage Clone](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/common/values/parsed_message_value.cc#L160)。

## 未采用方案及原因

| 方案 | 取舍 |
|---|---|
| 把所有宿主对象投影成 map | 简化只读输入，但失去具名字段检查、struct 构造及直接宿主恢复 |
| 为普通 Rust struct 生成 protobuf descriptor/消息 | 继承 protobuf 的字段约束并复制数据；适用于明确选择 protobuf 模型的业务，不作普通 Native 默认 |
| 仅用 Opaque + 访问函数 | 适合专用操作，没有所需的通用字段/has/type provider 合同；现有 Opaque 继续保留 |
| 默认按暴露字段执行可失败 CEL 相等 | 将 getter 转换错误引入整对象比较并要求扩展 C++ 结果通路；当前选择宿主/Protobuf bool 比较 |
| 先 Default 完整对象再赋值 | required DAG 可能提前大量分配，且模糊缺失与显式提供；当前使用字段槽位 |
| projection 强持上一个 projection | 累积 get/Drop 链；首版改为拥有型字段结果，不保存历史 parent 链 |
| 持久 bytes 作为所有 StructValue 的表示 | 丢失真实对象身份并付出常态 codec 成本；仅保留明确 codec 边界 |

这里记录取舍，不保留另一份旧 API 草图。官方 Rust protobuf 的 kernel、view 与版本问题独立见 [v4 状态报告](protobuf-v4-status.md)。
