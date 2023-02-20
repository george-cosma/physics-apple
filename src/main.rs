use std::thread::sleep_ms;

use physics::generate_board;

mod physics;
mod sequence;

// Rendering {
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
// }

const WIDTH: u32 = 480;
const HEIGHT: u32 = 360;
fn main() {
    let mut board = generate_board("frame.0811.png".to_string());

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Physics Apple")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            // Request draw here
            board.draw_static_field(pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                println!("[ERROR] pixels.render() failed: {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            
            // Resize the window
            // if let Some(size) = input.window_resized() {
            //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
            //         println!("[ERROR] pixels.resize_surface() failed: {err}");
            //         *control_flow = ControlFlow::Exit;
            //         return;
            //     }
            // }
            // Any update code here
            // life.update();
            
            window.request_redraw();
        }
    });
}
