#[cxx::bridge]
mod ffi {
    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!(<google/protobuf/arena.h>);
        type Arena;
        #[rust_name = "space_allocated"]
        fn SpaceAllocated(self: &Arena) -> u64;
        #[rust_name = "space_used"]
        fn SpaceUsed(self: &Arena) -> u64;

        include!(<google/protobuf/descriptor.h>);
        type DescriptorPool;

        include!(<google/protobuf/message.h>);
        type MessageFactory;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/protobuf.h>);
        fn NewArena() -> SharedPtr<Arena>;

        fn generated_pool() -> SharedPtr<DescriptorPool>;
        fn NewDescriptorPool(file_descriptor_set: &[u8]) -> SharedPtr<DescriptorPool>;

        fn generated_factory() -> SharedPtr<MessageFactory>;
        fn NewMessageFactory() -> SharedPtr<MessageFactory>;
    }
}

// Arena
pub use ffi::Arena;
unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}
impl Arena {
    pub fn new() -> cxx::SharedPtr<Self> {
        ffi::NewArena()
    }
}

impl std::fmt::Debug for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Arena {{ space_allocated: {}, space_used: {} }}",
            self.space_allocated(),
            self.space_used(),
        )
    }
}

// DescriptorPool
pub use ffi::DescriptorPool;
unsafe impl Send for DescriptorPool {}
unsafe impl Sync for DescriptorPool {}

impl DescriptorPool {
    pub fn generated() -> cxx::SharedPtr<Self> {
        ffi::generated_pool()
    }

    /// Build a `DescriptorPool` from a serialized `FileDescriptorSet`.
    ///
    /// Returns `Err` if the bytes are malformed or a file descriptor fails to build.
    /// The C++ side returns a null `shared_ptr` on any of these failures without
    /// further detail, so the error message is generic.
    pub fn new(file_descriptor_set: &[u8]) -> Result<cxx::SharedPtr<Self>, &'static str> {
        let pool = ffi::NewDescriptorPool(file_descriptor_set);
        if pool.is_null() {
            return Err("failed to build DescriptorPool from FileDescriptorSet");
        }
        Ok(pool)
    }
}

impl std::fmt::Debug for DescriptorPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const DescriptorPool;
        let is_generated = {
            let generated_ptr = Self::generated()
                .as_ref()
                .expect("generated_pool is not initialized")
                as *const ffi::DescriptorPool;
            ptr == generated_ptr
        };
        write!(
            f,
            "DescriptorPool {{ ptr: {ptr:p}, is_generated: {is_generated} }}",
        )
    }
}

// MessageFactory
pub use ffi::MessageFactory;
// SAFETY: The generated MessageFactory (singleton) is truly thread-safe.
// DynamicMessageFactory::GetPrototype() mutates internal caches without synchronization,
// so it is NOT safe for concurrent shared access. However, the Ctx architecture ensures
// DynamicMessageFactory is never shared: Ctx::clone_with_new_arena() creates a fresh
// instance per evaluation context, so each thread gets exclusive access. This invariant
// is critical — do NOT share a DynamicMessageFactory across threads.
unsafe impl Send for MessageFactory {}
unsafe impl Sync for MessageFactory {}

impl MessageFactory {
    pub fn generated() -> cxx::SharedPtr<Self> {
        ffi::generated_factory()
    }

    pub fn new() -> cxx::SharedPtr<Self> {
        ffi::NewMessageFactory()
    }
}

impl std::fmt::Debug for MessageFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const MessageFactory;
        let is_generated = {
            let generated_ptr = Self::generated()
                .as_ref()
                .expect("generated_factory is not initialized")
                as *const ffi::MessageFactory;
            ptr == generated_ptr
        };
        write!(
            f,
            "MessageFactory {{ ptr: {ptr:p}, is_generated: {is_generated} }}",
        )
    }
}
