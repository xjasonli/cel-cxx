use super::expr::*;
use crate::ffi::{
    Expr as FfiExpr,
    ListExprElement as FfiListExprElement,
    StructExprField as FfiStructExprField,
    MapExprEntry as FfiMapExprEntry,
    MacroExprFactory as FfiMacroExprFactory,
};
use std::pin::Pin;

/// Factory for constructing CEL expressions during macro expansion.
///
/// `MacroExprFactory` provides a safe interface for building new expression trees
/// within macro expanders. It handles the low-level details of creating properly
/// structured expressions with unique IDs and correct parent-child relationships.
///
/// # Usage
///
/// This type is provided to macro expander functions and should not be constructed
/// directly. All expression construction during macro expansion must go through
/// this factory to ensure correctness.
///
/// # Expression IDs
///
/// The factory automatically assigns unique IDs to all created expressions. These
/// IDs are used for source location tracking and error reporting.
///
/// # Examples
///
/// ```rust,no_run
/// # use cel_cxx::macros::{MacroExprFactory, Expr};
/// fn example_expander(factory: &mut MacroExprFactory, args: Vec<Expr>) -> Option<Expr> {
///     // Create a constant expression
///     let const_expr = factory.new_const(42);
///     
///     // Create a function call expression
///     let call_expr = factory.new_call("size", &[args[0].clone()]);
///     
///     // Create a binary operation
///     let result = factory.new_call("_>_", &[call_expr, const_expr]);
///     
///     Some(result)
/// }
/// ```
pub struct MacroExprFactory<'a>(pub(crate) std::sync::RwLock<Pin<&'a mut FfiMacroExprFactory>>);

impl<'a> MacroExprFactory<'a> {
    pub(crate) fn new(ffi_macro_expr_factory: Pin<&'a mut FfiMacroExprFactory>) -> Self {
        Self(std::sync::RwLock::new(ffi_macro_expr_factory))
    }

    /// Creates a deep copy of an expression with new IDs.
    ///
    /// This method duplicates the entire expression tree, assigning fresh IDs to
    /// all nodes. This is useful when you need to reuse an expression structure
    /// in multiple places within a macro expansion.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory, expr: &Expr) {
    /// // Create two independent copies of an expression
    /// let copy1 = factory.copy_expr(expr);
    /// let copy2 = factory.copy_expr(expr);
    /// # }
    /// ```
    pub fn copy_expr(&self, expr: &Expr) -> Expr {
        let ffi_expr: cxx::UniquePtr<FfiExpr> = expr.into();
        let ffi_copied = self.0.write().expect("Failed to write to RwLock")
            .as_mut().copy(&ffi_expr);
        ffi_copied.into()
    }

    /// Creates a deep copy of a list element with a new ID.
    ///
    /// Copies a list element, including its expression and optional flag,
    /// assigning a fresh ID to the element.
    pub fn copy_list_expr_element(&self, list_expr_element: &ListExprElement) -> ListExprElement {
        let ffi_list_expr_element: cxx::UniquePtr<FfiListExprElement> = list_expr_element.into();
        let ffi_copied = self.0.write().expect("Failed to write to RwLock")
            .as_mut().copy_list_element(&ffi_list_expr_element);
        ffi_copied.into()
    }

    /// Creates a deep copy of a struct field with a new ID.
    ///
    /// Copies a struct field, including its name, value expression, and optional flag,
    /// assigning a fresh ID to the field.
    pub fn copy_struct_expr_field(&self, struct_expr_field: &StructExprField) -> StructExprField {
        let ffi_struct_expr_field: cxx::UniquePtr<FfiStructExprField> = struct_expr_field.into();
        let ffi_copied = self.0.write().expect("Failed to write to RwLock")
            .as_mut().copy_struct_field(&ffi_struct_expr_field);
        ffi_copied.into()
    }

