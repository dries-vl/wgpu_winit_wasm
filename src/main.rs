
fn main() {
    // set up logging
    env_logger::init();

    // create window and event loop
    let (event_loop, window) = windowing::init_window();

    // start window event loop
    windowing::run_event_loop(event_loop, window);
}
