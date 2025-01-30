use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

pub const WIDTH: u32 = 804;
pub const HEIGHT: u32 = 720;

pub const SCALE: f64 = 2.0;

fn build_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
    WindowBuilder::new()
        .with_title("Physics Apple")
        .with_inner_size(scaled_size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}

pub fn run<F1, F2>(mut draw_function: F1, mut update_function: F2)
where
    F1: FnMut(&mut [u8]) + 'static,
    F2: FnMut() + 'static,
{
    let event_loop = EventLoop::new().expect("Could not create EventLoop");
    let mut input = WinitInputHelper::new();

    let window = build_window(&event_loop);
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop
        .run(|event, ev_window| {
            // The one and only event that winit_input_helper doesn't have for us...

            match &event {
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::RedrawRequested => {
                        draw_function(pixels.frame_mut());
                        if let Err(err) = pixels.render() {
                            println!("[ERROR] pixels.render() failed: {}", err);
                            ev_window.exit();
                            return;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            // For everything else, for let winit_input_helper collect events to build its state.
            // It returns `true` when it is time to update our game state and request a redraw.
            if input.update(&event) {
                // Close events
                if input.key_pressed(KeyCode::Escape)
                    || input.key_pressed(KeyCode::KeyQ)
                    || input.close_requested()
                    || input.destroyed()
                {
                    ev_window.exit();
                    return;
                }

                update_function();

                window.request_redraw();
            }
        })
        .expect("EventLoop failed");
}