    /// Creates a deep copy of a map entry with a new ID.
    ///
    /// Copies a map entry, including its key, value expressions, and optional flag,
    /// assigning a fresh ID to the entry.
    pub fn copy_map_expr_entry(&self, map_expr_entry: &MapExprEntry) -> MapExprEntry {
        let ffi_map_expr_entry: cxx::UniquePtr<FfiMapExprEntry> = map_expr_entry.into();
        let ffi_copied = self.0.write().expect("Failed to write to RwLock")
            .as_mut().copy_map_entry(&ffi_map_expr_entry);
        ffi_copied.into()
    }

    /// Creates an unspecified expression.
    ///
    /// An unspecified expression is an empty expression placeholder with no kind set.
    /// This is rarely needed in user code; prefer using more specific constructors.
    pub fn new_unspecified(&mut self) -> Expr {
        let ffi_unspecified = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_unspecified();
        ffi_unspecified.into()
    }

    /// Creates a constant expression from any value that can be converted to a CEL constant.
    ///
    /// This is the primary way to create literal values in macro expansions. The value
    /// must implement [`IntoConstant`](crate::IntoConstant).
    ///
    /// # Supported Types
    ///
    /// - Integers: `i64`, `i32`, `u64`, etc.
    /// - Floating point: `f64`, `f32`
    /// - Boolean: `bool`
    /// - String: `String`, `&str`
    /// - Bytes: `Vec<u8>`, `&[u8]`
    /// - Null: `()`, `Option::None`
    /// - Duration: `std::time::Duration`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let int_expr = factory.new_const(42i64);
    /// let str_expr = factory.new_const("hello");
    /// let bool_expr = factory.new_const(true);
    /// let null_expr = factory.new_const(());
    /// # }
    /// ```
    pub fn new_const<T>(&self, value: T) -> Expr
    where
        T: crate::IntoConstant,
    {
        let mut expr = Expr::from(self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_unspecified());
        expr.set_kind(ExprKind::Constant(value.into_constant()));
        expr
    }

    /// Creates an identifier expression.
    ///
    /// Identifier expressions reference variables or constants by name. This is used
    /// to create variable references in the expanded macro.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let var_expr = factory.new_ident("my_variable");
    /// let param_expr = factory.new_ident("x");
    /// # }
    /// ```
    pub fn new_ident(&self, name: &str) -> Expr {
        let ffi_ident = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_ident(name.into());
        ffi_ident.into()
    }

    /// Returns the name of the accumulator variable for comprehensions.
    ///
    /// When building comprehension expressions, this method provides the name of the
    /// automatically generated accumulator variable. This is useful for maintaining
    /// consistency with the compiler's internal naming scheme.
    pub fn accu_var_name(&self) -> String {
        let mut guard = self.0.write().expect("Failed to write to RwLock");
        guard.as_mut().accu_var_name().to_string_lossy().to_string()
    }

    /// Creates an identifier expression for the accumulator variable.
    ///
    /// This is a convenience method that combines [`accu_var_name`](#method.accu_var_name)
    /// and [`new_ident`](#method.new_ident) to create a reference to the accumulator
    /// variable used in comprehensions.
    pub fn new_accu_ident(&self) -> Expr {
        let ffi_accu_ident = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_accu_ident();
        ffi_accu_ident.into()
    }

    /// Creates a field selection expression.
    ///
    /// A select expression accesses a field on an object or map, e.g., `obj.field`.
    /// This is equivalent to the dot operator in CEL.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let obj = factory.new_ident("my_object");
    /// let field_access = factory.new_select(&obj, "field_name");
    /// // Results in: my_object.field_name
    /// # }
    /// ```
    pub fn new_select(&self, operand: &Expr, field: &str) -> Expr {
        let ffi_select = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_select(operand.into(), field.into());
        ffi_select.into()
    }

