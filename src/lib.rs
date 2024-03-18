use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

mod mandelbrot;
mod complex;

mod sketch;
use sketch::run_app;


// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = sketch::Model::new(1024,768);
    block_on(async {
        run_app(model).await;
    });
}
