mod font_register;

pub fn load_fonts(ctx: &eframe::egui::Context) {
    for font in font_register::FONT_REGISTER {
        ctx.add_font(
            eframe::egui::epaint::text::FontInsert::new(
                font.font_name
                ,eframe::egui::FontData::from_static(font.font_data)
                ,vec![
                    eframe::epaint::text::InsertFontFamily {
                        family: egui::FontFamily::Proportional,
                        priority: egui::epaint::text::FontPriority::Highest,
                    },
                    eframe::epaint::text::InsertFontFamily {
                        family: egui::FontFamily::Monospace,
                        priority: egui::epaint::text::FontPriority::Lowest,
                    },
                ],
            )
        );
    }
}
