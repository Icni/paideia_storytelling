use eframe::{egui::{self, Color32, Frame, Margin, Stroke, Vec2}, NativeOptions};

use crate::story::{CutoutDomain, CutoutStory};

mod story;

#[derive(Debug, Default)]
pub struct CutoutApp {
    domain: CutoutDomain,
    story: Option<CutoutStory>,
    event_draft: String,
    viewing_story: bool,
}

impl CutoutApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);

        cc.egui_ctx.style_mut(|style| {
            use egui::{FontFamily, FontId, TextStyle};

            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(36.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Name("Heading2".into()),
                    FontId::new(25.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Name("Context".into()),
                    FontId::new(23.0, FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(24.0, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(14.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Button,
                    FontId::new(24.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(10.0, FontFamily::Proportional),
                ),
            ]
            .into()
        });

        Self {
            event_draft: String::new(),
            domain: CutoutDomain::default(),
            story: None,
            viewing_story: false,
        }
    }
}

impl eframe::App for CutoutApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let style = ctx.style();
        let mut add_event = false;

        egui::CentralPanel::default()
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(32.))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = [6., 12.].into();

                ui.horizontal(|ui| {
                    ui.heading("Cutout storyteller");

                    let generate_text =
                        format!("Generate {}", egui_material_icons::icons::ICON_CASINO);

                    let generate_response =
                        ui.add_visible(false, egui::Button::new(&generate_text));

                    ui.allocate_space(
                        [ui.available_width() - generate_response.rect.width(), 0.].into(),
                    );

                    if ui.button(generate_text).clicked() {
                        self.story = self.domain.generate_story();
                        self.viewing_story = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().slider_width = 300.;
                    ui.add(egui::Slider::new(&mut self.domain.story_length, 0..=10));
                    ui.add_space(12.);
                    ui.label("Story length");
                });

                ui.horizontal(|ui| {
                    ui.add_sized(
                        [ui.available_width() - 30., 20.],
                        egui::TextEdit::singleline(&mut self.event_draft),
                    );

                    if ui.button(egui_material_icons::icons::ICON_ADD).clicked() {
                        add_event = true;
                    }
                });

                Frame::canvas(&style)
                    .inner_margin(Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.;

                                self.domain.events.retain(|event| {
                                    Frame::new()
                                        .fill(Color32::from_rgb(36, 36, 36))
                                        .corner_radius(4.0)
                                        .inner_margin(Margin::same(4))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(event);
                                                ui.allocate_space(
                                                    [ui.available_width() - 40., 0.].into(),
                                                );

                                                !ui.button(egui_material_icons::icons::ICON_REMOVE)
                                                    .clicked()
                                            })
                                            .inner
                                        })
                                        .inner
                                });
                            });
                    });

                if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !self.event_draft.is_empty() {
                    add_event = true;
                }
            });

        if add_event {
            self.domain.events.push(self.event_draft.clone());
            self.event_draft.clear();
        }

        egui::Window::new("My story")
            .collapsible(false)
            .fixed_size(ctx.viewport_rect().size() - Vec2::new(50., 100.))
            .resizable(false)
            .open(&mut self.viewing_story)
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
                if let Some(CutoutStory(text)) = &self.story {
                    ui.label(text);
                }
            });
    }
}

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder {
            title: Some("Cutout Storyteller".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native("Cutout Storyteller", native_options, Box::new(|cc| Ok(Box::new(CutoutApp::new(cc)))))
}
