use blog_wasm::BlogApp;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<BlogApp>::new().render();
}
