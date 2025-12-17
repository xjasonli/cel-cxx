#include <cel-cxx-ffi/include/type_provider.h>
#include <cel-cxx-ffi/src/common/type_provider.rs.h>

namespace rust::cel_cxx {

using Type = cel::Type;
using StructTypeField = cel::StructTypeField;
using EnumConstant = cel::TypeIntrospector::EnumConstant;
using ValueBuilder = cel::ValueBuilder;
using MessageFactory = google::protobuf::MessageFactory;
using Arena = google::protobuf::Arena;

// TypeIntrospector
class AnyFfiTypeIntrospectorWrapper: public cel::TypeIntrospector {
public:
    AnyFfiTypeIntrospectorWrapper(Box<AnyFfiTypeIntrospector> ffi): ffi_(std::move(ffi)) {}

    virtual absl::StatusOr<absl::optional<Type>> FindTypeImpl(absl::string_view name) const override {
        bool found = false;
        Type result;
        auto status = ffi_->FindTypeImpl(name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }

    virtual absl::StatusOr<absl::optional<EnumConstant>> FindEnumConstantImpl(
        absl::string_view type_name,
        absl::string_view value_name
    ) const override {
        bool found = false;
        EnumConstant result;
        auto status = ffi_->FindEnumConstantImpl(type_name, value_name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }

    virtual absl::StatusOr<absl::optional<StructTypeField>> FindStructTypeFieldByNameImpl(
        absl::string_view type_name,
        absl::string_view field_name
    ) const override {
        bool found = false;
        StructTypeField result(cel::common_internal::BasicStructTypeField("", 0, Type()));
        auto status = ffi_->FindStructTypeFieldByNameImpl(type_name, field_name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }
private:
    Box<AnyFfiTypeIntrospector> ffi_;
};

std::unique_ptr<TypeIntrospector> TypeIntrospector_new(Box<AnyFfiTypeIntrospector> ffi) {
    return std::make_unique<AnyFfiTypeIntrospectorWrapper>(std::move(ffi));
}

// TypeReflector
class AnyFfiTypeReflectorWrapper: public cel::TypeReflector {
public:
    AnyFfiTypeReflectorWrapper(Box<AnyFfiTypeReflector> ffi): ffi_(std::move(ffi)) {}

    virtual absl::StatusOr<std::unique_ptr<ValueBuilder>> NewValueBuilder(
        absl::string_view name,
        google::protobuf::MessageFactory* message_factory,
        google::protobuf::Arena* arena
    ) const override {
        std::unique_ptr<ValueBuilder> result;
        auto status = ffi_->NewValueBuilder(name, *message_factory, *arena, result);
        if (!status.ok()) {
            return status;
        }
        return result;
    }

    virtual absl::StatusOr<absl::optional<Type>> FindTypeImpl(absl::string_view name) const override {
        bool found = false;
        Type result;
        auto status = ffi_->FindTypeImpl(name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }

    virtual absl::StatusOr<absl::optional<EnumConstant>> FindEnumConstantImpl(
        absl::string_view type_name,
        absl::string_view value_name
    ) const override {
        bool found = false;
        EnumConstant result;
        auto status = ffi_->FindEnumConstantImpl(type_name, value_name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }

    virtual absl::StatusOr<absl::optional<StructTypeField>> FindStructTypeFieldByNameImpl(
        absl::string_view type_name,
        absl::string_view field_name
    ) const override {
        bool found = false;
        StructTypeField result(cel::common_internal::BasicStructTypeField("", 0, Type()));
        auto status = ffi_->FindStructTypeFieldByNameImpl(type_name, field_name, found, result);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        if (!found) {
            return absl::nullopt;
        }
        return result;
    }
private:
    Box<AnyFfiTypeReflector> ffi_;
};

std::unique_ptr<TypeReflector> TypeReflector_new(Box<AnyFfiTypeReflector> ffi) {
    return std::make_unique<AnyFfiTypeReflectorWrapper>(std::move(ffi));
}

} // namespace rust::cel_cxx
