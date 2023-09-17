use crate::macros::piksellar_wrapper;

piksellar_wrapper!{PiksellarDevice, wgpu::Device}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PiksellarSize<T>
where
    T: Sized,
{
    pub width: T,
    pub height: T,
}

impl<T> PiksellarSize<T>
where
    T: Sized,
{
    pub fn new(width: T, height: T) -> Self {
        Self {
            width, height
        }
    }
}
