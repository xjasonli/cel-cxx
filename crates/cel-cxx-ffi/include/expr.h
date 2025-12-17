#ifndef CEL_CXX_FFI_INCLUDE_COMMON_EXPR_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_EXPR_H_

#include <common/expr.h>

namespace rust::cel_cxx {

using Expr = cel::Expr;
using ListExprElement = cel::ListExprElement;
using StructExprField = cel::StructExprField;
using MapExprEntry = cel::MapExprEntry;

using Span_Expr = absl::Span<const cel::Expr>;
using Span_ListExprElement = absl::Span<const cel::ListExprElement>;
using Span_StructExprField = absl::Span<const cel::StructExprField>;
using Span_MapExprEntry = absl::Span<const cel::MapExprEntry>;

using MutSpan_Expr = absl::Span<cel::Expr>;
using MutSpan_ListExprElement = absl::Span<cel::ListExprElement>;
using MutSpan_StructExprField = absl::Span<cel::StructExprField>;
using MutSpan_MapExprEntry = absl::Span<cel::MapExprEntry>;

// Expr
inline std::unique_ptr<Expr> Expr_new() {
    return std::make_unique<Expr>();
}

inline void Expr_set_const_expr(Expr& expr, std::unique_ptr<cel::Constant> constant) {
    expr.set_const_expr(std::move(*constant));
}

inline void Expr_set_ident_expr(Expr& expr, std::unique_ptr<cel::IdentExpr> ident_expr) {
    expr.set_ident_expr(std::move(*ident_expr));
}

inline void Expr_set_select_expr(Expr& expr, std::unique_ptr<cel::SelectExpr> select_expr) {
    expr.set_select_expr(std::move(*select_expr));
}

inline void Expr_set_call_expr(Expr& expr, std::unique_ptr<cel::CallExpr> call_expr) {
    expr.set_call_expr(std::move(*call_expr));
}

inline void Expr_set_list_expr(Expr& expr, std::unique_ptr<cel::ListExpr> list_expr) {
    expr.set_list_expr(std::move(*list_expr));
}

inline void Expr_set_struct_expr(Expr& expr, std::unique_ptr<cel::StructExpr> struct_expr) {
    expr.set_struct_expr(std::move(*struct_expr));
}

inline void Expr_set_map_expr(Expr& expr, std::unique_ptr<cel::MapExpr> map_expr) {
    expr.set_map_expr(std::move(*map_expr));
}

inline void Expr_set_comprehension_expr(Expr& expr, std::unique_ptr<cel::ComprehensionExpr> comprehension_expr) {
    expr.set_comprehension_expr(std::move(*comprehension_expr));
}

inline size_t Expr_size_of() {
    return sizeof(cel::Expr);
}

inline void Expr_push_unique(std::vector<Expr>& vector, std::unique_ptr<Expr> value) {
    vector.push_back(std::move(*value));
}

inline std::unique_ptr<Expr> Expr_pop_unique(std::vector<Expr>& vector) {
    std::unique_ptr<Expr> result = nullptr;
    if (!vector.empty()) {
        result = std::make_unique<Expr>(std::move(vector.back()));
        vector.pop_back();
    }
    return result;
}

// ListExprElement
inline std::unique_ptr<ListExprElement> ListExprElement_new() {
    return std::make_unique<ListExprElement>();
}

inline size_t ListExprElement_size_of() {
    return sizeof(cel::ListExprElement);
}

inline void ListExprElement_push_unique(std::vector<ListExprElement>& vector, std::unique_ptr<ListExprElement> value) {
    vector.push_back(std::move(*value));
}

inline std::unique_ptr<ListExprElement> ListExprElement_pop_unique(std::vector<ListExprElement>& vector) {
    std::unique_ptr<ListExprElement> result = nullptr;
    if (!vector.empty()) {
        result = std::make_unique<ListExprElement>(std::move(vector.back()));
        vector.pop_back();
    }
    return result;
}

// StructExprField
inline std::unique_ptr<StructExprField> StructExprField_new() {
    return std::make_unique<StructExprField>();
}

inline size_t StructExprField_size_of() {
    return sizeof(cel::StructExprField);
}

inline void StructExprField_push_unique(std::vector<StructExprField>& vector, std::unique_ptr<StructExprField> value) {
    vector.push_back(std::move(*value));
}

inline std::unique_ptr<StructExprField> StructExprField_pop_unique(std::vector<StructExprField>& vector) {
    std::unique_ptr<StructExprField> result = nullptr;
    if (!vector.empty()) {
        result = std::make_unique<StructExprField>(std::move(vector.back()));
        vector.pop_back();
    }
    return result;
}

// MapExprEntry
inline std::unique_ptr<MapExprEntry> MapExprEntry_new() {
    return std::make_unique<MapExprEntry>();
}

inline size_t MapExprEntry_size_of() {
    return sizeof(cel::MapExprEntry);
}

inline void MapExprEntry_push_unique(std::vector<MapExprEntry>& vector, std::unique_ptr<MapExprEntry> value) {
    vector.push_back(std::move(*value));
}

inline std::unique_ptr<MapExprEntry> MapExprEntry_pop_unique(std::vector<MapExprEntry>& vector) {
    std::unique_ptr<MapExprEntry> result = nullptr;
    if (!vector.empty()) {
        result = std::make_unique<MapExprEntry>(std::move(vector.back()));
        vector.pop_back();
    }
    return result;
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_EXPR_H_
