#ifndef CEL_CXX_FFI_INCLUDE_PROTOBUF_H_
#define CEL_CXX_FFI_INCLUDE_PROTOBUF_H_

#include <google/protobuf/arena.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/message.h>
#include <google/protobuf/dynamic_message.h>
#include <google/protobuf/descriptor_database.h>
#include <google/protobuf/descriptor.pb.h>
#include <rust/cxx.h>
#include <internal/noop_delete.h>

namespace rust::cel_cxx {

using Arena = google::protobuf::Arena;
using DescriptorPool = google::protobuf::DescriptorPool;
using MessageFactory = google::protobuf::MessageFactory;

inline std::shared_ptr<Arena> NewArena() {
    return std::make_shared<Arena>();
}

inline std::shared_ptr<DescriptorPool> generated_pool() {
    return std::shared_ptr< DescriptorPool>(
        const_cast<DescriptorPool*>(DescriptorPool::generated_pool()),
        cel::internal::NoopDeleteFor<DescriptorPool>()
    );
}

inline std::shared_ptr<DescriptorPool> NewDescriptorPool(Slice<const uint8_t> file_descriptor_set) {
    google::protobuf::FileDescriptorSet fds;
    if (!fds.ParseFromArray(file_descriptor_set.data(), file_descriptor_set.size())) {
        throw std::invalid_argument("Failed to parse file descriptor set");
    }
    google::protobuf::SimpleDescriptorDatabase db;
    for (const auto& file : fds.file()) {
        db.Add(file);
    }
    return std::make_shared<DescriptorPool>(&db);
}

inline std::shared_ptr<MessageFactory> generated_factory() {
    return std::shared_ptr<MessageFactory>(
        MessageFactory::generated_factory(),
        cel::internal::NoopDeleteFor<MessageFactory>()
    );
}

inline std::shared_ptr<MessageFactory> NewMessageFactory() {
    return std::make_shared<google::protobuf::DynamicMessageFactory>();
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_PROTOBUF_H_
