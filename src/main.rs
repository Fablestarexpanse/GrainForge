use eframe::egui;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`)

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("GrainForge"), // Title
        ..Default::default()
    };

    eframe::run_native(
        "GrainForge",
        native_options,
        Box::new(|cc| Ok(Box::new(GrainForgeApp::new(cc)))),
    )
}

struct GrainForgeApp {
    // Placeholder for app state
}

impl GrainForgeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Init wgpu render state here in the future
        Self {}
    }
}

impl eframe::App for GrainForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GrainForge");
            ui.label("Detailed film grain synthesis tool.");
            ui.separator();
            ui.label("Structure initialized successfully.");
        });
    }
}
