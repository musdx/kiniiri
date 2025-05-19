use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([10000.0, 10000.0])
            .with_always_on_top()
            .with_mouse_passthrough(true)
            .with_transparent(true)
            .with_decorations(false),
        ..Default::default()
    };
    eframe::run_native(
        "vpetty",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::AlwaysOnTop,
        ));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::AlwaysOnTop,
        ));
        egui::Window::new("Pet")
            .frame(egui::Frame::NONE)
            .title_bar(false)
            .default_open(true)
            .resizable(false)
            .show(ctx, |ui| {
                ui.image(egui::include_image!("cat.webp"));
            });
    }
}
