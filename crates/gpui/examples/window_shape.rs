use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowKind, WindowOptions, WindowShape,
    div, prelude::*, px, red, size,
};

struct WindowDemo;
impl Render for WindowDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(red())
            .size_full()
            .window_control_area(gpui::WindowControlArea::Drag)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(200.0), px(200.0)),
                    cx,
                ))),
                kind: WindowKind::Overlay,
                shape: Some(WindowShape::Circle { radius: Some(px(100.0)) }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| WindowDemo {}),
        )
        .expect("error ");
        cx.activate(true);
    });
}