    /// Creates a field presence test expression.
    ///
    /// A presence test checks whether a field exists on an object without accessing
    /// its value. This is equivalent to the `has(obj.field)` macro in CEL.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let obj = factory.new_ident("my_object");
    /// let has_field = factory.new_presence_test(&obj, "optional_field");
    /// // Results in: has(my_object.optional_field)
    /// # }
    /// ```
    pub fn new_presence_test(&self, operand: &Expr, field: &str) -> Expr {
        let ffi_presence_test = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_presence_test(operand.into(), field.into());
        ffi_presence_test.into()
    }

    /// Creates a function call expression.
    ///
    /// Function calls invoke named functions with the provided arguments. This is used
    /// for both built-in operators (like `_+_`, `_==_`) and user-defined functions.
    ///
    /// # Common Operators
    ///
    /// CEL operators are represented as functions with special names:
    /// - Arithmetic: `_+_`, `_-_`, `_*_`, `_/_`, `_%_`, `_-_` (unary negation)
    /// - Comparison: `_==_`, `_!=_`, `_<_`, `_<=_`, `_>_`, `_>=_`
    /// - Logical: `_&&_`, `_||_`, `!_` (negation)
    /// - Ternary: `_?_:_` (conditional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let a = factory.new_const(10);
    /// let b = factory.new_const(20);
    /// let sum = factory.new_call("_+_", &[a, b]);
    /// // Results in: 10 + 20
    /// # }
    /// ```
    pub fn new_call(&self, function: &str, args: &[Expr]) -> Expr {
        let mut ffi_args = cxx::CxxVector::new();
        for arg in args {
            use crate::ffi::CxxVectorExt;
            ffi_args.pin_mut().push_unique(arg.into());
        }
        let ffi_call = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_call(function.into(), ffi_args);
        ffi_call.into()
    }

    /// Creates a member function call expression.
    ///
    /// Member calls invoke methods on a target object, e.g., `target.method(args)`.
    /// This is the method-call form as opposed to global function calls.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let list = factory.new_ident("my_list");
    /// let index = factory.new_const(0);
    /// let get_call = factory.new_member_call("get", &list, &[index]);
    /// // Results in: my_list.get(0)
    /// # }
    /// ```
    pub fn new_member_call(&self, function: &str, target: &Expr, args: &[Expr]) -> Expr {
        let mut ffi_args = cxx::CxxVector::new();
        for arg in args {
            use crate::ffi::CxxVectorExt;
            ffi_args.pin_mut().push_unique(arg.into());
        }
        let ffi_member_call = self.0.write().expect("Failed to write to RwLock")
            .as_mut()
            .new_member_call(function.into(), target.into(), ffi_args);
        ffi_member_call.into()
    }

    /// Creates a list element for use in list construction.
    ///
    /// List elements wrap expressions with an optional flag. The optional flag is
    /// used for optional element syntax in CEL (e.g., `[1, ?2, 3]` where `?2` might
    /// be omitted if undefined).
    ///
    /// # Parameters
    ///
    /// - `expr`: The expression value of this list element
    /// - `optional`: If `true`, this element can be omitted if the expression is undefined
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let required = factory.new_list_element(&factory.new_const(1), false);
    /// let optional = factory.new_list_element(&factory.new_ident("maybe"), true);
    /// # }
    /// ```
    pub fn new_list_element(&self, expr: &Expr, optional: bool) -> ListExprElement {
        let ffi_list_element = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_list_element(expr.into(), optional);
        ffi_list_element.into()
    }

    /// Creates a list literal expression.
    ///
    /// List expressions create CEL list values, e.g., `[1, 2, 3]`. Each element
    /// can be marked as optional for conditional inclusion.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let elem1 = factory.new_list_element(&factory.new_const(1), false);
    /// let elem2 = factory.new_list_element(&factory.new_const(2), false);
    /// let elem3 = factory.new_list_element(&factory.new_const(3), false);
    /// let list = factory.new_list(&[elem1, elem2, elem3]);
    /// // Results in: [1, 2, 3]
    /// # }
    /// ```
    pub fn new_list(&self, elements: &[ListExprElement]) -> Expr {
        let mut ffi_elements = cxx::CxxVector::new();
        for element in elements {
            use crate::ffi::CxxVectorExt;
            ffi_elements.pin_mut().push_unique(element.into());
        }
        let ffi_list = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_list(ffi_elements);
        ffi_list.into()
    }
    
