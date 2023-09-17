use crate::macros::piksell_wrapper;

piksell_wrapper!{PiksellDevice, wgpu::Device}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PiksellSize<T>
where
    T: Sized,
{
    pub width: T,
    pub height: T,
}

impl<T> PiksellSize<T>
where
    T: Sized,
{
    pub fn new(width: T, height: T) -> Self {
        Self {
            width, height
        }
    }
}
