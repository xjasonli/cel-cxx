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
    // Arena messages borrow factory prototypes, which borrow pool descriptors.
    // Fields must be dropped in this order.
    arena: cxx::SharedPtr<Arena>,
    message_factory: cxx::SharedPtr<MessageFactory>,
    descriptor_pool: cxx::SharedPtr<DescriptorPool>,
}

impl Ctx {
    pub fn new_generated() -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::generated(),
            message_factory: MessageFactory::generated(),
        }
    }

    pub fn new_dynamic(file_descriptor_set: &[u8]) -> Result<Self, &'static str> {
        Ok(Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::new(file_descriptor_set)?,
            message_factory: MessageFactory::new(),
        })
    }

    pub fn clone_with_new_arena(&self) -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: self.descriptor_pool.clone(),
            message_factory: self.message_factory.clone(),
        }
    }

    pub fn shared_descriptor_pool(&self) -> &cxx::SharedPtr<DescriptorPool> {
        &self.descriptor_pool
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
    /// Clones the context with a new arena and shared descriptors and prototypes.
    fn clone(&self) -> Self {
        self.clone_with_new_arena()
    }
}
