use anyhow::{anyhow, Result};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use crate::prelude::{
    PiksellAlphaMode, PiksellBackends, PiksellDeviceFeatures, PiksellDeviceLimits,
    PiksellDx12Compiler, PiksellPowerPreference, PiksellPresentMode, PiksellSize,
    PiksellSurfaceCapabilities, PiksellSurfaceConfiguration, PiksellTextureUsages, PiksellColor,
};

#[derive(Debug)]
pub struct PiksellStateBuilder {
    backends: PiksellBackends,
    dx12_shader_compiler: PiksellDx12Compiler,
    power_preference: PiksellPowerPreference,
    force_fallback_adapter: bool,
    device_label: Option<String>,
    device_features: PiksellDeviceFeatures,
    device_limits: PiksellDeviceLimits,
    present_mode: Option<PiksellPresentMode>,
    alpha_mode: Option<PiksellAlphaMode>,
}

impl Default for PiksellStateBuilder {
    fn default() -> Self {
        Self {
            backends: PiksellBackends::PRIMARY,
            dx12_shader_compiler: PiksellDx12Compiler::default(),
            power_preference: PiksellPowerPreference::default(),
            force_fallback_adapter: false,
            device_label: None,
            device_features: PiksellDeviceFeatures::default(),
            device_limits: PiksellDeviceLimits::default(),
            present_mode: None,
            alpha_mode: None,
        }
    }
}

impl PiksellStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_backends(mut self, backends: PiksellBackends) -> Self {
        self.backends = backends;
        self
    }

    pub fn with_dx12_compiler(mut self, compiler: PiksellDx12Compiler) -> Self {
        self.dx12_shader_compiler = compiler;
        self
    }

    pub fn with_power_preference(mut self, preference: PiksellPowerPreference) -> Self {
        self.power_preference = preference;
        self
    }

    pub fn with_device_label(mut self, label: &str) -> Self {
        self.device_label = Some(label.to_string());
        self
    }

    pub fn with_device_features(mut self, features: PiksellDeviceFeatures) -> Self {
        self.device_features = features;
        self
    }

    pub fn with_device_limits(mut self, limits: PiksellDeviceLimits) -> Self {
        self.device_limits = limits;
        self
    }

    pub fn with_present_mode(mut self, mode: PiksellPresentMode) -> Self {
        self.present_mode = Some(mode);
        self
    }

    pub fn with_alpha_mode(mut self, mode: PiksellAlphaMode) -> Self {
        self.alpha_mode = Some(mode);
        self
    }

    pub async fn build<Window>(
        self,
        window: &Window,
        size: PiksellSize,
    ) -> Result<PiksellState>
    where
        Window: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: self.backends,
            dx12_shader_compiler: self.dx12_shader_compiler,
        });

        let surface = unsafe { instance.create_surface(window)? };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: self.power_preference,
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: Some(&surface),
            })
            .await
            .or({
                instance
                    .enumerate_adapters(self.backends)
                    .find(|adapter| adapter.is_surface_supported(&surface))
            })
            .ok_or(anyhow!("[ERROR]: No supported adapter was found!\n"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: self.device_label.as_ref().map(|label| label.as_str()),
                    features: self.device_features,
                    limits: self.device_limits,
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_configuration = PiksellSurfaceConfiguration {
            usage: PiksellTextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: self
                .present_mode
                .unwrap_or(surface_capabilities.present_modes[0]),
            alpha_mode: self
                .alpha_mode
                .unwrap_or(surface_capabilities.alpha_modes[0]),
            view_formats: Vec::new(),
        };

        Ok(PiksellState {
            window_size: size,
            instance,
            surface,
            surface_capabilities,
            surface_configuration,
            adapter,
            device,
            queue,
        })
    }
}

#[derive(Debug)]
pub struct PiksellState {
    pub(crate) window_size: PiksellSize,
    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_capabilities: PiksellSurfaceCapabilities,
    pub(crate) surface_configuration: PiksellSurfaceConfiguration,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl PiksellState {
    pub fn resize(&mut self, new_size: PiksellSize) {
        self.window_size = new_size;
        self.surface_configuration.width = new_size.width;
        self.surface_configuration.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_configuration);
    }

    pub fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn get_device_features(&self) -> PiksellDeviceFeatures {
        self.device.features()
    }

    pub fn get_device_limits(&self) -> PiksellDeviceLimits {
        self.device.limits()
    }

    pub fn get_surface_capabilities(&self) -> PiksellSurfaceCapabilities {
        PiksellSurfaceCapabilities {
            formats: self.surface_capabilities.formats.clone(),
            present_modes: self.surface_capabilities.present_modes.clone(),
            alpha_modes: self.surface_capabilities.alpha_modes.clone(),
            usages: self.surface_capabilities.usages,
        }
    }

    pub fn render(&self, clear_color: Option<PiksellColor>) -> Result<()> {
        let output = self.surface.get_current_texture()?;
        let ref view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Piksell Render Encoder"),
            });
        
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Piksell State Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if let Some(color) = clear_color {
                            wgpu::LoadOp::Clear(color)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(vec![encoder.finish()]);
        output.present();
        
        Ok(())
    }
}
