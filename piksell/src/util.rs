pub type PiksellBackends = wgpu::Backends;
pub type PiksellDx12Compiler = wgpu::Dx12Compiler;
pub type PiksellPowerPreference = wgpu::PowerPreference;
pub type PiksellDeviceFeatures = wgpu::Features;
pub type PiksellDeviceLimits = wgpu::Limits;
pub type PiksellSurfaceCapabilities = wgpu::SurfaceCapabilities;
pub type PiksellSurfaceConfiguration = wgpu::SurfaceConfiguration;
pub type PiksellTextureFormat = wgpu::TextureFormat;
pub type PiksellPresentMode = wgpu::PresentMode;
pub type PiksellAlphaMode = wgpu::CompositeAlphaMode;

pub type PiksellTextureUsages = wgpu::TextureUsages;

pub type PiksellColor = wgpu::Color;


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PiksellSize {
    pub width: u32,
    pub height: u32,
}

impl PiksellSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width, height
        }
    }
}
