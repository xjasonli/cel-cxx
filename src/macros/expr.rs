use crate::Constant;

/// A CEL expression node in the abstract syntax tree.
///
/// `Expr` represents a single node in a CEL expression tree during macro expansion.
/// Each expression has a unique ID for source location tracking and an optional kind
/// that determines what type of expression it is (constant, function call, etc.).
///
/// # Structure
///
/// An expression consists of:
/// - `id`: A unique identifier for source position tracking and error reporting
/// - `kind`: The specific type and content of the expression (or `None` for unspecified)
///
/// # Expression Kinds
///
/// The kind field determines the expression type:
/// - [`Constant`]: Literal values (numbers, strings, booleans, etc.)
/// - [`IdentExpr`]: Variable or identifier references
/// - [`SelectExpr`]: Field selection (e.g., `obj.field`)
/// - [`CallExpr`]: Function or method calls
/// - [`ListExpr`]: List literals (e.g., `[1, 2, 3]`)
/// - [`StructExpr`]: Struct literals (e.g., `Person{name: "Alice"}`)
/// - [`MapExpr`]: Map literals (e.g., `{"key": "value"}`)
/// - [`ComprehensionExpr`]: List/map comprehensions
///
/// # Examples
///
/// ```rust,no_run
/// # use cel_cxx::macros::{Expr, ExprKind};
/// # fn example(expr: &Expr) {
/// // Access expression properties
/// let id = expr.id();
/// 
/// // Check expression kind
/// if let Some(kind) = expr.kind() {
///     match kind {
///         ExprKind::Constant(c) => println!("Constant: {:?}", c),
///         ExprKind::Ident(i) => println!("Identifier: {}", i.name),
///         ExprKind::Call(c) => println!("Function: {}", c.function),
///         _ => {}
///     }
/// }
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Expr {
    id: i64,
    kind: Option<Box<ExprKind>>,
}

impl std::ops::Deref for Expr {
    type Target = ExprKind;

    fn deref(&self) -> &Self::Target {
        self.kind.as_ref().unwrap()
    }
}

impl std::ops::DerefMut for Expr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.kind.as_mut().unwrap()
    }
}

impl Expr {
    /// Returns the unique identifier of this expression.
    ///
    /// Expression IDs are used for source location tracking and correlating
    /// expressions with their positions in the original source code.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Sets the unique identifier of this expression.
    ///
    /// This is typically handled by expression factory methods and rarely
    /// needs to be called directly in user code.
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    /// Returns a reference to the expression kind, if set.
    ///
    /// Returns `None` for unspecified expressions (expressions without a kind set).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{Expr, ExprKind};
    /// # fn example(expr: &Expr) {
    /// if let Some(kind) = expr.kind() {
    ///     if kind.is_constant() {
    ///         println!("This is a constant expression");
    ///     }
    /// }
    /// # }
    /// ```
    pub fn kind(&self) -> Option<&ExprKind> {
        self.kind.as_ref()
            .map(|k| k.as_ref())
    }

    /// Sets the expression kind.
    ///
    /// This replaces any existing kind with the new one. Use this to construct
    /// or modify expressions during macro expansion.
    pub fn set_kind(&mut self, kind: ExprKind) {
        self.kind = Some(Box::new(kind));
    }

    /// Clears the expression kind, making this an unspecified expression.
    ///
    /// This is rarely needed in practice; most expressions should have a kind.
    pub fn clear_kind(&mut self) {
        self.kind = None;
    }
}

/// The kind/type of a CEL expression.
///
/// `ExprKind` is an enum representing all possible expression types in CEL.
/// Each variant contains the specific data for that expression type.
///
/// # Variants
///
/// - [`Constant`]: Literal constant values
/// - [`IdentExpr`]: Variable or identifier references
/// - [`SelectExpr`]: Field selection or presence tests
/// - [`CallExpr`]: Function or method calls
/// - [`ListExpr`]: List literal expressions
/// - [`StructExpr`]: Struct literal expressions
/// - [`MapExpr`]: Map literal expressions
/// - [`ComprehensionExpr`]: Comprehension expressions for list/map processing
#[derive(Debug)]
pub enum ExprKind {
    /// A constant literal value.
    Constant(Constant),
    /// An identifier reference.
    Ident(IdentExpr),
    /// A field selection or presence test.
    Select(SelectExpr),
    /// A function or method call.
    Call(CallExpr),
    /// A list literal.
    List(ListExpr),
    /// A struct literal.
    Struct(StructExpr),
    /// A map literal.
    Map(MapExpr),
    /// A comprehension expression.
    Comprehension(ComprehensionExpr),
}

