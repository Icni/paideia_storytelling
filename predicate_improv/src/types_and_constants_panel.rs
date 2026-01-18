use std::mem::take;

use eframe::egui::{self, Color32, Frame, Margin, RichText, Stroke, Vec2};

use crate::
    app::{PredicateImprovApp, typed_object_input, untyped_object_input}
;

#[derive(Debug, Default, Clone)]
pub struct TypeDraft {
    pub name: String,
    pub supertypes: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ConstantDraft {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Default)]
pub struct TypesAndConstantsPanel {
    pub type_draft: TypeDraft,
    pub constant_draft: ConstantDraft,
    backup_type_draft: Option<TypeDraft>,
    backup_constant_draft: Option<ConstantDraft>,
    pub add_type: bool,
    pub add_constant: bool,
    show_type_modal: bool,
    show_constant_modal: bool,
}

impl TypesAndConstantsPanel {
    pub fn show(ui: &mut egui::Ui, app: &mut PredicateImprovApp) {
        ui.spacing_mut().item_spacing = [6., 12.].into();

        egui::SidePanel::left("types_panel")
            .frame(
                Frame::window(ui.style())
                    .stroke(Stroke::NONE)
                    .inner_margin(0),
            )
            .resizable(false)
            .min_width(ui.available_width() / 2. - 20.)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.label(
                    RichText::new("Types").text_style(egui::TextStyle::Name("Heading2".into())),
                );

                ui.add_space(16.);

                if ui.button("New type").clicked() {
                    app.types_and_constants_panel.show_type_modal = true;
                }
                Frame::canvas(ui.style())
                    .inner_margin(Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.;

                                app.domain.types.retain(|name, supertypes| {
                                    Frame::new()
                                        .fill(Color32::from_rgb(36, 36, 36))
                                        .corner_radius(4.0)
                                        .inner_margin(Margin::same(4))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if supertypes.is_empty() {
                                                    ui.label(&name.0);
                                                } else {
                                                    ui.label(format!("{}: ", &name.0));
                                                }

                                                for supertype in &mut *supertypes {
                                                    ui.label(&supertype.0);
                                                }

                                                ui.allocate_space(
                                                    [(ui.available_width() - 40.).max(0.), 0.]
                                                        .into(),
                                                );

                                                ui.menu_button("…", |ui| {
                                                    let mut retain = if ui.button("Edit").clicked()
                                                    {
                                                        app.types_and_constants_panel
                                                            .show_type_modal = true;

                                                        app.types_and_constants_panel
                                                            .type_draft
                                                            .name = name.clone().0;
                                                        app.types_and_constants_panel
                                                            .type_draft
                                                            .supertypes = supertypes
                                                            .iter_mut()
                                                            .map(|v| take(&mut v.0))
                                                            .collect();

                                                        app.types_and_constants_panel
                                                            .backup_type_draft = Some(
                                                            app.types_and_constants_panel
                                                                .type_draft
                                                                .clone(),
                                                        );

                                                        false
                                                    } else {
                                                        true
                                                    };

                                                    retain =
                                                        retain && !ui.button("Delete").clicked();

                                                    retain
                                                })
                                                .inner
                                            })
                                            .inner
                                        })
                                        .inner
                                        .unwrap_or(true)
                                });
                            })
                    });
            });

        egui::SidePanel::right("constants_panel")
            .frame(
                Frame::window(ui.style())
                    .stroke(Stroke::NONE)
                    .inner_margin(0),
            )
            .resizable(false)
            .min_width(ui.available_width() - 20.)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.label(
                    RichText::new("Constants").text_style(egui::TextStyle::Name("Heading2".into())),
                );

                ui.add_space(16.);

                if ui.button("New constant").clicked() {
                    app.types_and_constants_panel.show_constant_modal = true;
                }
                Frame::canvas(ui.style())
                    .inner_margin(Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.;

                                app.domain.constants.retain(|name, r#type| {
                                    Frame::new()
                                        .fill(Color32::from_rgb(36, 36, 36))
                                        .corner_radius(4.0)
                                        .inner_margin(Margin::same(4))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("{}: {}", name.0, r#type.0));

                                                ui.allocate_space(
                                                    [(ui.available_width() - 40.).max(0.), 0.]
                                                        .into(),
                                                );

                                                ui.menu_button("…", |ui| {
                                                    let mut retain = if ui.button("Edit").clicked()
                                                    {
                                                        app.types_and_constants_panel
                                                            .show_type_modal = true;

                                                        app.types_and_constants_panel
                                                            .constant_draft
                                                            .name = name.clone().0;
                                                        app.types_and_constants_panel
                                                            .constant_draft
                                                            .r#type = r#type.clone().0;

                                                        app.types_and_constants_panel
                                                            .backup_constant_draft = Some(
                                                            app.types_and_constants_panel
                                                                .constant_draft
                                                                .clone(),
                                                        );

                                                        false
                                                    } else {
                                                        true
                                                    };

                                                    retain =
                                                        retain && !ui.button("Delete").clicked();

                                                    retain
                                                })
                                                .inner
                                            })
                                            .inner
                                        })
                                        .inner
                                        .unwrap_or(true)
                                });
                            })
                    });
            });

        if app.types_and_constants_panel.show_type_modal {
            egui::Modal::new("type_modal".into()).show(ui.ctx(), |ui| type_modal(app, ui));
        }

        if app.types_and_constants_panel.show_constant_modal {
            egui::Modal::new("constant_modal".into()).show(ui.ctx(), |ui| constant_modal(app, ui));
        }
    }
}

