use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| if input.update(&event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
            *control_flow = ControlFlow::Exit;
        }
    });
}