impl ExprKind {
    /// Returns `true` if this is a constant expression.
    pub fn is_constant(&self) -> bool {
        matches!(self, ExprKind::Constant(_))
    }

    /// Returns `true` if this is an identifier expression.
    pub fn is_ident(&self) -> bool {
        matches!(self, ExprKind::Ident(_))
    }

    /// Returns `true` if this is a select expression.
    pub fn is_select(&self) -> bool {
        matches!(self, ExprKind::Select(_))
    }
    
    /// Returns `true` if this is a call expression.
    pub fn is_call(&self) -> bool {
        matches!(self, ExprKind::Call(_))
    }

    /// Returns `true` if this is a list expression.
    pub fn is_list(&self) -> bool {
        matches!(self, ExprKind::List(_))
    }
    
    /// Returns `true` if this is a struct expression.
    pub fn is_struct(&self) -> bool {
        matches!(self, ExprKind::Struct(_))
    }

    /// Returns `true` if this is a map expression.
    pub fn is_map(&self) -> bool {
        matches!(self, ExprKind::Map(_))
    }
    
    /// Returns `true` if this is a comprehension expression.
    pub fn is_comprehension(&self) -> bool {
        matches!(self, ExprKind::Comprehension(_))
    }

    /// Returns a reference to the constant value if this is a constant expression.
    pub fn as_constant(&self) -> Option<&Constant> {
        if let ExprKind::Constant(c) = self {
            Some(c)
        } else {
            None
        }
    }

    /// Returns a reference to the identifier if this is an identifier expression.
    pub fn as_ident(&self) -> Option<&IdentExpr> {
        if let ExprKind::Ident(i) = self {
            Some(i)
        } else {
            None
        }
    }

