use crate::absl::{MutSpanElement, SpanElement, StringView};
use crate::common::Constant;
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/strings/string_view.h>);
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<common/expr.h>);
        type Constant = super::Constant;
        type ExprKindCase = super::ExprKindCase;
        type Expr;

        fn id(self: &Expr) -> i64;
        fn set_id(self: Pin<&mut Expr>, id: i64);
        fn kind_case(self: &Expr) -> ExprKindCase;

        fn has_const_expr(self: &Expr) -> bool;
        fn const_expr(self: &Expr) -> &Constant;
        #[rust_name = "const_expr_mut"]
        fn mutable_const_expr(self: Pin<&mut Expr>) -> Pin<&mut Constant>;

        fn has_ident_expr(self: &Expr) -> bool;
        fn ident_expr(self: &Expr) -> &IdentExpr;
        #[rust_name = "ident_expr_mut"]
        fn mutable_ident_expr(self: Pin<&mut Expr>) -> Pin<&mut IdentExpr>;

        fn has_select_expr(self: &Expr) -> bool;
        fn select_expr(self: &Expr) -> &SelectExpr;
        #[rust_name = "select_expr_mut"]
        fn mutable_select_expr(self: Pin<&mut Expr>) -> Pin<&mut SelectExpr>;

        fn has_call_expr(self: &Expr) -> bool;
        fn call_expr(self: &Expr) -> &CallExpr;
        #[rust_name = "call_expr_mut"]
        fn mutable_call_expr(self: Pin<&mut Expr>) -> Pin<&mut CallExpr>;

        fn has_list_expr(self: &Expr) -> bool;
        fn list_expr(self: &Expr) -> &ListExpr;
        #[rust_name = "list_expr_mut"]
        fn mutable_list_expr(self: Pin<&mut Expr>) -> Pin<&mut ListExpr>;

        fn has_struct_expr(self: &Expr) -> bool;
        fn struct_expr(self: &Expr) -> &StructExpr;
        #[rust_name = "struct_expr_mut"]
        fn mutable_struct_expr(self: Pin<&mut Expr>) -> Pin<&mut StructExpr>;

        fn has_map_expr(self: &Expr) -> bool;
        fn map_expr(self: &Expr) -> &MapExpr;
        #[rust_name = "map_expr_mut"]
        fn mutable_map_expr(self: Pin<&mut Expr>) -> Pin<&mut MapExpr>;

        fn has_comprehension_expr(self: &Expr) -> bool;
        fn comprehension_expr(self: &Expr) -> &ComprehensionExpr;
        #[rust_name = "comprehension_expr_mut"]
        fn mutable_comprehension_expr(self: Pin<&mut Expr>) -> Pin<&mut ComprehensionExpr>;

        type UnspecifiedExpr;
        type IdentExpr;
        fn name(self: &IdentExpr) -> &CxxString;
        fn set_name<'a>(self: Pin<&mut IdentExpr>, name: string_view<'a>);

        type SelectExpr;
        fn has_operand(self: &SelectExpr) -> bool;
        fn operand(self: &SelectExpr) -> &Expr;
        #[rust_name = "operand_mut"]
        fn mutable_operand(self: Pin<&mut SelectExpr>) -> Pin<&mut Expr>;
        fn set_operand(self: Pin<&mut SelectExpr>, operand: UniquePtr<Expr>);
        fn release_operand(self: Pin<&mut SelectExpr>) -> UniquePtr<Expr>;

        fn field(self: &SelectExpr) -> &CxxString;
        fn set_field<'a>(self: Pin<&mut SelectExpr>, field: string_view<'a>);

        fn test_only(self: &SelectExpr) -> bool;
        fn set_test_only(self: Pin<&mut SelectExpr>, test_only: bool);

        type CallExpr;
        fn function(self: &CallExpr) -> &CxxString;
        fn set_function<'a>(self: Pin<&mut CallExpr>, function: string_view<'a>);

        fn has_target(self: &CallExpr) -> bool;
        fn target(self: &CallExpr) -> &Expr;
        #[rust_name = "target_mut"]
        fn mutable_target(self: Pin<&mut CallExpr>) -> Pin<&mut Expr>;
        fn set_target(self: Pin<&mut CallExpr>, target: UniquePtr<Expr>);
        fn release_target(self: Pin<&mut CallExpr>) -> UniquePtr<Expr>;

        fn args(self: &CallExpr) -> &CxxVector<Expr>;
        #[rust_name = "args_mut"]
        fn mutable_args(self: Pin<&mut CallExpr>) -> Pin<&mut CxxVector<Expr>>;
        fn add_args(self: Pin<&mut CallExpr>) -> Pin<&mut Expr>;

        type ListExprElement;
        fn has_expr(self: &ListExprElement) -> bool;
        fn expr(self: &ListExprElement) -> &Expr;
        #[rust_name = "expr_mut"]
        fn mutable_expr(self: Pin<&mut ListExprElement>) -> Pin<&mut Expr>;
        fn set_expr(self: Pin<&mut ListExprElement>, expr: UniquePtr<Expr>);

        fn optional(self: &ListExprElement) -> bool;
        fn set_optional(self: Pin<&mut ListExprElement>, optional: bool);

        type ListExpr;
        fn elements(self: &ListExpr) -> &CxxVector<ListExprElement>;
        #[rust_name = "elements_mut"]
        fn mutable_elements(self: Pin<&mut ListExpr>) -> Pin<&mut CxxVector<ListExprElement>>;
        fn add_elements(self: Pin<&mut ListExpr>) -> Pin<&mut ListExprElement>;

        type StructExprField;
        fn id(self: &StructExprField) -> i64;
        fn set_id(self: Pin<&mut StructExprField>, id: i64);
        fn name(self: &StructExprField) -> &CxxString;
        fn set_name<'a>(self: Pin<&mut StructExprField>, name: string_view<'a>);
        fn has_value(self: &StructExprField) -> bool;
        fn value(self: &StructExprField) -> &Expr;
        #[rust_name = "value_mut"]
        fn mutable_value(self: Pin<&mut StructExprField>) -> Pin<&mut Expr>;
        fn set_value(self: Pin<&mut StructExprField>, value: UniquePtr<Expr>);
        fn optional(self: &StructExprField) -> bool;
        fn set_optional(self: Pin<&mut StructExprField>, optional: bool);

        type StructExpr;
        fn name(self: &StructExpr) -> &CxxString;
        fn set_name<'a>(self: Pin<&mut StructExpr>, name: string_view<'a>);
        fn fields(self: &StructExpr) -> &CxxVector<StructExprField>;
        #[rust_name = "fields_mut"]
        fn mutable_fields(self: Pin<&mut StructExpr>) -> Pin<&mut CxxVector<StructExprField>>;
        fn add_fields(self: Pin<&mut StructExpr>) -> Pin<&mut StructExprField>;

        type MapExprEntry;
        fn id(self: &MapExprEntry) -> i64;
        fn set_id(self: Pin<&mut MapExprEntry>, id: i64);

        fn has_key(self: &MapExprEntry) -> bool;
        fn key(self: &MapExprEntry) -> &Expr;
        #[rust_name = "key_mut"]
        fn mutable_key(self: Pin<&mut MapExprEntry>) -> Pin<&mut Expr>;
        fn set_key(self: Pin<&mut MapExprEntry>, key: UniquePtr<Expr>);

        fn has_value(self: &MapExprEntry) -> bool;
        fn value(self: &MapExprEntry) -> &Expr;
        #[rust_name = "value_mut"]
        fn mutable_value(self: Pin<&mut MapExprEntry>) -> Pin<&mut Expr>;
        fn set_value(self: Pin<&mut MapExprEntry>, value: UniquePtr<Expr>);

        fn optional(self: &MapExprEntry) -> bool;
        fn set_optional(self: Pin<&mut MapExprEntry>, optional: bool);

        type MapExpr;
        fn entries(self: &MapExpr) -> &CxxVector<MapExprEntry>;
        #[rust_name = "entries_mut"]
        fn mutable_entries(self: Pin<&mut MapExpr>) -> Pin<&mut CxxVector<MapExprEntry>>;
        fn add_entries(self: Pin<&mut MapExpr>) -> Pin<&mut MapExprEntry>;

        type ComprehensionExpr;
        fn iter_var(self: &ComprehensionExpr) -> &CxxString;
        fn set_iter_var<'a>(self: Pin<&mut ComprehensionExpr>, iter_var: string_view<'a>);

        fn iter_var2(self: &ComprehensionExpr) -> &CxxString;
        fn set_iter_var2<'a>(self: Pin<&mut ComprehensionExpr>, iter_var2: string_view<'a>);

        fn has_iter_range(self: &ComprehensionExpr) -> bool;
        fn iter_range(self: &ComprehensionExpr) -> &Expr;
        #[rust_name = "iter_range_mut"]
        fn mutable_iter_range(self: Pin<&mut ComprehensionExpr>) -> Pin<&mut Expr>;
        fn set_iter_range(self: Pin<&mut ComprehensionExpr>, iter_range: UniquePtr<Expr>);
        fn release_iter_range(self: Pin<&mut ComprehensionExpr>) -> UniquePtr<Expr>;

        fn accu_var(self: &ComprehensionExpr) -> &CxxString;
        fn set_accu_var<'a>(self: Pin<&mut ComprehensionExpr>, accu_var: string_view<'a>);

        fn has_accu_init(self: &ComprehensionExpr) -> bool;
        fn accu_init(self: &ComprehensionExpr) -> &Expr;
        #[rust_name = "accu_init_mut"]
        fn mutable_accu_init(self: Pin<&mut ComprehensionExpr>) -> Pin<&mut Expr>;
        fn set_accu_init(self: Pin<&mut ComprehensionExpr>, accu_init: UniquePtr<Expr>);
        fn release_accu_init(self: Pin<&mut ComprehensionExpr>) -> UniquePtr<Expr>;

        fn has_loop_condition(self: &ComprehensionExpr) -> bool;
        fn loop_condition(self: &ComprehensionExpr) -> &Expr;
        #[rust_name = "loop_condition_mut"]
        fn mutable_loop_condition(self: Pin<&mut ComprehensionExpr>) -> Pin<&mut Expr>;
        fn set_loop_condition(self: Pin<&mut ComprehensionExpr>, loop_condition: UniquePtr<Expr>);
        fn release_loop_condition(self: Pin<&mut ComprehensionExpr>) -> UniquePtr<Expr>;

        fn has_loop_step(self: &ComprehensionExpr) -> bool;
        fn loop_step(self: &ComprehensionExpr) -> &Expr;
        #[rust_name = "loop_step_mut"]
        fn mutable_loop_step(self: Pin<&mut ComprehensionExpr>) -> Pin<&mut Expr>;
        fn set_loop_step(self: Pin<&mut ComprehensionExpr>, loop_step: UniquePtr<Expr>);
        fn release_loop_step(self: Pin<&mut ComprehensionExpr>) -> UniquePtr<Expr>;

        fn has_result(self: &ComprehensionExpr) -> bool;
        fn result(self: &ComprehensionExpr) -> &Expr;
        #[rust_name = "result_mut"]
        fn mutable_result(self: Pin<&mut ComprehensionExpr>) -> Pin<&mut Expr>;
        fn set_result(self: Pin<&mut ComprehensionExpr>, result: UniquePtr<Expr>);
        fn release_result(self: Pin<&mut ComprehensionExpr>) -> UniquePtr<Expr>;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/expr.h>);

        // Expr
        fn Expr_new() -> UniquePtr<Expr>;
        fn Expr_set_const_expr(expr: Pin<&mut Expr>, constant: UniquePtr<Constant>);
        fn Expr_set_ident_expr(expr: Pin<&mut Expr>, ident_expr: UniquePtr<IdentExpr>);
        fn Expr_set_select_expr(expr: Pin<&mut Expr>, select_expr: UniquePtr<SelectExpr>);
        fn Expr_set_call_expr(expr: Pin<&mut Expr>, call_expr: UniquePtr<CallExpr>);
        fn Expr_set_list_expr(expr: Pin<&mut Expr>, list_expr: UniquePtr<ListExpr>);
        fn Expr_set_struct_expr(expr: Pin<&mut Expr>, struct_expr: UniquePtr<StructExpr>);
        fn Expr_set_map_expr(expr: Pin<&mut Expr>, map_expr: UniquePtr<MapExpr>);
        fn Expr_set_comprehension_expr(expr: Pin<&mut Expr>, comprehension_expr: UniquePtr<ComprehensionExpr>);

        fn Expr_size_of() -> usize;
        fn Expr_push_unique(expr: Pin<&mut CxxVector<Expr>>, value: UniquePtr<Expr>);
        fn Expr_pop_unique(expr: Pin<&mut CxxVector<Expr>>) -> UniquePtr<Expr>;

        fn ListExprElement_new() -> UniquePtr<ListExprElement>;
        fn ListExprElement_size_of() -> usize;
        fn ListExprElement_push_unique(list_element: Pin<&mut CxxVector<ListExprElement>>, value: UniquePtr<ListExprElement>);
        fn ListExprElement_pop_unique(list_element: Pin<&mut CxxVector<ListExprElement>>) -> UniquePtr<ListExprElement>;

        fn StructExprField_new() -> UniquePtr<StructExprField>;
        fn StructExprField_size_of() -> usize;
        fn StructExprField_push_unique(struct_field: Pin<&mut CxxVector<StructExprField>>, value: UniquePtr<StructExprField>);
        fn StructExprField_pop_unique(struct_field: Pin<&mut CxxVector<StructExprField>>) -> UniquePtr<StructExprField>;

        fn MapExprEntry_new() -> UniquePtr<MapExprEntry>;
        fn MapExprEntry_size_of() -> usize;
        fn MapExprEntry_push_unique(map_entry: Pin<&mut CxxVector<MapExprEntry>>, value: UniquePtr<MapExprEntry>);
        fn MapExprEntry_pop_unique(map_entry: Pin<&mut CxxVector<MapExprEntry>>) -> UniquePtr<MapExprEntry>;
    }
}

