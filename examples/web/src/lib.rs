#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn).expect("[ERROR]: Couldn't intialize logger!");
    }

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    #[allow(unused_mut, unused_variables)]
    let mut window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;

        window.set_inner_size(PhysicalSize::new(800, 640));
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("[ERROR]: Couldn't append canvas to document body.");
    }

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
