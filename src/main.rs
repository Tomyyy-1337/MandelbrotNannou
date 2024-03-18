// native app entry_point

use async_std::task::block_on;

use sketch::run_app;

mod sketch;
mod mandelbrot;
mod complex;

fn main() {
    let model = sketch::Model::new(1024,768);
    block_on(async {
        run_app(model).await;
    });
}
