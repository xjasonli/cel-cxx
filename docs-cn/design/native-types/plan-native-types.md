# Native Types、Prost 与统一转换实施计划

状态：可执行的实施计划与后续执行记录；生产实现尚未开始。

日期：2026-10-11。

本文件用于持续实施，执行时更新状态、证据、问题与阻塞记录。[核心设计](design.md) 是唯一语义权威；本计划安排工作，不重新定义接口或默认行为。编写计划、修订设计或通过临时模型不等于完成生产实现。

## Goal Input

按 `docs-cn/design/native-types/design.md` 实现首版 Native Types、Prost adapter 与统一值转换，完成真实 Rust↔C++ 的变量、函数、字段、构造和结果恢复闭环。保留现有 RuntimeEnv、Opaque 与可准入的 descriptor pool 能力，移除旧 bytes/v3/不安全借用通路。

实现必须落实 pool 表示闭包、双向 Parsed/Custom 比较、普通 NullValue 的 C++ int 字段语义、共享 ValueContext、目标 Env 的 checked AST 重新检查，以及消费型 root/重复输出的最小副本与释放合同。依赖升级、MSRV 和 feature 支持以实际构建为依据；首版不包含 future v4、Any、legacy/foreign Custom 接入或 Native select optimization。

执行期间，代码文件（注释、标识符、字面量）及计划/指定执行台账以外的文档不得携带计划执行状态：不写阶段/步骤编号、内部迭代 ID、评审轮次或“提交前删除”标记。临时 instrumentation 仅限本计划明确声明的探针，提交前移除并用 rg 验证；依赖它的临时测试也不提交，证据记录在本计划。正式测试夹具的 Clone/Drop/codec 计数断言属于长期验收，可以保留。

Git 操作仅限本地；除非用户另行明确要求发布，不 push、不创建/更新 PR、不修改任何 remote。不要 reset、覆盖或混入用户已有变更，不以清理工作树为由丢弃它们。

## Authority And Scope

- [design.md](design.md)：类型、profile、所有权、配置、拒绝边界与验收的唯一权威。
- [目录说明](README.md)、[源码对照](supplements/native-interop.md)、[Option 理由](supplements/option-semantics.md)、[验证记录](supplements/validation.md)：解释与证据，不能替代正式实现测试。
- `docs/design/native-types/archive/protobuf-support-legacy.md`：历史提案，不恢复其 bytes/v3/v4 默认行为。

范围覆盖 cel-cxx runtime、FFI、derive/codegen、prost build integration、cel-cpp 通用补丁、依赖 pin 与活跃使用文档。版本一致性、下游独立 crate、同步/异步取消、并发、arena 和正式性能夹具属于交付内容。future v4/Any/完整 Unknown、partial evaluation、外部 C++ 对象接入与优化仍按设计明确拒绝；不写空实现或兼容 alias 来充数。

## Source Baseline Read For This Plan

| 来源 | 编写时基线 | 核对方式/定位 |
|---|---|---|
| cel-cxx | `7ff30ee425d391643a0abd11a85a934e70324077` | `git rev-parse HEAD`；本工作树还有用户的设计文档变更 |
| cel-cpp | `c84e883df5932dcc992dd3494a771c37e85ae1fe` | `crates/cel-build-utils/cel/MODULE.bazel` 的 git_override；本地对照 checkout 也为此提交 |
| prost/prost-build/prost-types | 锁定基线 0.14.1 | Cargo.lock 与 cargo registry 对照；不从范围 `0.14` 推断固定版本 |
| C++ protobuf | 35.1 | 本地 cel-cpp Bazel external 与反例 probe；重新构建时复核依赖解析 |
| Rust | 实际 `rustc 1.96.1 (31fca3adb 2026-06-26)` | `rustc --version`；仅安装 stable，未验证声明的 1.80 |

以下锚点使用符号，不使用容易漂移的源码行号。执行时重新测量路径/符号与实际树；本表是已读取的基线快照，不代表未来树仍相同。

## Current Code Snapshot

