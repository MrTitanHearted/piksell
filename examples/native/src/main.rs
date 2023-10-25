use winit::{
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use piksell::prelude::*;

#[tokio::main]
async fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = PiksellSize::new(window.inner_size().width, window.inner_size().height);

    let mut state = PiksellStateBuilder::new()
        .with_backends(PiksellBackends::DX12)
        .build(&window, size).await.unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let winit::event::Event::RedrawRequested(_) = event {
            state.render(Some(PiksellColor::BLUE)).unwrap();
        }
        
        if input.update(&event) {
            if let Some(size) = input.window_resized() {
                state.resize(PiksellSize::new(size.width, size.height));
            }
            
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
            }

            window.request_redraw();
        }
    });
}
