use crate as rust;
pub(crate) use cel_cxx_ffi::*;

mod activation;
mod types;
mod values;
pub(crate) use activation::*;
pub(crate) use types::*;
pub(crate) use values::*;

#[derive(Debug)]
pub struct Ctx {
    arena: cxx::SharedPtr<Arena>,
    descriptor_pool: cxx::SharedPtr<DescriptorPool>,
    message_factory: cxx::SharedPtr<MessageFactory>,
    /// Whether this context uses a dynamic descriptor pool/message factory.
    /// DynamicMessageFactory is not thread-safe, so clone_with_new_arena must
    /// create a fresh instance for each evaluation context.
    is_dynamic: bool,
}

impl Ctx {
    pub fn new_generated() -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::generated(),
            message_factory: MessageFactory::generated(),
            is_dynamic: false,
        }
    }

    pub fn new_dynamic(file_descriptor_set: &[u8]) -> Result<Self, &'static str> {
        Ok(Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::new(file_descriptor_set)?,
            message_factory: MessageFactory::new(),
            is_dynamic: true,
        })
    }

    pub fn clone_with_new_arena(&self) -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: self.descriptor_pool.clone(),
            // DynamicMessageFactory is not thread-safe for concurrent use.
            // Create a fresh instance for each evaluation context to avoid
            // data races when multiple threads evaluate concurrently.
            message_factory: if self.is_dynamic {
                MessageFactory::new()
            } else {
                self.message_factory.clone()
            },
            is_dynamic: self.is_dynamic,
        }
    }

    #[allow(unused)]
    pub fn shared_arena(&self) -> &cxx::SharedPtr<Arena> {
        &self.arena
    }

    pub fn shared_descriptor_pool(&self) -> &cxx::SharedPtr<DescriptorPool> {
        &self.descriptor_pool
    }

    #[allow(unused)]
    pub fn shared_message_factory(&self) -> &cxx::SharedPtr<MessageFactory> {
        &self.message_factory
    }

    pub fn arena(&self) -> &Arena {
        self.arena.as_ref().expect("arena is not initialized")
    }

    pub fn descriptor_pool(&self) -> &DescriptorPool {
        self.descriptor_pool
            .as_ref()
            .expect("descriptor pool is not initialized")
    }

    pub fn message_factory(&self) -> &MessageFactory {
        self.message_factory
            .as_ref()
            .expect("message factory is not initialized")
    }
}

impl Clone for Ctx {
    /// Clones the context with a new arena and (if dynamic) a fresh MessageFactory.
    ///
    /// This ensures `DynamicMessageFactory` is never shared across threads,
    /// which would cause data races since it is not thread-safe.
    fn clone(&self) -> Self {
        self.clone_with_new_arena()
    }
}
