use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowExtWebSys;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    // set up logging
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    // create window and event loop
    let (event_loop, window) = windowing::init_window();

    // web-specific logic
    {
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

    // start window event loop
    windowing::run_event_loop(event_loop, window);
}