| 当前路径/符号 | 实际情况 | 目标差距 |
|---|---|---|
| `src/values/mod.rs`：StructValue / from_bytes / to_bytes | StructValue 保存 type_name 与 bytes | Box<dyn Struct>、真实 owner、受控 downcast/move |
| `src/values/traits.rs`：FromValue::from_value | GAT 借用解码只有 Value 参数 | 带 ValueContext 的递归解码及独立 FromOwnedValue |
| `src/values/impls/optional.rs`、list.rs、map.rs；`src/values/optional.rs` | Option/Optional 重复；递归解码不携带 operation | 严格 std Option、多层语义、统一预算 |
| `src/ffi/values.rs`：value_from_rust / value_to_rust | struct 依赖 bytes/message 编解码 | representation 准入、Custom wrapper、受信 owner 提取 |
| `src/ffi/activation.rs`：FfiActivationImpl / FfiFunctionImpl | 绑定转换会 Clone；函数导入/导出缺少统一 context | owned/borrowed 来源、contextual 调用、结果批量提取 |
| `src/program/inner.rs`：clone_with_new_arena | eval 已创建自己的 arena | operation 登记、planning 隔离、取消清理与消费型结果 |
| `src/ffi/types.rs`；FFI Ast::return_type / TypeSpec_to_type | 类型/返回类型按 descriptor 查找 | provider-only、复合参数、失败传播与重新检查导入 |
| `crates/cel-cxx-ffi/src/common/type_provider.*`、include/type_provider.h | 通用 reflector 生命周期较粗，Status 包装有 payload 丢失风险 | immutable metadata owner、独占 builder 与专用三分支 glue |
| cel-cpp TypeCheckEnv / RuntimeTypeProvider / TypeRegistry | checker 有默认链回退；runtime facade 未提供所需安装协议 | 单一组合 provider、原子安装与冻结 |
| cel-cpp ParsedMessageValue::Equal / CustomStructValue::Equal | Parsed 侧可直接走字段 helper | Parsed/Custom 两序调用同一个 Custom comparator |
| cel-cpp Value::Enum、WrapField 与 struct_value_builder | NullValue 普通字段读取 null，部分 setter 将 int 写成 0 | 普通字段 raw int；JSON Value kind 保留 null |
| `src/function/*`、`src/macros/*` | 现有借用/async 适配须迁移 | Owned/Borrow/OptionalArg 与安全 lending 生命周期 |
| `crates/cel-cxx-macros/src/prost_value.rs` 等 | 现有生成规则是旧 protobuf 值通路 | descriptor 驱动的真正 Protobuf profile |
| Cargo.toml | resolver=3、rust-version=1.80，同时保留 v3/v4 feature 与旧示例 | 先核对真实工具链最低要求，再迁移依赖/feature |

