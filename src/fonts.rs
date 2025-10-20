use eframe::egui;

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "noto_sans".to_owned(),
        egui::FontData::from_static(
            include_bytes!("../assets/fonts/NotoSansSC-VariableFont_wght.ttf")
        ).into(),
    );

    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans".to_owned());

    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "noto_sans".to_owned());

    ctx.set_fonts(fonts);
}
