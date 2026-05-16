use eframe::egui::{
    self, Color32, CornerRadius, Margin, Shadow, Stroke, Theme, Visuals,
};

pub const MINT: Color32 = Color32::from_rgb(168, 230, 207);
pub const MINT_SOFT: Color32 = Color32::from_rgb(195, 240, 222);
pub const MINT_DEEP: Color32 = Color32::from_rgb(110, 190, 160);

const RADIUS: u8 = 8;
const WINDOW_RADIUS: u8 = 12;

pub fn apply_theme(ctx: &egui::Context) {
    ctx.set_visuals_of(Theme::Dark, dark_visuals());
    ctx.set_visuals_of(Theme::Light, light_visuals());
    ctx.style_mut_of(Theme::Dark, tweak_style);
    ctx.style_mut_of(Theme::Light, tweak_style);
}

fn tweak_style(style: &mut egui::Style) {
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.menu_margin = Margin::same(8);
    style.spacing.window_margin = Margin::same(10);
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();

    v.panel_fill = Color32::from_rgb(24, 25, 28);
    v.window_fill = Color32::from_rgb(32, 34, 38);
    v.extreme_bg_color = Color32::from_rgb(16, 17, 20);
    v.faint_bg_color = Color32::from_rgb(36, 38, 42);
    v.code_bg_color = Color32::from_rgb(40, 42, 46);

    v.selection.bg_fill = MINT.linear_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0, MINT);
    v.hyperlink_color = MINT;

    let radius = CornerRadius::same(RADIUS);
    v.widgets.noninteractive.corner_radius = radius;
    v.widgets.inactive.corner_radius = radius;
    v.widgets.hovered.corner_radius = radius;
    v.widgets.active.corner_radius = radius;
    v.widgets.open.corner_radius = radius;

    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(48, 50, 54));
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 222, 226));

    v.widgets.inactive.bg_fill = Color32::from_rgb(44, 46, 50);
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(38, 40, 44);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(56, 58, 62));
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(210, 212, 216));

    v.widgets.hovered.bg_fill = Color32::from_rgb(54, 60, 58);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(46, 52, 50);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, MINT.linear_multiply(0.7));
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::WHITE);

    v.widgets.active.bg_fill = MINT.linear_multiply(0.35);
    v.widgets.active.weak_bg_fill = MINT.linear_multiply(0.25);
    v.widgets.active.bg_stroke = Stroke::new(1.5, MINT);
    v.widgets.active.fg_stroke = Stroke::new(1.5, Color32::WHITE);

    v.widgets.open.bg_fill = Color32::from_rgb(48, 56, 52);
    v.widgets.open.bg_stroke = Stroke::new(1.0, MINT.linear_multiply(0.5));

    v.window_corner_radius = CornerRadius::same(WINDOW_RADIUS);
    v.menu_corner_radius = CornerRadius::same(10);
    v.window_shadow = Shadow {
        offset: [0, 6],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(120),
    };
    v.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 28,
        spread: 0,
        color: Color32::from_black_alpha(140),
    };

    v
}

fn light_visuals() -> Visuals {
    let mut v = Visuals::light();

    v.panel_fill = Color32::from_rgb(248, 249, 250);
    v.window_fill = Color32::WHITE;
    v.extreme_bg_color = Color32::WHITE;
    v.faint_bg_color = Color32::from_rgb(240, 242, 244);
    v.code_bg_color = Color32::from_rgb(236, 239, 242);

    v.selection.bg_fill = MINT.linear_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0, MINT_DEEP);
    v.hyperlink_color = MINT_DEEP;

    let radius = CornerRadius::same(RADIUS);
    v.widgets.noninteractive.corner_radius = radius;
    v.widgets.inactive.corner_radius = radius;
    v.widgets.hovered.corner_radius = radius;
    v.widgets.active.corner_radius = radius;
    v.widgets.open.corner_radius = radius;

    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 222, 226));
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 42, 46));

    v.widgets.inactive.bg_fill = Color32::from_rgb(238, 240, 243);
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(246, 247, 249);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(210, 213, 218));
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 42, 46));

    v.widgets.hovered.bg_fill = MINT_SOFT;
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(225, 244, 234);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, MINT_DEEP);
    v.widgets.hovered.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 24, 28));

    v.widgets.active.bg_fill = MINT;
    v.widgets.active.weak_bg_fill = MINT_SOFT;
    v.widgets.active.bg_stroke = Stroke::new(1.5, MINT_DEEP);
    v.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 24, 28));

    v.widgets.open.bg_fill = MINT_SOFT;
    v.widgets.open.bg_stroke = Stroke::new(1.0, MINT_DEEP);

    v.window_corner_radius = CornerRadius::same(WINDOW_RADIUS);
    v.menu_corner_radius = CornerRadius::same(10);
    v.window_shadow = Shadow {
        offset: [0, 4],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(30),
    };
    v.popup_shadow = Shadow {
        offset: [0, 6],
        blur: 22,
        spread: 0,
        color: Color32::from_black_alpha(40),
    };

    v
}

pub fn toggle_theme(ctx: &egui::Context) {
    let next = match ctx.theme() {
        Theme::Dark => Theme::Light,
        Theme::Light => Theme::Dark,
    };
    ctx.set_theme(next);
}
