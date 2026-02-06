use egui::{Color32, Context, FontDefinitions, FontFamily, Style, Visuals};

pub const BG_PRIMARY: Color32 = Color32::from_rgb(26, 26, 30); // #1A1A1E
pub const BG_SECONDARY: Color32 = Color32::from_rgb(36, 36, 40); // #242428
pub const BG_TERTIARY: Color32 = Color32::from_rgb(46, 46, 52); // #2E2E34

pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(124, 92, 255); // #7C5CFF
pub const ACCENT_SECONDARY: Color32 = Color32::from_rgb(255, 107, 107); // #FF6B6B

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255); // #FFFFFF
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 168); // #A0A0A8
pub const TEXT_MUTED: Color32 = Color32::from_rgb(107, 107, 115); // #6B6B73

pub const BORDER_COLOR: Color32 = Color32::from_rgb(58, 58, 66); // #3A3A42
pub const SUCCESS: Color32 = Color32::from_rgb(74, 222, 128); // #4ADE80
pub const WARNING: Color32 = Color32::from_rgb(251, 191, 36); // #FBBF24

pub fn apply_theme(ctx: &Context) {
    let mut visuals = Visuals::dark();

    // Backgrounds
    visuals.window_fill = BG_SECONDARY;
    visuals.panel_fill = BG_PRIMARY;
    
    // Widgets
    visuals.widgets.noninteractive.bg_fill = BG_PRIMARY;
    visuals.widgets.noninteractive.fg_stroke.color = TEXT_SECONDARY;
    
    visuals.widgets.inactive.bg_fill = BG_SECONDARY;
    visuals.widgets.inactive.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
    
    visuals.widgets.hovered.bg_fill = BG_TERTIARY;
    visuals.widgets.hovered.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
    
    visuals.widgets.active.bg_fill = ACCENT_PRIMARY;
    visuals.widgets.active.fg_stroke.color = Color32::WHITE;
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    
    visuals.widgets.open.bg_fill = BG_TERTIARY;
    visuals.widgets.open.fg_stroke.color = TEXT_PRIMARY;
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(6);

    visuals.selection.bg_fill = ACCENT_PRIMARY.linear_multiply(0.4);
    visuals.selection.stroke.color = ACCENT_PRIMARY;

    visuals.window_stroke.color = BORDER_COLOR;
    visuals.window_shadow.color = Color32::from_black_alpha(40);

    ctx.set_visuals(visuals);

    let mut style = Style::default();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0); // sm
    style.spacing.window_margin = egui::Margin::same(8);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    
    // Smooth interactions
    style.animation_time = 0.15; // 150ms

    ctx.set_style(style);

    // Font setup (using defaults for now, but structured for easy replacement)
    // In a real scenario with asset loading, we would load Inter/JetBrains Mono here.
    let mut fonts = FontDefinitions::default();
    
    // Example: prioritizing a font if we had it loaded
    // fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "Inter".to_owned());
    
    ctx.set_fonts(fonts);
}