// ExprKind
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExprKindCase {
    Unspecified = 0,
    Constant,
    Ident,
    Select,
    Call,
    List,
    Struct,
    Map,
    Comprehension,
}

unsafe impl cxx::ExternType for ExprKindCase {
    type Id = cxx::type_id!("cel::ExprKindCase");
    type Kind = cxx::kind::Trivial;
}

impl ExprKindCase {
    pub fn is_unspecified(&self) -> bool {
        *self == ExprKindCase::Unspecified
    }

    pub fn is_constant(&self) -> bool {
        *self == ExprKindCase::Constant
    }
    
    pub fn is_ident(&self) -> bool {
        *self == ExprKindCase::Ident
    }

    pub fn is_select(&self) -> bool {
        *self == ExprKindCase::Select
    }
    
    pub fn is_call(&self) -> bool {
        *self == ExprKindCase::Call
    }

    pub fn is_list(&self) -> bool {
        *self == ExprKindCase::List
    }
    
    pub fn is_struct(&self) -> bool {
        *self == ExprKindCase::Struct
    }

    pub fn is_map(&self) -> bool {
        *self == ExprKindCase::Map
    }
    
    pub fn is_comprehension(&self) -> bool {
        *self == ExprKindCase::Comprehension
    }
}

// Expr
pub use ffi::Expr;
unsafe impl Send for Expr {}
unsafe impl Sync for Expr {}

