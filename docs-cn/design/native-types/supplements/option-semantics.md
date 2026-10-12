# Option 与值转换：设计理由与边界

定位：[核心设计](../design.md) 的补充说明，解释 std Option、CEL optional、字段 presence、null 及 Box/Arc 的组合关系。具体 API 与首版行为以核心设计 §6–8、§10–11 为准；本文不再维护第二份接口草图或验收清单。

调研基线：2026-10-09；按 2026-10-10 的两类 profile 与字段槽位 builder 整理。[返回设计目录](../README.md) · [语言/接口验证](validation.md#rust-类型与转换探针)

## 为什么统一普通 Option 值语义

Rust 字段、变量和函数可以使用同一个 `Option<i64>`：

```rust
struct User { age: Option<i64> }
fn next_age(age: Option<i64>) -> Option<i64> { age.map(|n| n + 1) }
```

如果普通字段把最外层 Option 剥成 int，而独立变量和函数仍是 optional<int>，`next_age(age)` 与 `next_age(user.age)` 就不能直接组合，字段 None 还需要另一个默认读取规则。统一为 CEL optional 能保留类型和 Some/None 状态。

`τ(T)` 表示 Rust T 的 CEL 类型。映射适用于已经具备对应适配能力的 T；静态类型一致不保证所有所有权形式都能无复制恢复。

| 位置 | 采用的解释 | 要避免的混淆 |
|---|---|---|
| 变量 | 已绑定的 optional 值 | None 不等于没绑定变量 |
| 表达式结果 | 严格解码实际 optional | null、类型错误、求值错误不转为 None |
| 函数参数 | 一个必须传入的 optional 参数 | 可选内容不等于可省略实参 |
| 函数返回 | None 是成功的 optional.none | 不等于没有返回值或 Err |
| 普通 Rust struct/record 字段 | 字段存在，内容为 CEL optional | has 与 hasValue 回答不同问题 |
| prost 生成字段 | Option 可以保存 schema presence | 不能用普通值 trait 覆盖 protobuf 类型/default |
| 未来 v4 message 字段 | 按底层 message/schema 访问 | `_opt()` 返回 Option 不改变原字段 CEL 类型 |

CEL optional、字段 presence 与 null 是三种独立概念。protobuf optional scalar 的字段类型仍是 scalar；对它执行 optional select 才得到 optional 值。跨语言依据见 [实现对照](native-interop.md#各实现的实际约定)。

## 值状态与错误必须分开

| 输入/结果 | 含义 |
|---|---|
| optional.none | 已知没有 optional 内容 |
| optional.of(null) | 有内容，内容恰好是 null |
| optional.of(optional.none()) | 外层有内容，内层没有内容 |
| 已声明但未绑定的变量 | 实际读取时报求值错误；短路未读到时可以不触发 |
| Error / Unknown | 引擎错误/未知状态，不是成功的 None |

`has(x)` 不能查询变量绑定；它是字段选择宏。Option 参数接收 None 时函数照常执行，但 CEL 仍须传这个实参。

函数返回 `Result<Option<T>, E>` 的外层 Result 是错误通道：

```text
Ok(Some(v)) → optional.of(v)
Ok(None)    → optional.none
Err(e)      → 错误通道
```

`Option<Result<T, E>>` 不是同一合同。标准 Rust T、null 也不在 decoder 中自动提升为 Some/None，否则 checker、重载与 runtime 的规则会不一致。

源码：[activation](../../../../src/ffi/activation.rs#L136)、[标识符求值](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/ident_step.cc#L38)、[has 宏](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/parser/macro.cc#L73)、[函数返回转换](../../../../src/values/traits.rs#L654)。

## 普通字段为何既 present 又可能为 None

普通 Rust struct/record 已经初始化了每个暴露字段。`age: None` 是该字段的内容，不是对象缺少字段。因此：

```cel
has(user.age)                 // true
user.age.hasValue()           // false（age 为 None）
next_age(user.age)            // 传入 None
user.?age                    // optional.of(optional.none())
```

最后一行的外层 optional 表示字段 presence，内层是字段值。压平它会改变类型层次，也与 C++ checker 的 OptionalType(field_type) 不一致。目标对象/attribute 的 Error/Unknown 仍由引擎处理；不能把整个 has 表达式编译成常量 true。

None 不需要构造一个默认 Child，因此 `Option<Child>` 的只读访问不要求 Child: Default。Some 的拥有型读取通过对应字段转换取得独立值，Native child 使用 Clone。

enum 的 variant 选择字段按当前分支判断 presence，直接暴露 payload 类型；无 payload 分支也有 null 标记字段。它与实际 struct 字段不同，具体区别见下面的决策说明；规范统一维护在 [核心设计 §4.10](../design.md#410-presence-与-optional-的固定决策)。

### 条件初始化不是字段 None

```cel
User{age: optional.none()}                 // 已提供，内容为 None
User{?age: optional.none()}                // 省略条目；需要显式字段 default，否则缺字段错误
User{?age: optional.of(optional.none())}   // 已提供，内容为 None
```

字段槽位 builder 区分 Unset 与 Set(None)。只有显式声明的逐字段 default/factory 能处理省略条目；不存在“先创建整个 User::default()”的隐含步骤。构造成功后的 has 不追踪历史赋值条目。

源码：[optional select 类型](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_checker_impl.cc#L1210)、[select 求值](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/select_step.cc#L165)、[条件初始化](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/create_struct_step.cc#L102)。

## enum 的 presence 与 optional 决策

普通 Native enum 的分支在 Rust 值中本来就由 variant 决定，不需要额外构造一层 Option。CEL 已有独立的字段 presence 接口，因此 `e.C` 可以直接是 E.C，`has(e.C)` 检查是否选中，`e.?C` 则由调用方明确请求 optional 结果。这保留了 e.C.x 的自然访问，也不改变 payload 的静态类型。

无 payload 的 unit/empty tuple/empty record variant，以及 payload 全 skip 的 case，同样生成选择字段，选中时返回 null 标记。因而所有 variant 都能用 has 查询，无需另记“某些 case 只能比较 kind”。标记的 some(null) 与未选中的 none 分开；它不把整个 E 变成 null，也不允许隐藏字段绕过构造 default/factory。

`Maybe(None)` 是“分支存在，payload 的 Option 无内容”：has(e.Maybe)=true、e.Maybe=none、e.?Maybe=some(none)。另一分支则 has=false、普通读取错误、optional 选择 none。selected 0/false/空集合/null 也始终 present。不能用 Optional.OfNonZeroValue 或 payload 的 hasValue 推导 variant presence。

实际 CEL 链式语义需要进一步区分两个动作：

- e.?C.x 与 e.?C.?x 都继续安全选择，遇到 absent 返回 none；e.C.?x 的第一个选择已经是普通读取，后面的 ? 不捕获前面的错误。
- 若 m 本身是 optional<E>，m.C 已触发 optional receiver 的链式选择，与 m.?C 同样得到 optional<E.C>。m=None 与 Some(E) 的 C 未选中可以汇合为 none，但成功 payload 内部的 Option 仍保留，因此 m.Maybe 是 optional<optional<T>>。
- has(e.C.x) 先求值 e.C，只检测最后的 x；has(e.?C) 不是 has 接受的字段选择 AST，应使用 has(e.C) 或 e.?C.hasValue()。

这些规则来自 optional library 的选择语义，不是 Native 专有的自动解包。依据：[CEL optional 语法说明](https://github.com/cel-expr/cel-go/blob/12ea74fdf5f4806087b979ee05d6e38ab35c151f/cel/library.go)、[C++ optional receiver 与 has](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/eval/eval/select_step.cc#L271)、[C++ 静态类型](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_checker_impl.cc#L1172)、[C++ 显式 optional 选择](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/runtime/optional_types.cc#L119)。真实引擎探针见 [验证记录](validation.md#cel-presenceoptional-组合验证)。

这项决策限定于普通 Struct derive 映射；std Option、函数返回 Result、protobuf 数字 enum、protobuf parent 中的 oneof，以及明确选择的 scalar/opaque adapter 保留各自合同。尤其 protobuf oneof 允许全 absent，未选中成员读取使用 schema default；普通合法 E 恰好选中一个 variant，缺失整个 E 要用外层 Option<E>。

## protobuf 上下文为何不同

| 同为 Rust Option 的情况 | None 的字段读取 |
|---|---|
| 普通 Rust `Option<i64>` | CEL optional.none |
| prost protobuf optional int64 | protobuf schema default，类型仍是 int |
| prost 默认 Int64Value carrier | CEL null，采用 wrapper 专用规则 |
| prost 普通子 message | schema default view，不制造真实 M |

只有 descriptor/生成 metadata 才能区分这些情况。Protobuf profile 属于类型定义，不能按 Env 翻转普通 Option 的全局转换。普通 Rust 父对象可以包含 protobuf 子对象；父字段与子对象分别使用各自定义。

整个 prost M 放入 `Option<M>` 变量或函数边界时，外层仍是普通 optional<M>。未来 v4 的 `_opt()` getter 返回值一旦被单独绑定，也使用普通 Option 值语义；v4 message 本身不按 Rust Option 成员推导字段类型。详见 [v4 存储与 getter](protobuf-v4-status.md#已发布的新增能力)。

## 嵌套、集合与 null

```text
Option<Option<T>> → optional_type<optional_type<τ(T)>>
Vec<Option<T>>    → list<optional_type<τ(T)>>
Map<K, Option<T>> → map<τ(K), optional_type<τ(T)>>
```

None 元素不删除 list 项或 map entry；map key 仍受 CEL 类型限制。内层转换失败不通过过滤元素隐藏。全局 None→null 不能无损表达嵌套 Option。

CEL 没有一般的 `T | null` 类型。primitive wrapper 的专用 nullable 规则与 legacy null assignment 又不同：后者可能让某些 optional/struct 声明接受 null，却不保证 Rust decoder 能解码。核心设计关闭 legacy null assignment，保留 wrapper 专用规则，不用 Null→None 掩盖差异。

源码：[checker 兼容规则](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/internal/type_inference_context.cc#L51)、[配置](https://github.com/xjasonli/cel-cpp/blob/c84e883df5932dcc992dd3494a771c37e85ae1fe/checker/checker_options.h#L29)。

## 为什么移除泛型 Optional 包装

现有 `Optional<T>` 存储 `Box<Option<T>>`，与 std Option 使用相同的 CEL 类型和值转换。它没有独立的 presence、null 或函数匹配语义，却需要维护另一套 map/filter/借用方法。

| 现有附加能力或成本 | 对应处理 |
|---|---|
| CEL 风格 Display | 由动态 Value/helper 提供 |
| 隐含 Box 打断递归 | 宿主显式使用 `Option<Box<Node>>`；Box 在 CEL 类型上透明 |
| derive(Default) 附加 T: Default、缺少 Copy、借用包装分配 | 可以修复包装，但不是必须保留包装的语义理由 |
| 本地 newtype 可定义严格 std TryFrom | 有独立价值，但本设计选择本地 FromValue/decode 统一解码 |
| 想约束动态 optional 内容或隐藏布局 | 将来可使用私有存储的具体 newtype；当前泛型包装的 DerefMut 本来也不保证该不变量 |

删除的是同义泛型容器，不是 `Value::Optional`、OptionalType 或 CEL optional library。也不把 `Optional<T>` 简单 alias 成 Option 后宣称保留全部旧方法。

源码：[当前 Optional](../../../../src/values/optional.rs#L34)、[两套值转换](../../../../src/values/impls/optional.rs#L4)。

### 动态 Value 仍需递归间接层

目标 payload 为 `Value::Optional(Option<Box<Value>>)`。Box 打断递归；none 不需要额外 Box。别名 `OptionalValue` 只有命名作用，不增加独立类型身份，不能据此另实现 Display 或与 Option blanket 重叠的转换。

动态访问还要区分“不是 optional”与“optional.none”。单个 Option<&Value> 无法同时表达这两种空状态；严格 decode 使用 Result<Option<T>, ...>，访问器也须保留相应层次。

C++ OptionalValue::Of 禁止包装 Error/Unknown。即使 Rust newtype 限制了原始内容，合法宿主值在 FFI 转换时仍可能失败，因此边界 guard 不能省略。具体修补范围只在 [核心设计 §10.2](../design.md#102-error-与-unknown) 维护。

## 为什么不能统一依靠 std TryFrom

标准库已有 `From<T> for Option<T>` 和关联 blanket TryFrom：

```rust
Option::<Value>::try_from(Value::Optional(None))
// Ok(Some(Value::Optional(None)))，不是 CEL 解码后的 None

Option::<Value>::try_from(Value::Int(7))
// Ok(Some(Value::Int(7)))，不是“非 optional”的类型错误
```

这是标准 Rust 包装，cel-cxx 不能覆盖。当前受限 TryFrom 通过内层 Error = FromValueError 排除了 Value 的 identity conversion；扩大为涵盖 Value 的泛型 FromValue 会与标准 blanket 重叠。

Box<Value>/Arc<Value> 也存在直接包装；通用 `TryFrom<Value> for Box<T>` 还受 fundamental/uncovered 参数的 orphan 约束。不能概括为“所有 Arc TryFrom 都非法”，但也不能把 std TryFrom 作为完整组合的前提。

本地 `FromValue` 是 cel-cxx 的 trait，可以定义严格解码，并通过 decode 提供统一入口。源码与规则：[当前转换 trait](../../../../src/values/traits.rs#L42)、[std Option](https://doc.rust-lang.org/std/option/enum.Option.html)、[Rust orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)。相关正负例见验证记录。

### 递归解码为何必须显式传递 context

当前 FromValue 只有输入 Value，Option/list 的递归实现可以调用内层解码，却无法传递目标 registry、profile 配置或共同预算。若每层都创建独立 operation，深层 Optional/集合会反复获得新预算；只有文字要求“共享 context”不能保证实现成立。

[核心 §6.2](../design.md#62-本地转换-traits) 因此规定 FromValue/FromOwnedValue 的必需方法显式接收 ValueContext；无 context 的 decode 便利入口只在最外层创建 operation。Option、Box/Arc、集合、函数参数与 profile materializer 继续递归委托同一个 context。借用输出寿命只绑定 Value，不绑定短期的 context mutable borrow；lending async 在 await 前结束该 borrow，由外层 guard 保活 operation。不能利用 std TryFrom、独立 decode 或隐含 TLS 绕过这条协议。

字段 presence 与转换资源分开：has/zero/type/debug 无 arena，限定为不物化、不递归转换的查询，不计 Native 转换预算。ForEach 的 presence 筛选不收费，真正的 get/转换收费；重复表达式调用仍受 CEL 执行限制约束。完整合同与边界见 [核心 §9.6](../design.md#96-operation-context-的具体传递)、[§12.5](../design.md#125-深度与成本)。

## Box/Arc 类型透明不等于所有权可互换

Option 的单一泛型实现可以递归委托 T；Box/Arc 透明保留 T 的 CEL 类型。不需要分别特化 Option<T>、Option<Box<T>>、Option<Arc<T>>，这些实现会重叠。

难点是所有权能力：在共享转换 capability 上叠加 Clone blanket 与 TypedStruct blanket 也会重叠，应由一个共享转换入口委托具体能力。derive 给本地宿主生成能力，库为标准容器提供统一实现；不要求下游替外部 Arc 实现外部 cel-cxx trait。这不影响库为 TypedStruct 提供动态 Struct 转发。

| 操作 | 首版普通 Native 的所有权行为 |
|---|---|
| 消费 T/Box<T> 输入 | 移入拥有实际 T 的 Struct adapter |
| 消费 Arc<T> 输入 | 独占时移出 T，否则 Clone；不承诺保留原 Arc 身份 |
| 从 &Value 解码 T/Box<T>/Arc<T> | 验证后 Clone 出真实 T，再按目标类型包装 |
| 消费 Value 解码 T/Box<T>/Arc<T> | 验证后移出真实 T，再按目标类型包装 |
| schema/default 或 message view | 使用明确的 typed materializer，不能假装存在真实 T |

动态对象统一为 Box<dyn Struct>，TypedStruct 要求宿主 Clone，Struct 使用 DynClone；字段 getter 从 &Parent 返回拥有值时可以复制 Child。Box 输入允许 move，但共享 &Value 不能被搬空。Arc<&T> 与 Arc<T>、Box<&T> 与 Box<T> 不可互换，size 相同也不能证明指针 cast 安全。

相同 CEL 类型不能靠 T/Box<T>/Arc<T> 区分重载；转换失败也不能尝试另一个 overload 掩盖错误。owned GAT 只约束解码的输出类型，消费型解码是另一个接口；borrowed/lending 回调仍遵守独立生命周期合同。

具体规则见 [核心 §5](../design.md#5-rust-值表示与所有权)、[§6.5](../design.md#65-arc-的转换策略) 与 [§12.5](../design.md#125-深度与成本)。宿主 PartialEq/Clone/Drop 不可抢占，不能把 operation 深度当作对所有递归的保护。
