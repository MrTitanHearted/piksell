#[macro_export]
macro_rules! piksell_wrapper {
    ($wrapper_type:ident, $render_resource_type:ty) => {
        #[derive(Debug, Clone)]
        pub struct $wrapper_type(std::sync::Arc<$render_resource_type>);

        impl $wrapper_type {
            pub fn new(value: $render_resource_type) -> Self {
                Self(std::sync::Arc::new(value))
            }

            pub fn try_unwrap(self) -> Option<$render_resource_type> {
                std::sync::Arc::try_unwrap(self.0).ok()
            }
        }

        impl std::ops::Deref for $wrapper_type {
            type Target = $render_resource_type;

            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }


        impl std::convert::AsRef<$render_resource_type> for $wrapper_type {
            fn as_ref(&self) -> &$render_resource_type {
                self.0.as_ref()
            }
        }

        unsafe impl Send for $wrapper_type {}
        unsafe impl Sync for $wrapper_type {}
    }
}

#[macro_export]
macro_rules! piksell_id {
    ($atomic_id_type:ident) => {
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub struct $atomic_id_type(core::num::NonZeroU32);

        impl $atomic_id_type {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicU32, Ordering};

                static COUNTER: AtomicU32 = AtomicU32::new(1);

                let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
                Self(core::num::NonZeroU32::new(counter).expect(&format!("[ERROR]: The system ran out of unique '{}'s!", stringify!($atomic_id_type))))
            }
        }
    }
}

pub use piksell_wrapper;