impl Expr {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Expr_new()
    }

    pub fn set_const_expr(self: Pin<&mut Self>, constant: cxx::UniquePtr<Constant>) {
        ffi::Expr_set_const_expr(self, constant);
    }

    pub fn set_ident_expr(self: Pin<&mut Self>, ident_expr: cxx::UniquePtr<IdentExpr>) {
        ffi::Expr_set_ident_expr(self, ident_expr);
    }
    
    pub fn set_select_expr(self: Pin<&mut Self>, select_expr: cxx::UniquePtr<SelectExpr>) {
        ffi::Expr_set_select_expr(self, select_expr);
    }

    pub fn set_call_expr(self: Pin<&mut Self>, call_expr: cxx::UniquePtr<CallExpr>) {
        ffi::Expr_set_call_expr(self, call_expr);
    }
    
    pub fn set_list_expr(self: Pin<&mut Self>, list_expr: cxx::UniquePtr<ListExpr>) {
        ffi::Expr_set_list_expr(self, list_expr);
    }

    pub fn set_struct_expr(self: Pin<&mut Self>, struct_expr: cxx::UniquePtr<StructExpr>) {
        ffi::Expr_set_struct_expr(self, struct_expr);
    }
    
    pub fn set_map_expr(self: Pin<&mut Self>, map_expr: cxx::UniquePtr<MapExpr>) {
        ffi::Expr_set_map_expr(self, map_expr);
    }

    pub fn set_comprehension_expr(self: Pin<&mut Self>, comprehension_expr: cxx::UniquePtr<ComprehensionExpr>) {
        ffi::Expr_set_comprehension_expr(self, comprehension_expr);
    }
}