    /// Creates a struct field for use in struct construction.
    ///
    /// Struct fields consist of a field name, value expression, and optional flag.
    /// The optional flag allows fields to be omitted if their value is undefined.
    ///
    /// # Parameters
    ///
    /// - `name`: The field name
    /// - `value`: The expression providing the field value
    /// - `optional`: If `true`, this field can be omitted if the value is undefined
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let field = factory.new_struct_field(
    ///     "name",
    ///     &factory.new_const("Alice"),
    ///     false
    /// );
    /// # }
    /// ```
    pub fn new_struct_field(&self, name: &str, value: &Expr, optional: bool) -> StructExprField {
        let ffi_struct_field = self.0.write().expect("Failed to write to RwLock")
            .as_mut()
            .new_struct_field(name.into(), value.into(), optional);
        ffi_struct_field.into()
    }

    /// Creates a struct literal expression.
    ///
    /// Struct expressions create structured objects with named fields, e.g.,
    /// `Person{name: "Alice", age: 30}`. The struct name is typically a message
    /// type name.
    ///
    /// # Parameters
    ///
    /// - `name`: The struct type name (e.g., `"Person"`, `"com.example.Message"`)
    /// - `fields`: The fields of the struct
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let name_field = factory.new_struct_field(
    ///     "name",
    ///     &factory.new_const("Alice"),
    ///     false
    /// );
    /// let age_field = factory.new_struct_field(
    ///     "age",
    ///     &factory.new_const(30),
    ///     false
    /// );
    /// let person = factory.new_struct("Person", &[name_field, age_field]);
    /// // Results in: Person{name: "Alice", age: 30}
    /// # }
    /// ```
    pub fn new_struct(&self, name: &str, fields: &[StructExprField]) -> Expr {
        let mut ffi_fields = cxx::CxxVector::new();
        for field in fields {
            use crate::ffi::CxxVectorExt;
            ffi_fields.pin_mut().push_unique(field.into());
        }
        let ffi_struct = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_struct(name.into(), ffi_fields);
        ffi_struct.into()
    }
    
    /// Creates a map entry for use in map construction.
    ///
    /// Map entries consist of key and value expressions, plus an optional flag.
    /// The optional flag allows entries to be omitted if either key or value is undefined.
    ///
    /// # Parameters
    ///
    /// - `key`: The expression providing the map key
    /// - `value`: The expression providing the map value
    /// - `optional`: If `true`, this entry can be omitted if key or value is undefined
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let entry = factory.new_map_entry(
    ///     &factory.new_const("key"),
    ///     &factory.new_const("value"),
    ///     false
    /// );
    /// # }
    /// ```
    pub fn new_map_entry(&self, key: &Expr, value: &Expr, optional: bool) -> MapExprEntry {
        let ffi_map_entry = self.0.write().expect("Failed to write to RwLock")
            .as_mut()
            .new_map_entry(key.into(), value.into(), optional);
        ffi_map_entry.into()
    }

    /// Creates a map literal expression.
    ///
    /// Map expressions create CEL map values, e.g., `{"key": "value", "foo": "bar"}`.
    /// Each entry can be marked as optional for conditional inclusion.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory) {
    /// let entry1 = factory.new_map_entry(
    ///     &factory.new_const("name"),
    ///     &factory.new_const("Alice"),
    ///     false
    /// );
    /// let entry2 = factory.new_map_entry(
    ///     &factory.new_const("age"),
    ///     &factory.new_const(30),
    ///     false
    /// );
    /// let map = factory.new_map(&[entry1, entry2]);
    /// // Results in: {"name": "Alice", "age": 30}
    /// # }
    /// ```
    pub fn new_map(&self, entries: &[MapExprEntry]) -> Expr {
        let mut ffi_entries = cxx::CxxVector::new();
        for entry in entries {
            use crate::ffi::CxxVectorExt;
            ffi_entries.pin_mut().push_unique(entry.into());
        }
        let ffi_map = self.0.write().expect("Failed to write to RwLock")
            .as_mut().new_map(ffi_entries);
        ffi_map.into()
    }
    
    
    /// Creates a comprehension expression with a single iteration variable.
    ///
    /// Comprehensions are CEL's powerful list processing construct, similar to list
    /// comprehensions in Python. They consist of:
    /// - An iteration variable that ranges over a collection
    /// - An accumulator variable that maintains state across iterations
    /// - A loop condition that determines when to continue
    /// - A loop step that updates the accumulator
    /// - A result expression that produces the final value
    ///
    /// # Parameters
    ///
    /// - `iter_var`: Name of the iteration variable (e.g., `"x"`)
    /// - `iter_range`: Expression providing the collection to iterate over
    /// - `accu_var`: Name of the accumulator variable (e.g., `"__result__"`)
    /// - `accu_init`: Initial value of the accumulator
    /// - `loop_condition`: Condition to continue iterating (e.g., `true` for all elements)
    /// - `loop_step`: Expression to update the accumulator each iteration
    /// - `result`: Final expression computed from the accumulator
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory, list: Expr) {
    /// // Comprehension to filter a list: [x | x in list, x > 0]
    /// let iter_var = "x";
    /// let accu_var = factory.accu_var_name();
    /// let iter_range = list;
    /// let accu_init = factory.new_list(&[]);
    /// let loop_condition = factory.new_call("_>_", &[
    ///     factory.new_ident(iter_var),
    ///     factory.new_const(0)
    /// ]);
    /// let loop_step = factory.new_call("_+_", &[
    ///     factory.new_accu_ident(),
    ///     factory.new_list(&[factory.new_list_element(
    ///         &factory.new_ident(iter_var),
    ///         false
    ///     )])
    /// ]);
    /// let result = factory.new_accu_ident();
    /// 
    /// let comprehension = factory.new_comprehension(
    ///     iter_var,
    ///     &iter_range,
    ///     &accu_var,
    ///     &accu_init,
    ///     &loop_condition,
    ///     &loop_step,
    ///     &result
    /// );
    /// # }
    /// ```
    pub fn new_comprehension(
        &self, 
        iter_var: &str, 
        iter_range: &Expr, 
        accu_var: &str, 
        accu_init: &Expr, 
        loop_condition: &Expr, 
        loop_step: &Expr, 
        result: &Expr) -> Expr {
        let ffi_iter_var = iter_var.into();
        let ffi_iter_range = iter_range.into();
        let ffi_accu_var = accu_var.into();
        let ffi_accu_init = accu_init.into();
        let ffi_loop_condition = loop_condition.into();
        let ffi_loop_step = loop_step.into();
        let ffi_result = result.into();
        let ffi_comprehension = self.0.write().expect("Failed to write to RwLock")
            .as_mut()
            .new_comprehension(
                ffi_iter_var,
                ffi_iter_range,
                ffi_accu_var,
                ffi_accu_init,
                ffi_loop_condition,
                ffi_loop_step,
                ffi_result
            );
        ffi_comprehension.into()
    }

    /// Creates a comprehension expression with two iteration variables.
    ///
    /// This variant of comprehension supports iterating over key-value pairs, such
    /// as when processing maps. The first iteration variable represents keys, and
    /// the second represents values.
    ///
    /// # Parameters
    ///
    /// - `iter_var`: Name of the first iteration variable (e.g., key in maps)
    /// - `iter_var2`: Name of the second iteration variable (e.g., value in maps)
    /// - `iter_range`: Expression providing the collection to iterate over
    /// - `accu_var`: Name of the accumulator variable
    /// - `accu_init`: Initial value of the accumulator
    /// - `loop_condition`: Condition to continue iterating
    /// - `loop_step`: Expression to update the accumulator each iteration
    /// - `result`: Final expression computed from the accumulator
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory, map: Expr) {
    /// // Comprehension to filter map entries: {k: v | k, v in map, v > 0}
    /// let comprehension = factory.new_comprehension2(
    ///     "k",
    ///     "v",
    ///     &map,
    ///     &factory.accu_var_name(),
    ///     &factory.new_map(&[]),
    ///     &factory.new_call("_>_", &[factory.new_ident("v"), factory.new_const(0)]),
    ///     &factory.new_call("_+_", &[
    ///         factory.new_accu_ident(),
    ///         factory.new_map(&[factory.new_map_entry(
    ///             &factory.new_ident("k"),
    ///             &factory.new_ident("v"),
    ///             false
    ///         )])
    ///     ]),
    ///     &factory.new_accu_ident()
    /// );
    /// # }
    /// ```
    pub fn new_comprehension2(
        &self, 
        iter_var: &str, 
        iter_var2: &str, 
        iter_range: &Expr, 
        accu_var: &str, 
        accu_init: &Expr, 
        loop_condition: &Expr, 
        loop_step: &Expr, 
        result: &Expr) -> Expr {
        let ffi_iter_var = iter_var.into();
        let ffi_iter_var2 = iter_var2.into();
        let ffi_iter_range = iter_range.into();
        let ffi_accu_var = accu_var.into();
        let ffi_accu_init = accu_init.into();
        let ffi_loop_condition = loop_condition.into();
        let ffi_loop_step = loop_step.into();
        let ffi_result = result.into();
        let ffi_comprehension = self.0.write().expect("Failed to write to RwLock")
            .as_mut()
            .new_comprehension2(
                ffi_iter_var,
                ffi_iter_var2,
                ffi_iter_range,
                ffi_accu_var,
                ffi_accu_init,
                ffi_loop_condition,
                ffi_loop_step,
                ffi_result
            );
        ffi_comprehension.into()
    }
    
    /// Reports a compilation error at the current location.
    ///
    /// This method creates an error expression that will cause compilation to fail
    /// with the specified error message. Use this when a macro detects invalid
    /// usage or unsupported patterns.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory, args: Vec<Expr>) {
    /// if args.len() != 2 {
    ///     // Return an error if argument count is wrong
    ///     let error = factory.report_error("expected exactly 2 arguments");
    ///     return;
    /// }
    /// # }
    /// ```
    pub fn report_error(&self, message: &str) -> Expr {
        let ffi_report_error = self.0.write().expect("Failed to write to RwLock")
            .as_mut().report_error(message.into());
        ffi_report_error.into()
    }

    /// Reports a compilation error at the location of a specific expression.
    ///
    /// This method creates an error expression that will cause compilation to fail
    /// with the specified error message, associating the error with the source
    /// location of the given expression. This provides better error messages by
    /// pointing to the exact location of the problem.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{MacroExprFactory, Expr};
    /// # fn example(factory: &mut MacroExprFactory, arg: &Expr) {
    /// // Check if argument is a constant
    /// if arg.kind().and_then(|k| k.as_constant()).is_none() {
    ///     // Report error at the argument's location
    ///     let error = factory.report_error_at(arg, "expected a constant value");
    ///     return;
    /// }
    /// # }
    /// ```
    pub fn report_error_at(&self, expr: &Expr, message: &str) -> Expr {
        let ffi_expr: cxx::UniquePtr<FfiExpr> = expr.into();
        let ffi_report_error = self.0.write().expect("Failed to write to RwLock")
            .as_mut().report_error_at(&ffi_expr, message.into());
        ffi_report_error.into()
    }
}

impl std::fmt::Debug for MacroExprFactory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MacroExprFactory(..)")
    }
}
