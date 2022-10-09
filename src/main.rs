use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(1280.0, 800.0)
                .resizable(true)
                .child(TextBlock::new().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}