impl SpanElement for Expr {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_Expr");
}

impl MutSpanElement for Expr {
    type TypeId = cxx::type_id!("rust::cel_cxx::MutSpan_Expr");
}

impl crate::SizedExternType for Expr {
    fn size_of() -> usize {
        ffi::Expr_size_of()
    }
}

unsafe impl crate::cxx::UniquePtrVectorElement for Expr {
    fn push_unique(v: Pin<&mut cxx::CxxVector<Self>>, value: cxx::UniquePtr<Self>) {
        ffi::Expr_push_unique(v, value);
    }

    fn pop_unique(v: Pin<&mut cxx::CxxVector<Self>>) -> cxx::UniquePtr<Self> {
        ffi::Expr_pop_unique(v)
    }
}

// UnspecifiedExpr
pub use ffi::UnspecifiedExpr;
unsafe impl Send for UnspecifiedExpr {}
unsafe impl Sync for UnspecifiedExpr {}

// IdentExpr
pub use ffi::IdentExpr;
unsafe impl Send for IdentExpr {}
unsafe impl Sync for IdentExpr {}

// SelectExpr
pub use ffi::SelectExpr;
unsafe impl Send for SelectExpr {}
unsafe impl Sync for SelectExpr {}

// CallExpr
pub use ffi::CallExpr;
unsafe impl Send for CallExpr {}
unsafe impl Sync for CallExpr {}

