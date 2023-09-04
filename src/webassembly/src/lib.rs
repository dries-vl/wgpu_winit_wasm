#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

#[cfg(target_arch = "wasm32")]
mod state;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    // run
    let future = async {
        run().await;
    };
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(target_arch = "wasm32")]
pub async fn run() {
    // set up logging
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    }

    // create window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // web-specific logic
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys; // ide doesnt realise platform is wasm; #[cfg(wasm_platform)]
        use winit::dpi::PhysicalSize;
        // winit prevents sizing with CSS, so we have to set the size manually when on web.
        window.set_inner_size(PhysicalSize::new(450, 400));
        // attach winit window to html canvas
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-winit-wgpu")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // wgpu
    let state = state::State::new(window).await;

    // run event loop
    run_event_loop(event_loop, state);
}

#[cfg(target_arch = "wasm32")]
fn run_event_loop(event_loop: EventLoop<()>, mut state: state::State) {
    // start window event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            state.window().request_redraw();
        },
        _ => {}
    });
}