基线反例的实际输入、输出与模型边界已记录于[验证资料](supplements/validation.md#组合-provider-与值表示验证)。它们揭示待修补行为；新的 EnvTypeProvider、move bridge、derive 和 profile 尚未生产验证。

## Execution Contract

1. 实施前重读核心设计与本计划，运行基线清点，记录实际 HEAD、锁版本和工作树内容；当前树已有 staged/unstaged/untracked 文档，不要求清空。
2. 按依赖实施，每个阶段记录开始/结束、命令、测试数量和结果。接口跨 Rust/C++ 的原子迁移可在阶段内暂时红构建；逐项记录并修复，不能隐藏失败、忽略测试或提交临时 shim。
3. 阶段状态只反映生产实现与其验收。局部模型、只检查 Rust 而跳过 C++、只改 sibling checkout 或缓存 external，都不能记为相应闭环 Done。
4. 设计内可解决的问题记录 Issue 的决策与依据后继续。真实语义缺口先用编译/运行证据修订权威设计；不得恢复 bytes fallback、同名 Parsed 恢复或 lifetime cast。
5. 代码文件（注释、标识符、字面量）和非计划文档不携带阶段/步骤编号、迭代 ID、评审轮次或临时删除标记；注释只保存设计动机与可验证事实。临时探针在仓库外，提交前 rg 检查干净；依赖临时 instrumentation 的测试不提交。
6. Git 仅限本地；除非用户明确要求发布，不 push、不创建/更新 PR、不修改 remote。本计划没有远程发布阶段。
7. 修改 cel-cpp 真实 checkout/本地分支，随后用本地可重建 patch 或明确 pin 集成到 cel-cxx；不能把 external 缓存改动当成依赖交付。若需先做本地提交可按用户已有授权执行，仍遵守工作树内容归属。
8. 每个可交付阶段完成后更新记录；最终移除旧活跃 API 文档、核对 feature/MSRV/平台及依赖可重建性。不得仅更新摘要而保留相互矛盾的正文/示例/测试。

## Implementation State

状态：Not started / In progress / Done / Blocked / Deferred / Superseded。

| 阶段 | 状态 | 前置依赖 | 最近验证命令 | 说明 |
|---|---|---|---|---|
| 0. 基线与回归夹具清点 | Not started | 无 | — | 本计划编写时的源码阅读不等于基线构建通过 |
| 1. 手写 Native 与类型协议闭环 | Not started | 0 | — | 先形成真实 readonly 单字段路径 |
| 2. 消费所有权与 operation 闭环 | Not started | 1 | — | 在全面生成器前验证 root/重复输出/取消 |
| 3. C++ 比较、enum 与值错误修补 | Not started | 1, 2 | — | 支撑全部 profile 与容器 |
| 4. 完整 Rust derive 与槽位构造 | Not started | 1, 2, 3 | — | 各形态/递归/下游 privacy |
| 5. 安全函数与 optional 组合 | Not started | 2, 3, 4 | — | owned/borrowed/lending async |
| 6. Prost 生成与 Protobuf profile | Not started | 1–5 | — | FDS、WKT、default、required、enum |
| 7. 迁移、依赖与文档收敛 | Not started | 1–6 | — | 删除旧路径并重建 pin |
| 8. 完整验收与交付检查 | Not started | 0–7 | — | §18 全矩阵与真实副本计数 |

## Execution Records / Verification Notes

| 阶段 | 开始 | 结束 | 命令/证据与实际结果 | 遗留情况 |
|---|---|---|---|---|
| — | — | — | 尚无生产实施记录；文档修订与历史 probe 见验证资料 | 不据此标记任何实施阶段 Done |

## Dependency Graph

```text
0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8
```

阶段 0 可先运行已有 C++ 反例；通用 C++ 补丁在阶段 3 集中验收。核心顺序先解决名称与实际 carrier 的一致性，再证明 owner/context，最后把已验证的运行协议交给 derive/prost 生成器；不能先大规模生成 adapter 再补生命周期。早期 readonly 夹具本来就没有 constructor，不能用一个假的成功 builder 代替后续 opt-in 构造。

## Issue Policy

记录实现中发现的设计缺口、树漂移、非预期构建失败、行为差异与非平凡选择。决策、理由、下一步都放在同一个 Issue 行；不另建 Autonomous Decision Log。

状态：Open / Investigating / Resolved / Deferred / Blocked / Informational。下列 Resolved 指**设计决策已明确**，不代表生产实现或回归测试通过；完成情况只由 Implementation State 与证据决定。

## Issue Log

| ID | 状态 | 阶段 | 问题 | 调查/影响 | 决策 | 理由 | 下一步 |
|---|---|---|---|---|---|---|---|
| I-001 | Resolved | 1 | Native 覆盖后 pool 父返回旧 Child | 真实 C++ probe 读出 string 与隐藏字段 | ValidatePoolClosure 拒绝冲突 pool 根，五方法同一分类 | 无隐式 carrier 转换；不把 metadata 当成布局变更 | 有限图/回边/dyn/空容器真实回归 |
| I-002 | Resolved | 3 | Parsed/Custom 相等不对称 | 同名空对象得到 false/true | 两种顺序均使用 Custom comparator；Empty 专用校验 | 核心无需识别 Rust，保留默认结构性 Custom 行为 | ==/!=/membership/容器双序测试 |
| I-003 | Resolved | 3, 6 | 普通 NullValue 声明 int，实际返回 null | C++ pool NullHost 数值表达式失败 | 普通 enum getter/accessor/setter 全部 raw int；JSON kind 独立 | 无 Native 的 Env 也必须满足同一字段合同 | singular/repeated/map/oneof/default 的 0/7/null/溢出 |
| I-004 | Resolved | 1, 2, 5 | 解码无法共享预算，Has 无 arena | 当前 FromValue 无 context | FromValue/FromOwnedValue 带 ValueContext；纯 Has 等不计转换预算 | 显式传递可检查，无 TLS/隐藏 operation 推断 | 参数/嵌套/async/独立 decode 与 CEL 限制 |
| I-005 | Resolved | 1 | checked AST 沿用旧字段/函数 metadata | 按名称恢复不足以证明旧类型有效 | 丢弃旧元数据，在目标 Env 重检并重新规划 | 不添加脆弱的全 Env hash；无关注册不妨碍导入 | age:int→string 与 overload/参数变更回归 |
| I-006 | Resolved | 2, 8 | 深层字段复制可能 O(n²) | Clone 剩余子树的工作量为 n(n−1)/2 | 保留拥有型结果，断言最小副本与实际复制量 | 不伪造零复制/硬 CPU 限制，也不引入 parent 链 | n=16/32/64/128 与 1 MiB payload |
| I-007 | Resolved | 6 | prost-types 旧 carrier 与 35.1 新 schema 混用 | 0.14.1 FileDescriptorProto 无 edition；Syntax 只有 0/1，35.1 schema 另有 2 | message metadata 对齐实际布局；内建 Syntax 常量绑定完整权威 enum，typed carrier 保留已知 variant 限制 | 防止虚构字段、错误布局与旧 enum 子集冲突；不拼接用户 schema | 内建清单、完整常量与 typed 2 拒绝 |
| I-008 | Open | 0, 7 | 声明 MSRV 与 resolver/锁依赖未核对 | 只有 1.96.1 实测，不能证明 1.80 | 先测声明版本、Cargo resolver 与全部锁依赖最低要求；如基线声明自相矛盾，以实际可验证的最低版本修正清单和文档并记录依据 | 不把本机通过当作 MSRV 支持，也不无依据提高版本 | 清点要求、运行真实最低工具链构建，保留具体失败/修订证据 |

## Blocker Policy

仅当进展确实需要用户输入、外部状态变化或未授权的公共语义决定时记录 blocker。编译错误多、实现复杂、私有 helper 布局、验证耗时或存在设计内解决方式都不是 blocker。优先调查并继续可独立完成的工作；不虚构阻塞。

## Blocker Log

| ID | 状态 | 阶段 | 阻塞条件 | 调查/已排除方案 | 需要的输入/变化 | 关联问题 |
|---|---|---|---|---|---|---|
| — | None | — | 当前无已知阻塞 | — | — | — |

## Quality Rules

- TypedValue 保持纯静态类型；定义 deterministic、注册 explicit，运行值只验证。不用全局注册表或按字段 miss 回退 pool。
- 真实 T 由 Box 拥有；私有 view 擦除实际 Self。downcast 检查 TypeId/definition，失败保留原 owner；C++ 外壳独占不能证明宿主可 move。
- 所有递归 decoder/字段/物化共享 context；无 reset/TLS，future 不持有短期 mutable context 借用。地址仅用于保活期间的循环检测，不用于比较成功捷径。
- 领域失败保留 CEL Error，基础设施失败保留 Status code/message/payload；Unknown 不折叠成 None。callback panic 不越 FFI，也不伪装成比较 false。
- 不使用 unwrap/expect 处理正常转换、构造、FFI 或取消分支；不要为中间构建加 placeholder、长期别名、bytes fallback 或 ignored tests。明确 Unsupported/Unimplemented 的设计边界保持为拒绝。
- 新 trait/宏支持在独立下游 crate 编译，证明 dyn/GAT/coherence/privacy/Send，而非仅在本 crate 内成立。
- 正式 Clone/Drop/codec 计数使用可重复的测试夹具。仓库外临时 ABI/probe/计时工具只留结果与复现方法，提交前删除任何引入生产文件的临时 instrumentation 并 rg 检查。
- 阶段/步骤编号、迭代 ID、评审轮次和临时删除标记仅出现在本计划/执行台账；代码与非计划文档只表达最终语义。Git 保持本地，未经明确发布指令不操作 remote。
- 检查用符号与实际测试数量；不存在的路径、零匹配 grep 或零执行测试不能算通过。只重跑因新改动/失败/未解决问题而需要的检查。

## Phase 0：基线与回归夹具清点

**目标与依赖**：无前置依赖。确认实际树、工具链和可运行测试，为后续阶段建立可比证据。

**主要文件**：Cargo.toml、Cargo.lock、build.rs、`crates/cel-build-utils/cel/MODULE.bazel`/patches、现有 tests/examples，以及 Source Snapshot 列出的 Rust/C++ 符号。

**工作**：

1. 记录 `git status --short`、双方 `git rev-parse HEAD` 与用户已有变更归属；用内容清单/备份保护其工作。执行前树变化需要重读相关设计/代码。
2. 将本计划每个源符号映射到实际路径与最新定义位置；核对 PR #2115 已存在的 checked ID/type matcher，不重写已有能力。核对 protobuf/prost/protoc、Cargo/rustc 与 dependency lock。
3. 清点现有 Rust/Cpp 测试及 Bazel 实际 target；复现 validation 中 pool 投影、NullValue 与相等基线。记录测试数量、实际输出与层级；CPP 反例不是 Rust bridge 集成结果。
4. 对 I-008 测量最低工具链要求。不能通过设置 `CEL_CXX_FFI_SKIP_BUILD` 来证明集成构建或 MSRV；如仅做 Rust 类型检查，明确记为局部检查。
5. 无 .gitattributes 归一化时，记录换行/编码基线。若执行产生本地提交，`git diff --numstat <base> HEAD` 与 `git diff --numstat --ignore-cr-at-eol <base> HEAD` 必须逐行相等；提交前也对工作树修改检查。

**验证**：`cargo metadata --locked --format-version 1`；`cargo check --workspace --locked`；`cargo test --workspace --locked`；cel-cpp `bazel query 'kind(".*_test rule", //common/... + //checker/... + //runtime/... + //eval/...)'` 后运行受影响目标。不能把基线已有失败隐藏为成功，须调查并归属到 Issue。

**完成条件**：符号/树/版本映射与实际命令结果已记录，真实测试入口已确认，MSRV 问题有可执行解决步骤，用户变更未丢失。

## Phase 1：手写 Native 与类型协议闭环

**目标与依赖**：依赖 0。用手写 readonly Root/Child 夹具证明实际 Env 注册、检查、字段选择和结果转换，不先依赖 derive。

**主要文件**：`src/values/{mod.rs,traits.rs,impls/struct.rs}`；新增 `src/extensions/native_types/*`、`src/types/registry.rs`；`src/env/{mod.rs,inner.rs}`；`src/ffi/{types.rs,values.rs}`；FFI common/type_provider、ast、checker/runtime glue；cel-cpp checker builder/env、TypeRegistry、RuntimeTypeProvider、TypeSpec resolver、Resolver/FlatExprBuilder。

**工作与约束**：

- 建立 Struct/TypedStruct、Any 擦除、canonical definition、字段 TypeRef、profile 与 immutable registry；所有名称/索引/参数归属有 owner。Box<dyn Struct> 与 public macro support 在独立 consumer 编译。
- 为 FromValue/FromOwnedValue 和 getter 提供 ValueContext；先迁移标准 decoder 的递归调用，固定最外层建 operation 的规则。所有创建 Native 的转换验证实际宿主和定义，TypedValue 不注册。
- 导出 CompiledRegistry，安装同一个 EnvTypeProvider 到 checker/runtime；实现五方法、miss/Some(empty)/failure、enum/Opaque 合并与稳定 facade。完整名称只选一次来源。
- ValidatePoolClosure 用显式栈/去重检查所有实际 message 边与 schema 预算；缓存仅发布完整结果。Native 覆盖 Child 时拒绝 pool Parent，禁止旧 carrier 的类型投影；不扫描全 pool。
- BasicStruct TypeSpec 与参数恢复贯穿声明、planner、返回类型；checked import 创建 unchecked AST、丢弃旧 type/reference map、目标 Env 重检与能力遍历，失败不降级。
- 以最小真实 Custom wrapper 和当前 readonly 能力打通 `x.age`/child scalar 读取。constructor 未 opt-in 时明确不可用；不设置假的 factory，不接入任意 Parsed/foreign 输入。进一步零 Clone 提取由阶段 2 完成。

**验证**：真实 Rust integration 先覆盖手写 Root；C++ provider/type_spec/checker 测试；downstream 正例/compile-fail；pool Parent/Child 冲突、自环/互递归、map/repeated/oneof、WKT、并发首次查询；User.age/函数签名跨 Env 导入。检查空值不遗漏静态依赖、旧 metadata 不影响新类型、无关注册不误拒绝。

**完成条件**：有/无 Native Env 的类型规则一致，实际表达式闭环可运行；五方法不泄露被拒绝类型，生命周期与 strict import 回归通过，新增接口不只是模型或 declaration。

## Phase 2：消费所有权与 operation 闭环

**目标与依赖**：依赖 1。在扩展生成器前落实完整 root 的最小副本和本次 operation 的资源/释放边界。

**主要文件**：`src/activation.rs`、`src/ffi/activation.rs`、`src/program/{inner.rs,eval_dispatch.rs}`、Rust value/view/decoder、FFI 新 Custom wrapper/store/operation bindings 与 evaluation options glue。

**工作与约束**：owned activation 真正接管绑定，borrowed activation 保留 source；Cpp view/token 只管理受控来源，不改变公开 Box Clone 合同。输出按 origin 批量分析重复数，结束所有 C++ 视图使用后复制必要副本/移出原 Box。Clone(target arena) 不保留原 arena 借用。planning/eval/显式操作各自 RAII 登记，弱引用与 guard 清理；并发/重入隔离。全部标准/Native decoder、getter、builder 接口引用同一 operation；纯 Rust decode 不需要伪造 arena，has/zero 不做隐含计费。

**验证**：Root identity 零 Clone、m 次输出为 m−1 或保留 source 时 m 次；owner/Drop 恰好释放；真实 child 与重复变量、list/map/optional 结果独立；释放 activation/program/env/arena 后 downcast 仍有效。预算 exhaustion 后需预算的操作持续失败，纯 builtin/Error/Unknown 回传不额外扣费，CEL 逻辑产生的 builtin 结果在 raw/typed 路径一致；计费常量以正式夹具验证，临时模型不锁定生产常量。覆盖未登记 arena、跨 Env C++ Value 拒绝、并发和取消/失败测试。已有 async 返回至少用安全 owned 夹具验证取消；完整 lending 接口在阶段 5 扩展。

**完成条件**：最小 readonly host 的真实 FFI 零 Clone/重复输出/跨 arena 通过，资源未被逐层重置，无长期 context/arena 借用。不能用引用计数、unique_ptr 或临时 lifetime cast 代替来源证明。

## Phase 3：C++ 比较、enum 与值错误修补

**目标与依赖**：依赖 1、2。将通用值语义用于真实 wrapper、pool 与全部容器路径。

**主要文件**：cel-cpp `common/values/{parsed_message_value,custom_struct_value,struct_value}.*`、`common/value.cc`、`common/values/struct_value_builder.cc`、enum accessor、runtime/standard equality、eval equality/select/container access、runtime/optional_types、function_step 与 Rust Status/optional glue。

**工作与约束**：

- Parsed/Custom 两序调用 Custom Equal，结构性 helper 只用于默认 Custom 实现；Native 对普通 Parsed 为 false，受信 Empty 专用检查。排除地址捷径对 NaN/comparator 的绕过，核心不识别 Rust glue。
- 普通 NullValue 的 singular/repeated/map/oneof/default getter 与 setter统一 raw int、i32 与 enum policy；JSON WKT null 分派独立。无 Native Env 同样生效。
- getter/has/constructor 的领域错误转换成 CEL 值，保留基础设施 Status payload；ForEach 的 Error 字段交给 callback，false/error 停止。所有 optional 包装通过共同 helper 传播 Error/Unknown，不能误触 Of 的 DCHECK。
- 复用现有 checked overload ID；补齐 wrapper 可赋值与动态多候选 ambiguity，错误不吞为首个候选或 miss。bool comparator 不引入 fallible 字段聚合协议。

**验证**：Cpp 双序空对象/Empty/伪造 Empty 夹具，==/!=/in/Contains、直接/栈式、checked/parse-only、嵌套 optional/list/map；无 Native pool NullHost 的 0/7/null/溢出；普通 enum policy；JSON null/时间/WKT 回归；领域错误逻辑短路、payload、optional error 和 Error/Unknown 传播。

**完成条件**：实际 accessor/setter 与所有已列比较入口都通过；生产 pin/patch 可供 cel-cxx 测试使用，不仅 sibling 的独立 probe 通过。

## Phase 4：完整 Rust derive 与槽位构造

**目标与依赖**：依赖 1–3。将已验证协议用于所有支持的普通 Rust 形态。

**主要文件**：`crates/cel-cxx-macros/src/lib.rs`、新增 struct derive/codegen、symbol helpers；Rust definition/capability/builder；FFI ValueBuilderState；新增 tests/native_types、独立 consumer 与 compile-fail fixtures。

**工作与约束**：named/newtype/tuple/unit、非空 enum 的各 variant、合成 VariantView owner/key、固定 presence/optional、skip/default/name/generic/cfg/privacy 均依设计生成。字段依赖惰性且不形成 schema Arc 环，不生成 Self 递归 where 证明链。opt-in builder 只存独占槽位，失败锁定 CEL Error，finish 验证后逐字段组装，不创建整对象 Default。合成 record 不恢复为 E，不泄露 parent 链。

**验证**：设计 §18.1、18.4 的正例与编译负例；独立下游、crate rename、raw identifier/泛型；A↔B/Expr 递归；零字段/unit/Empty 区分；Maybe(None) 与未选中 `.C`/`.?C`；可失败 getter、只读/缺字段 literal 与取消 builder。真实 FFI 遍历只选中的字段并保持 callback stop。

**完成条件**：所有支持形态有生成器+真实表达式测试，不支持形态有定位诊断；TypedStruct/blanket/coherence 可编译，构造发布的 T 完整且不依赖常态 codec。

## Phase 5：安全函数与 optional 组合

**目标与依赖**：依赖 2–4。移除旧 Argument cast，以真实 owner/context 支持 owned、scoped borrowed 与 lending async。

**主要文件**：`src/function/*`、`src/macros/*`、`src/values/impls/{optional,list,map}.rs`、`src/ffi/activation.rs` 与 async dispatch、函数宏/示例。

**工作与约束**：签名描述不隐式注册；Owned/Borrow/OptionalArg 与 ArgumentList 显式接收 ValueContext，所有参数共用 operation。私有 ArgumentInputs 持有 owned Value 或受信 Rust 存储 lease；普通拥有型函数根据输入来源调用 FromOwnedValue/FromValue，不产生 import Clone 后再次借用 decode Clone 的副本。BorrowedFunction 在借用期构造 owned Value；BorrowedAsyncFunction 返回随输入生命周期变化的 boxed Future，外层拥有参数/lease 并保活 guard，await 前结束 context mutable borrow。lease 指向实际 Value/T，不是 C++/arena 引用或伪造 Any wrapper。返回按声明+registry 校验；错误返回不被类型 mismatch 覆盖。普通 Option 严格保持 none/some(null)/some(none)，std TryFrom 只作受限便利；重载按 CEL 签名拒绝冲突，optional 内部歧义与 parse-only 多候选有明确诊断。

**验证**：String/&str、真实 &T/owned T、Option/Box/Arc/容器/Value 六位置组合；owned Value 消费解码与 Native lease 的必要单次副本计数；scoped Borrow 的真实 host 引用和 owner 保活；同步与 tokio/smol 的 owned/lending；create/poll/drop panic 与取消；borrow escape、固定 Fut、非 Send、view 无真实 host 编译/运行负例；多个参数共用预算，Context 引用不逃逸；checked/parse-only 重载及空集合回归。

**完成条件**：旧不安全生命周期转换不存在，borrowed 结果全部拥有化，cancel/drop 释放全部输入和 guard；std Option 与 owned decoder 的返回类型真实成立。

## Phase 6：Prost 生成与 Protobuf profile

**目标与依赖**：依赖 1–5。基于 descriptor 与实际 carrier 生成 adapter，覆盖已限定 protobuf 语义。

**主要文件**：新增独立 prost build helper；`crates/cel-cxx-macros/src/prost_value.rs`；runtime prost profile/内建 adapter；build.rs/proto fixtures；required/default-view/WKT/enum 生成模块。

**工作与约束**：固定 schema/carrier 绑定并校验原名/tag/oneof、Bytes/BTreeMap/boxing/extern/custom attributes。option presence、oneof parent 成员、raw enum/effective policy、alias 常量、required completeness 与 required-message cycle 拒绝。SchemaDefault 按需持 schema，不构造 M::default；槽位物化只创建合法完整对象。标准 PartialEq 有受控证明才能复用，生成 implicit float signed-zero presence 修正与 default-view bool 比较，不调用 CEL getter。WKT root/字段/literal 共用专用 adapter，Empty 的 () carrier 不污染全局 unit；JSON、时间、NullValue 的上下文分离。prost-types 清单按 0.14.1 实际布局生成，不能向 FileDescriptorProto 添加 edition；Any/edition/groups/MessageSet 等按依赖闭包拒绝。

**验证**：设计 §18.5 全部矩阵；cargo build helper 的独立下游生成/编译；proto2/3、imports/extern、oneof/synthetic optional、raw unknown/aliases、required 自环/互递归/有限 DAG、schema default、WKT valid/invalid、signed zero/NaN/hidden fields、wire 差异明确断言。普通 C++ pool 与 prost 路径各自测试 NullValue，不互相替代。

**完成条件**：生成 adapter 与实际 Rust 类型一致，函数/字段/构造/结果均真实闭环；缺失 required 不分配默认子树，相等正常只返回 bool，未支持依赖提前明确拒绝。内建 Syntax 的完整常量匹配 pool、typed 2 拒绝，descriptor 消息无虚构 edition；这两项不能用同一“版本兼容”假设带过。

## Phase 7：迁移、依赖与文档收敛

**目标与依赖**：依赖 1–6。收敛到可发布的单一目标接口，确认两仓库可重建。

**主要文件**：Cargo.toml/Cargo.lock/build.rs、宏 crate feature/exports、旧 protobuf_value/protobuf_legacy_value、Optional 包装、tests/examples、README/docs/docs-cn 的活跃指南、Bazel pin/patches。

**工作与约束**：删除 v3 与未实现 v4 placeholder、常态 bytes StructValue、泛型 Optional 与不安全函数适配；迁移原测试而非 ignore。显式 codec 使用实际 prost M；Opaque 与 FDS/enum 接入保留。核对 I-008 后同步真实 MSRV/工具链，运行相应最低工具链构建。CPP 变更用本地重建 patch/明确 pin 集成，记录依赖提交与构建产物归属，清缓存后可重建。中文/英文活跃文档、宏示例与 feature 名称一致，历史档案保留 superseded 标记，不按残留搜索误删除历史事实。

**验证**：Rust 格式/完整 workspace/feature 组合、独立 downstream、干净缓存的 C++ 联调构建；以符号扫描旧 API/导出/feature/源文件与 active docs，再人工分类 codec/历史档案的合法命中。检查同语言相对链接/锚点、代码示例与公开 API。

**完成条件**：单一活跃接口与依赖版本可重建；原用户文档变更被保留并合并，移除的能力不残留在活跃承诺中；真实 MSRV 证据明确。

## Phase 8：完整验收与交付检查

**目标与依赖**：依赖 0–7。按设计 §18 对真实实现验收，维护可复现的完成证据。

**主要文件**：生产 Rust/C++ 测试、downstream/compile-fail、性能夹具与结果、本计划执行记录；必要时同步核心设计的实测边界。

**工作与约束**：将每个 §18 条目映射到实际 test/command/result，检查空矩阵；支持有/无 Native、checked/parse-only、同步/tokio/smol、prost/no-default、Opaque 与 FDS pool。根 identity、m 次输出、child/VariantView/SchemaDefault、lending cancel/panic、跨 arena/Env、budget 并发相互组合。深拷贝链 n=16/32/64/128 严格断言 n(n−1)/2 节点及已定义 bytes，包含分叉/list 与 1 MiB payload；区分宿主 callback 内成本与库额外副本。不把 benchmark 时间当硬 SLA，不把超时样本删除或改称成功。

**验证**：以下 Final Verification Matrix 加设计 §18 的实际映射；Cpp unit 与 Rust↔C++ integration 都要通过，记录实际执行数量。若本地不具备某平台，必须由适当 CI/交叉构建提供证据，未测平台不能称已通过。最后核对工作树内容、换行、文档残留与 instrumentation；每个待提交 hunk 分类为目标实现/必要迁移/正式测试/文档证据，不混入用户无关变更。

**完成条件**：全部首版必要测试和副本/owner 门槛有实际通过证据，遗留 issue 已解决或属于明确范围外的延期；不将 required/authority/ABI/lifetime/资源回归延期后标记完成。

## Final Verification Matrix

实施时将测试名称与非零执行数量填入执行记录；新测试 target 尚未创建，不能声称下列命令已通过。

```bash
cargo fmt --all -- --check
git diff --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo test --workspace --locked --no-default-features
cargo test --workspace --locked --no-default-features --features prost
cargo test --workspace --locked --features prost,tokio
cargo test --workspace --locked --features prost,smol
cargo test --workspace --locked --all-features
cargo test --doc --locked --all-features
```

最低 Rust 工具链的 `cargo +<已验证 MSRV> check/test`、独立 renamed consumer、compile-fail 与性能命令在阶段 0/相应阶段建立后以实际命令补入，不能用当前 stable 或 FFI_SKIP_BUILD 代替。

在**实际 cel-cpp checkout**执行并同步依赖集成证据：

```bash
bazel test //common:value_test //common:type_spec_resolver_test //checker/...
bazel test //runtime/standard:equality_functions_test //eval/eval:equality_steps_test
bazel test //runtime:optional_types_test
```

上面 common:value_test 包含 values 测试文件；新 provider/closure/import/bridge 夹具以实际 BUILD 目标加入。若目标因 rebase 变化，更新映射而非静默跳过；只在 sibling 通过的测试仍须有 cel-cxx pin/patch 的真实集成结果。

最终符号审计包括：

- StructValue 恰有一个 Box<dyn Struct> 定义，Struct/TypedStruct/FromOwnedValue/ValueContext 各有一个规范定义，所有用法实际编译。
- SetTypeProvider/InstallTypeProvider 的实现、调用与冻结检查存在；两侧持有同一 provider。ValidatePoolClosure、checked import 的清 metadata/recheck、全部新 TypeSpec 验证由真实测试覆盖。
- recursive decoder 调用带 context 方法；正常内部路径不调用独立 decode 重新创建 operation。Has/zero 不读取 arena/TLS，计数断言符合设计。
- ParsedMessageValue::Equal 的 Custom 分支不直接调用字段 helper；保留 Custom 默认 helper。NullValue 普通 getter/setter 不走 null/强制 0，JSON kind 仍专用。
- 旧 protobuf_legacy/protobuf_value 源文件、feature/export、泛型 Optional 与 Argument lifetime cast 在活跃代码中为零；explicit codec 与历史档案命中逐项分类。
- 日常 Native codec=0、root Clone=0、m 次输出的最小副本、深拷贝链的精确复制量与最终 owner/drop 都由正式测试断言，不能只凭源码 grep。

所有审计用当前路径+符号执行 rg 并记录定义数量；源码移动或 rebasing 后更新映射。若有本地提交，无归一化仓库的 numstat/ignore-cr-at-eol 对比必须相等。

## Final Completion Criteria

核心设计 §18 的首版必要能力与拒绝边界全部落实；真实类型、carrier、owner、context、比较与 AST 规则一致；依赖可重建，feature/MSRV/下游/privacy/线程与取消证据完整。全部状态与 issue 对应实际实现/测试结果，工作树无临时 instrumentation、无伪造通过记录、无无关内容混入。

future v4、Any 与 Native 优化未实现不影响首版完成，其拒绝边界必须已测试。源 probe、模型或单仓库补丁通过不能替代完整实现；未经用户明确指令，交付检查不会进行远程发布。