// ListExpr
pub use ffi::ListExpr;
unsafe impl Send for ListExpr {}
unsafe impl Sync for ListExpr {}

// StructExpr
pub use ffi::StructExpr;
unsafe impl Send for StructExpr {}
unsafe impl Sync for StructExpr {}

// MapExpr
pub use ffi::MapExpr;
unsafe impl Send for MapExpr {}
unsafe impl Sync for MapExpr {}

// ComprehensionExpr
pub use ffi::ComprehensionExpr;
unsafe impl Send for ComprehensionExpr {}
unsafe impl Sync for ComprehensionExpr {}

// ListExprElement
pub use ffi::ListExprElement;
unsafe impl Send for ListExprElement {}
unsafe impl Sync for ListExprElement {}

impl ListExprElement {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::ListExprElement_new()
    }
}

impl SpanElement for ListExprElement {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_ListExprElement");
}

impl MutSpanElement for ListExprElement {
    type TypeId = cxx::type_id!("rust::cel_cxx::MutSpan_ListExprElement");
}

impl crate::SizedExternType for ListExprElement {
    fn size_of() -> usize {
        ffi::ListExprElement_size_of()
    }
}

unsafe impl crate::cxx::UniquePtrVectorElement for ListExprElement {
    fn push_unique(v: Pin<&mut cxx::CxxVector<Self>>, value: cxx::UniquePtr<Self>) {
        ffi::ListExprElement_push_unique(v, value);
    }

    fn pop_unique(v: Pin<&mut cxx::CxxVector<Self>>) -> cxx::UniquePtr<Self> {
        ffi::ListExprElement_pop_unique(v)
    }
}

// StructExprField
pub use ffi::StructExprField;
unsafe impl Send for StructExprField {}
unsafe impl Sync for StructExprField {}

impl StructExprField {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::StructExprField_new()
    }
}

impl SpanElement for StructExprField {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_StructExprField");
}
impl MutSpanElement for StructExprField {
    type TypeId = cxx::type_id!("rust::cel_cxx::MutSpan_StructExprField");
}

impl crate::SizedExternType for StructExprField {
    fn size_of() -> usize {
        ffi::StructExprField_size_of()
    }
}

unsafe impl crate::cxx::UniquePtrVectorElement for StructExprField {
    fn push_unique(v: Pin<&mut cxx::CxxVector<Self>>, value: cxx::UniquePtr<Self>) {
        ffi::StructExprField_push_unique(v, value);
    }

    fn pop_unique(v: Pin<&mut cxx::CxxVector<Self>>) -> cxx::UniquePtr<Self> {
        ffi::StructExprField_pop_unique(v)
    }
}

// MapExprEntry
pub use ffi::MapExprEntry;
unsafe impl Send for MapExprEntry {}
unsafe impl Sync for MapExprEntry {}

impl MapExprEntry {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MapExprEntry_new()
    }
}

impl SpanElement for MapExprEntry {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_MapExprEntry");
}

impl MutSpanElement for MapExprEntry {
    type TypeId = cxx::type_id!("rust::cel_cxx::MutSpan_MapExprEntry");
}

impl crate::SizedExternType for MapExprEntry {
    fn size_of() -> usize {
        ffi::MapExprEntry_size_of()
    }
}

unsafe impl crate::cxx::UniquePtrVectorElement for MapExprEntry {
    fn push_unique(v: Pin<&mut cxx::CxxVector<Self>>, value: cxx::UniquePtr<Self>) {
        ffi::MapExprEntry_push_unique(v, value);
    }

    fn pop_unique(v: Pin<&mut cxx::CxxVector<Self>>) -> cxx::UniquePtr<Self> {
        ffi::MapExprEntry_pop_unique(v)
    }
}