fn type_modal(app: &mut PredicateImprovApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("type name:");
        ui.text_edit_singleline(&mut app.types_and_constants_panel.type_draft.name);
    });

    {
        let mut i = 0;
        app.types_and_constants_panel
            .type_draft
            .supertypes
            .retain_mut(|ty| {
                i += 1;
                untyped_object_input(ui, ty, format!("supertype {i}"))
            });
    }

    ui.horizontal(|ui| {
        ui.label("Add supertype");
        if ui.button(egui_material_icons::icons::ICON_ADD).clicked()
            && app.types_and_constants_panel.type_draft.supertypes.len() < 5
        {
            app.types_and_constants_panel
                .type_draft
                .supertypes
                .push(String::new());
        }
    });

    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("Submit").min_size(Vec2::new(ui.available_width() / 2., 0.)))
            .clicked()
        {
            app.types_and_constants_panel.show_type_modal = false;
            app.types_and_constants_panel.add_type = true;
        }

        if ui
            .add(egui::Button::new("Cancel").min_size(Vec2::new(ui.available_width(), 0.)))
            .clicked()
        {
            app.types_and_constants_panel.show_type_modal = false;

            if let Some(draft) = take(&mut app.types_and_constants_panel.backup_type_draft) {
                app.types_and_constants_panel.type_draft = draft;
                app.types_and_constants_panel.add_type = true;
            }
        }
    });
}

fn constant_modal(app: &mut PredicateImprovApp, ui: &mut egui::Ui) {
    typed_object_input(
        &app.domain,
        ui,
        &mut app.types_and_constants_panel.constant_draft.name,
        &mut app.types_and_constants_panel.constant_draft.r#type,
        "constant:",
    );

    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("Submit").min_size(Vec2::new(ui.available_width() / 2., 0.)))
            .clicked()
        {
            app.types_and_constants_panel.show_constant_modal = false;
            app.types_and_constants_panel.add_constant = true;
        }

        if ui
            .add(egui::Button::new("Cancel").min_size(Vec2::new(ui.available_width(), 0.)))
            .clicked()
        {
            app.types_and_constants_panel.show_constant_modal = false;

            if let Some(draft) = take(&mut app.types_and_constants_panel.backup_constant_draft) {
                app.types_and_constants_panel.constant_draft = draft;
                app.types_and_constants_panel.add_constant = true;
            }
        }
    });
}
