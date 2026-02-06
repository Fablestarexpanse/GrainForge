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
    state: grainforge::app::state::AppState,
}

impl GrainForgeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply custom theme
        grainforge::app::theme::apply_theme(&cc.egui_ctx);
        
        Self {
            state: grainforge::app::state::AppState::default(),
        }
    }
}

impl eframe::App for GrainForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        grainforge::ui::main_window::show(ctx, &mut self.state);
    }
}
