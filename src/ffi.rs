use crate as rust;
pub(crate) use cel_cxx_ffi::*;

mod activation;
mod error;
mod types;
mod values;
pub(crate) use activation::*;
pub(crate) use error::*;
pub(crate) use types::*;
pub(crate) use values::*;

#[derive(Clone, Debug)]
pub struct Ctx {
    arena: cxx::SharedPtr<Arena>,
    descriptor_pool: cxx::SharedPtr<DescriptorPool>,
    message_factory: cxx::SharedPtr<MessageFactory>,
}

impl Ctx {
    pub fn new_generated() -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::generated(),
            message_factory: MessageFactory::generated(),
        }
    }

    #[allow(unused)]
    pub fn new_dynamic(file_descriptor_set: &[u8]) -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: DescriptorPool::new(file_descriptor_set),
            message_factory: MessageFactory::new(),
        }
    }

    pub fn clone_with_new_arena(&self) -> Self {
        Self {
            arena: Arena::new(),
            descriptor_pool: self.descriptor_pool.clone(),
            message_factory: self.message_factory.clone(),
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
