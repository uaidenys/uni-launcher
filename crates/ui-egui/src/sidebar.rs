//! The white side menu: brand, the lists (Início, Meus apps, …), settings, the theme switch and
//! the company credit.

use egui::{Color32, CornerRadius, FontId, RichText, Sense, Ui, vec2};

use crate::app::{Dialog, LauncherUi};
use crate::settings::Filter;
use crate::theme::{self, ThemeChoice, Tokens};

pub(crate) const SIDEBAR_W: f32 = 220.0;

/// The raised surface of the side menu and the cards: white on the light theme.
pub(crate) fn surface(t: &Tokens) -> Color32 {
    if t.dark { Color32::from_rgb(0x22, 0x32, 0x50) } else { Color32::WHITE }
}

enum Item {
    List(Filter),
    Settings,
}

impl LauncherUi {
    pub(crate) fn sidebar(&mut self, ui: &mut Ui) {
        let t = Tokens::get(ui.ctx());
        ui.add_space(22.0);
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            let (slot, _) = ui.allocate_exact_size(vec2(30.0, 30.0), Sense::hover());
            egui::Image::new(crate::icons::artcraft_mark()).paint_at(ui, slot);
            ui.add_space(8.0);
            ui.label(RichText::new("Uni Launcher").font(FontId::new(17.0, theme::semibold())).color(t.ink));
        });
        ui.add_space(28.0);
        let items: [(&str, &str, Item); 4] = [
            ("home", "Início", Item::List(Filter::All)),
            ("grid", "Meus apps", Item::List(Filter::Installed)),
            ("⟳", "Atualizações", Item::List(Filter::Updates)),
            ("⤓", "Downloads", Item::List(Filter::NotInstalled)),
        ];
        for (glyph, label, item) in items {
            self.nav_item(ui, glyph, label, item);
        }
        ui.add_space(10.0);
        let (line, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
        ui.painter().hline(line.shrink2(vec2(8.0, 0.0)).x_range(), line.center().y, t.hairline());
        ui.add_space(10.0);
        self.nav_item(ui, "⚙", "Configurações", Item::Settings);
        self.nav_item(ui, "info", "Sobre", Item::Settings);

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.add_space(18.0);
            ui.label(RichText::new("Desenvolvido por Denys Rodrigues de Oliveira").size(9.5).color(t.faint));
            ui.label(RichText::new("Uniphar").size(11.0).color(t.accent_ink));
            ui.label(RichText::new("© JRD Indústria Farmacêutica Ltda.").size(11.0).color(t.accent_ink));
            ui.add_space(12.0);
            let dark = t.dark;
            let mut on = dark;
            ui.horizontal(|ui| {
                if switch(ui, &mut on, t.invert_bg, t.line_strong).clicked() {
                    self.settings.theme = if on { ThemeChoice::Dark } else { ThemeChoice::Light };
                }
                ui.label(RichText::new(if dark { "Tema escuro" } else { "Tema claro" }).size(13.0).color(t.ink));
            });
        });
    }

    fn nav_item(&mut self, ui: &mut Ui, glyph: &str, label: &str, item: Item) {
        let t = Tokens::get(ui.ctx());
        let active = matches!(item, Item::List(f) if f == self.settings.filter) && self.dialog.is_none();
        let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 44.0), Sense::click());
        let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        let p = ui.painter();
        let ink = if active {
            p.rect_filled(rect, CornerRadius::same(12), t.invert_bg);
            t.invert_fg
        } else {
            if resp.hovered() {
                p.rect_filled(rect, CornerRadius::same(12), t.sunken);
            }
            t.ink
        };
        let y = rect.center().y;
        let c = egui::pos2(rect.left() + 26.0, y);
        let s = egui::Stroke::new(1.6, ink);
        match glyph {
            // Painted: the UI font has no house, grid or info glyphs.
            "home" => {
                p.line(vec![c + vec2(-7.0, -1.0), c + vec2(0.0, -7.0), c + vec2(7.0, -1.0)], s);
                p.line(vec![c + vec2(-5.0, -2.5), c + vec2(-5.0, 6.5), c + vec2(5.0, 6.5), c + vec2(5.0, -2.5)], s);
            }
            "grid" => {
                for (dx, dy) in [(-6.5, -6.5), (1.0, -6.5), (-6.5, 1.0), (1.0, 1.0)] {
                    p.rect_stroke(egui::Rect::from_min_size(c + vec2(dx, dy), vec2(5.5, 5.5)), CornerRadius::same(1), s, egui::StrokeKind::Middle);
                }
            }
            "info" => {
                p.circle_stroke(c, 7.5, s);
                p.line_segment([c + vec2(0.0, -0.5), c + vec2(0.0, 4.0)], s);
                p.circle_filled(c + vec2(0.0, -3.5), 1.1, ink);
            }
            _ => {
                p.text(c, egui::Align2::CENTER_CENTER, glyph, FontId::proportional(16.0), ink);
            }
        }
        p.text(egui::pos2(rect.left() + 46.0, y), egui::Align2::LEFT_CENTER, label, FontId::proportional(14.5), ink);
        if active {
            p.text(egui::pos2(rect.right() - 16.0, y), egui::Align2::RIGHT_CENTER, "›", FontId::proportional(16.0), ink);
        }
        ui.add_space(4.0);
        if resp.clicked() {
            match item {
                Item::List(f) => self.settings.filter = f,
                Item::Settings => self.dialog = Some(Dialog::Settings),
            }
        }
    }
}

/// "Bom dia / Boa tarde / Boa noite, <primeiro nome do usuário do Windows>".
pub(crate) fn greeting(hour: u32) -> String {
    let part = match hour {
        5..=11 => "Bom dia",
        12..=17 => "Boa tarde",
        _ => "Boa noite",
    };
    let user = std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_default();
    let first = user.split(['.', ' ', '_']).next().unwrap_or("");
    let mut c = first.chars();
    match c.next() {
        Some(f) => format!("{part}, {}{}", f.to_uppercase(), c.as_str()),
        None => part.to_string(),
    }
}

/// A pill switch, knob on the right when on.
fn switch(ui: &mut Ui, on: &mut bool, on_color: Color32, off_color: Color32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(36.0, 20.0), Sense::click());
    if resp.clicked() {
        *on = !*on;
    }
    let k = ui.ctx().animate_bool(resp.id, *on);
    let p = ui.painter();
    p.rect_filled(rect, CornerRadius::same(10), off_color.lerp_to_gamma(on_color, k));
    let x = egui::lerp(rect.left() + 10.0..=rect.right() - 10.0, k);
    p.circle_filled(egui::pos2(x, rect.center().y), 7.5, Color32::WHITE);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}