    /// Returns a reference to the select expression if this is a select expression.
    pub fn as_select(&self) -> Option<&SelectExpr> {
        if let ExprKind::Select(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Returns a reference to the call expression if this is a call expression.
    pub fn as_call(&self) -> Option<&CallExpr> {
        if let ExprKind::Call(c) = self {
            Some(c)
        } else {
            None
        }
    }

    /// Returns a reference to the list expression if this is a list expression.
    pub fn as_list(&self) -> Option<&ListExpr> {
        if let ExprKind::List(l) = self {
            Some(l)
        } else {
            None
        }
    }

    /// Returns a reference to the struct expression if this is a struct expression.
    pub fn as_struct(&self) -> Option<&StructExpr> {
        if let ExprKind::Struct(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Returns a reference to the map expression if this is a map expression.
    pub fn as_map(&self) -> Option<&MapExpr> {
        if let ExprKind::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }

    /// Returns a reference to the comprehension expression if this is a comprehension expression.
    pub fn as_comprehension(&self) -> Option<&ComprehensionExpr> {
        if let ExprKind::Comprehension(c) = self {
            Some(c)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the constant value if this is a constant expression.
    pub fn as_constant_mut(&mut self) -> Option<&mut Constant> {
        if let ExprKind::Constant(c) = self {
            Some(c)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the identifier if this is an identifier expression.
    pub fn as_ident_mut(&mut self) -> Option<&mut IdentExpr> {
        if let ExprKind::Ident(i) = self {
            Some(i)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the select expression if this is a select expression.
    pub fn as_select_mut(&mut self) -> Option<&mut SelectExpr> {
        if let ExprKind::Select(s) = self {
            Some(s)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the call expression if this is a call expression.
    pub fn as_call_mut(&mut self) -> Option<&mut CallExpr> {
        if let ExprKind::Call(c) = self {
            Some(c)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the list expression if this is a list expression.
    pub fn as_list_mut(&mut self) -> Option<&mut ListExpr> {
        if let ExprKind::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the struct expression if this is a struct expression.
    pub fn as_struct_mut(&mut self) -> Option<&mut StructExpr> {
        if let ExprKind::Struct(s) = self {
            Some(s)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the map expression if this is a map expression.
    pub fn as_map_mut(&mut self) -> Option<&mut MapExpr> {
        if let ExprKind::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }
    
    /// Returns a mutable reference to the comprehension expression if this is a comprehension expression.
    pub fn as_comprehension_mut(&mut self) -> Option<&mut ComprehensionExpr> {
        if let ExprKind::Comprehension(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

/// An identifier expression referencing a variable or constant.
///
/// Identifier expressions represent variable names in CEL, such as `x`, `my_var`,
/// or `request.path`.
#[derive(Debug)]
pub struct IdentExpr {
    /// The name of the identifier.
    pub name: String,
}

/// A field selection expression.
///
/// Select expressions access fields on objects or maps. They can represent either:
/// - Field access: `obj.field` (when `test_only` is `false`)
/// - Presence test: `has(obj.field)` (when `test_only` is `true`)
#[derive(Debug)]
pub struct SelectExpr {
    /// The expression being selected from (the object).
    pub operand: Expr,
    /// The name of the field being selected.
    pub field: String,
    /// If `true`, this is a presence test rather than field access.
    pub test_only: bool,
}

/// A function or method call expression.
///
/// Call expressions represent both global function calls and method calls.
/// For global calls, the `target` field is an unspecified expression.
#[derive(Debug)]
pub struct CallExpr {
    /// The name of the function being called.
    pub function: String,
    /// The target expression for method calls (empty for global functions).
    pub target: Expr,
    /// The arguments passed to the function.
    pub args: Vec<Expr>,
}

/// An element in a list expression.
///
/// List elements wrap expressions with metadata about whether they are optional.
#[derive(Debug)]
pub struct ListExprElement {
    /// The expression value of this element.
    pub expr: Expr,
    /// If `true`, this element is omitted if the expression is undefined.
    pub optional: bool,
}

/// A list literal expression.
///
/// List expressions create CEL list values, e.g., `[1, 2, 3]`.
#[derive(Debug)]
pub struct ListExpr {
    /// The elements of the list.
    pub elements: Vec<ListExprElement>,
}

/// A field in a struct expression.
///
/// Struct fields consist of a name, value expression, and optional flag.
#[derive(Debug)]
pub struct StructExprField {
    /// The unique ID of this field for source tracking.
    pub id: i64,
    /// The name of the field.
    pub name: String,
    /// The expression providing the field value.
    pub value: Expr,
    /// If `true`, this field is omitted if the value is undefined.
    pub optional: bool,
}

/// A struct literal expression.
///
/// Struct expressions create structured objects with named fields,
/// e.g., `Person{name: "Alice", age: 30}`.
#[derive(Debug)]
pub struct StructExpr {
    /// The name of the struct type.
    pub name: String,
    /// The fields of the struct.
    pub fields: Vec<StructExprField>,
}

/// An entry in a map expression.
///
/// Map entries consist of key and value expressions, plus an optional flag.
#[derive(Debug)]
pub struct MapExprEntry {
    /// The unique ID of this entry for source tracking.
    pub id: i64,
    /// The expression providing the map key.
    pub key: Expr,
    /// The expression providing the map value.
    pub value: Expr,
    /// If `true`, this entry is omitted if key or value is undefined.
    pub optional: bool,
}

/// A map literal expression.
///
/// Map expressions create CEL map values, e.g., `{"key": "value", "foo": "bar"}`.
#[derive(Debug)]
pub struct MapExpr {
    /// The entries of the map.
    pub entries: Vec<MapExprEntry>,
}

/// A comprehension expression for list/map processing.
///
/// Comprehensions are CEL's powerful iteration construct, similar to list
/// comprehensions in Python. They consist of iteration, accumulation, and
/// result computation phases.
///
/// # Structure
///
/// A comprehension has:
/// - One or two iteration variables that range over a collection
/// - An accumulator variable that maintains state across iterations
/// - A loop condition that determines when to continue
/// - A loop step that updates the accumulator
/// - A result expression that produces the final value
///
/// # Examples
///
/// Single variable: `[x * 2 | x in [1, 2, 3]]`
/// - `iter_var`: `"x"`
/// - `iter_range`: `[1, 2, 3]`
/// - Result: `[2, 4, 6]`
///
/// Two variables (map iteration): `[k | k, v in {"a": 1, "b": 2}, v > 1]`
/// - `iter_var`: `"k"` (key)
/// - `iter_var2`: `"v"` (value)
/// - `iter_range`: `{"a": 1, "b": 2}`
/// - Result: `["b"]`
#[derive(Debug)]
pub struct ComprehensionExpr {
    /// The first iteration variable name (required).
    pub iter_var: String,
    /// The second iteration variable name (empty string if not used).
    pub iter_var2: String,
    /// The expression providing the collection to iterate over.
    pub iter_range: Expr,
    /// The accumulator variable name.
    pub accu_var: String,
    /// The initial value of the accumulator.
    pub accu_init: Expr,
    /// The condition to continue iterating (often just `true`).
    pub loop_condition: Expr,
    /// The expression to update the accumulator each iteration.
    pub loop_step: Expr,
    /// The final expression computed from the accumulator.
    pub result: Expr,
}

impl From<&Expr> for cxx::UniquePtr<crate::ffi::Expr> {
    fn from(value: &Expr) -> Self {
        use crate::ffi::Expr as FfiExpr;
        use crate::ffi::Constant as FfiConstant;
        use crate::ffi::CxxVectorExt;

        let mut ffi_expr = FfiExpr::new();
        ffi_expr.pin_mut().set_id(value.id());
        match value.kind() {
            None => {}
            Some(ExprKind::Constant(constant)) => {
                let ffi_constant: cxx::UniquePtr<FfiConstant> = constant.into();
                ffi_expr.pin_mut()
                    .set_const_expr(ffi_constant);
            }
            Some(ExprKind::Ident(ident)) => {
                let mut ident_expr = ffi_expr.pin_mut().ident_expr_mut();
                ident_expr.as_mut().set_name(ident.name.as_str().into());
            }
            Some(ExprKind::Select(select)) => {
                let mut select_expr = ffi_expr.pin_mut().select_expr_mut();
                select_expr.as_mut().set_operand((&select.operand).into());
                select_expr.as_mut().set_field(select.field.as_str().into());
                select_expr.as_mut().set_test_only(select.test_only);
            }
            Some(ExprKind::Call(call)) => {
                let mut call_expr = ffi_expr.pin_mut().call_expr_mut();
                call_expr.as_mut().set_function(call.function.as_str().into());
                call_expr.as_mut().set_target((&call.target).into());
                let mut args = call_expr.as_mut().args_mut();
                for arg in &call.args {
                    args.as_mut().push_unique(arg.into());
                }
            }
            Some(ExprKind::List(list)) => {
                let mut list_expr = ffi_expr.pin_mut().list_expr_mut();
                let mut elements = list_expr.as_mut().elements_mut();
                for element in &list.elements {
                    elements.as_mut().push_unique(element.into());
                }
            }
            Some(ExprKind::Struct(struct_)) => {
                let mut struct_expr = ffi_expr.pin_mut().struct_expr_mut();
                let mut fields = struct_expr.as_mut().fields_mut();
                for field in &struct_.fields {
                    fields.as_mut().push_unique(field.into());
                }
            }
            Some(ExprKind::Map(map)) => {
                let mut map_expr = ffi_expr.pin_mut().map_expr_mut();
                let mut entries = map_expr.as_mut().entries_mut();
                for entry in &map.entries {
                    entries.as_mut().push_unique(entry.into());
                }
            }
            Some(ExprKind::Comprehension(comprehension)) => {
                let mut comprehension_expr = ffi_expr.pin_mut().comprehension_expr_mut();
                comprehension_expr.as_mut().set_iter_var(comprehension.iter_var.as_str().into());
                comprehension_expr.as_mut().set_iter_var2(comprehension.iter_var2.as_str().into());
                comprehension_expr.as_mut().set_iter_range((&comprehension.iter_range).into());
                comprehension_expr.as_mut().set_accu_var(comprehension.accu_var.as_str().into());
                comprehension_expr.as_mut().set_accu_init((&comprehension.accu_init).into());
                comprehension_expr.as_mut().set_loop_condition((&comprehension.loop_condition).into());
                comprehension_expr.as_mut().set_loop_step((&comprehension.loop_step).into());
                comprehension_expr.as_mut().set_result((&comprehension.result).into());
            }
        }
        ffi_expr
    }
}

impl From<Expr> for cxx::UniquePtr<crate::ffi::Expr> {
    fn from(value: Expr) -> Self {
        Self::from(&value)
    }
}

impl From<&crate::ffi::Expr> for Expr {
    fn from(value: &crate::ffi::Expr) -> Self {
        use crate::ffi::ExprKindCase as FfiExprKindCase;

        let mut expr = Expr::default();
        expr.set_id(value.id());

        let kind = match value.kind_case() {
            FfiExprKindCase::Unspecified => None,
            FfiExprKindCase::Constant => Some(ExprKind::Constant(Constant::from(value.const_expr()))),
            FfiExprKindCase::Ident => Some(ExprKind::Ident(IdentExpr::from(value.ident_expr()))),
            FfiExprKindCase::Select => Some(ExprKind::Select(SelectExpr::from(value.select_expr()))),
            FfiExprKindCase::Call => Some(ExprKind::Call(CallExpr::from(value.call_expr()))),
            FfiExprKindCase::List => Some(ExprKind::List(ListExpr::from(value.list_expr()))),
            FfiExprKindCase::Struct => Some(ExprKind::Struct(StructExpr::from(value.struct_expr()))),
            FfiExprKindCase::Map => Some(ExprKind::Map(MapExpr::from(value.map_expr()))),
            FfiExprKindCase::Comprehension => Some(ExprKind::Comprehension(ComprehensionExpr::from(value.comprehension_expr()))),
        };

        if let Some(kind) = kind {
            expr.set_kind(kind);
        }
        expr
    }
}

impl From<cxx::UniquePtr<crate::ffi::Expr>> for Expr {
    fn from(value: cxx::UniquePtr<crate::ffi::Expr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::IdentExpr> for IdentExpr {
    fn from(value: &crate::ffi::IdentExpr) -> Self {
        IdentExpr {
            name: value.name().to_string(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::IdentExpr>> for IdentExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::IdentExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::SelectExpr> for SelectExpr {
    fn from(value: &crate::ffi::SelectExpr) -> Self {
        SelectExpr {
            operand: Expr::from(value.operand()),
            field: value.field().to_string(),
            test_only: value.test_only(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::SelectExpr>> for SelectExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::SelectExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::CallExpr> for CallExpr {
    fn from(value: &crate::ffi::CallExpr) -> Self {
        CallExpr {
            function: value.function().to_string(),
            target: Expr::from(value.target()),
            args: value.args()
                .into_iter()
                .map(|arg| Expr::from(arg))
                .collect(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::CallExpr>> for CallExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::CallExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::ListExpr> for ListExpr {
    fn from(value: &crate::ffi::ListExpr) -> Self {
        ListExpr {
            elements: value.elements()
                .into_iter()
                .map(|element| ListExprElement::from(element))
                .collect(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::ListExpr>> for ListExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::ListExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::StructExpr> for StructExpr {
    fn from(value: &crate::ffi::StructExpr) -> Self {
        StructExpr {
            name: value.name().to_string(),
            fields: value.fields()
                .into_iter()
                .map(|field| StructExprField::from(field))
                .collect(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::StructExpr>> for StructExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::StructExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::MapExpr> for MapExpr {
    fn from(value: &crate::ffi::MapExpr) -> Self {
        MapExpr {
            entries: value.entries()
                .into_iter()
                .map(|entry| MapExprEntry::from(entry))
                .collect(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::MapExpr>> for MapExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::MapExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&crate::ffi::ComprehensionExpr> for ComprehensionExpr {
    fn from(value: &crate::ffi::ComprehensionExpr) -> Self {
        ComprehensionExpr {
            iter_var: value.iter_var().to_string(),
            iter_var2: value.iter_var2().to_string(),
            iter_range: Expr::from(value.iter_range()),
            accu_var: value.accu_var().to_string(),
            accu_init: Expr::from(value.accu_init()),
            loop_condition: Expr::from(value.loop_condition()),
            loop_step: Expr::from(value.loop_step()),
            result: Expr::from(value.result()),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::ComprehensionExpr>> for ComprehensionExpr {
    fn from(value: cxx::UniquePtr<crate::ffi::ComprehensionExpr>) -> Self {
        Self::from(&*value)
    }
}

impl From<&ListExprElement> for cxx::UniquePtr<crate::ffi::ListExprElement> {
    fn from(value: &ListExprElement) -> Self {
        use crate::ffi::ListExprElement as FfiListExprElement;

        let mut ffi_element = FfiListExprElement::new();
        ffi_element.pin_mut().set_expr((&value.expr).into());
        ffi_element.pin_mut().set_optional(value.optional);
        ffi_element
    }
}

impl From<ListExprElement> for cxx::UniquePtr<crate::ffi::ListExprElement> {
    fn from(element: ListExprElement) -> Self {
        Self::from(&element)
    }
}

impl From<&crate::ffi::ListExprElement> for ListExprElement {
    fn from(value: &crate::ffi::ListExprElement) -> Self {
        ListExprElement {
            expr: Expr::from(value.expr()),
            optional: value.optional(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::ListExprElement>> for ListExprElement {
    fn from(value: cxx::UniquePtr<crate::ffi::ListExprElement>) -> Self {
        Self::from(&*value)
    }
}

impl From<&StructExprField> for cxx::UniquePtr<crate::ffi::StructExprField> {
    fn from(value: &StructExprField) -> Self {
        use crate::ffi::StructExprField as FfiStructExprField;

        let mut ffi_field = FfiStructExprField::new();
        ffi_field.pin_mut().set_id(value.id);
        ffi_field.pin_mut().set_name(value.name.as_str().into());
        ffi_field.pin_mut().set_value((&value.value).into());
        ffi_field.pin_mut().set_optional(value.optional);
        ffi_field
    }
}

impl From<StructExprField> for cxx::UniquePtr<crate::ffi::StructExprField> {
    fn from(value: StructExprField) -> Self {
        Self::from(&value)
    }
}

impl From<&crate::ffi::StructExprField> for StructExprField {
    fn from(value: &crate::ffi::StructExprField) -> Self {
        StructExprField {
            id: value.id(),
            name: value.name().to_string(),
            value: Expr::from(value.value()),
            optional: value.optional(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::StructExprField>> for StructExprField {
    fn from(value: cxx::UniquePtr<crate::ffi::StructExprField>) -> Self {
        Self::from(&*value)
    }
}

impl From<&MapExprEntry> for cxx::UniquePtr<crate::ffi::MapExprEntry> {
    fn from(value: &MapExprEntry) -> Self {
        use crate::ffi::MapExprEntry as FfiMapExprEntry;
        let mut ffi_entry = FfiMapExprEntry::new();
        ffi_entry.pin_mut().set_id(value.id);
        ffi_entry.pin_mut().set_key((&value.key).into());
        ffi_entry.pin_mut().set_value((&value.value).into());
        ffi_entry.pin_mut().set_optional(value.optional);
        ffi_entry
    }
}

impl From<MapExprEntry> for cxx::UniquePtr<crate::ffi::MapExprEntry> {
    fn from(value: MapExprEntry) -> Self {
        Self::from(&value)
    }
}

impl From<&crate::ffi::MapExprEntry> for MapExprEntry {
    fn from(value: &crate::ffi::MapExprEntry) -> Self {
        MapExprEntry {
            id: value.id(),
            key: Expr::from(value.key()),
            value: Expr::from(value.value()),
            optional: value.optional(),
        }
    }
}

impl From<cxx::UniquePtr<crate::ffi::MapExprEntry>> for MapExprEntry {
    fn from(value: cxx::UniquePtr<crate::ffi::MapExprEntry>) -> Self {
        Self::from(&*value)
    }
}
