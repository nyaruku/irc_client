pub struct Font {
    pub font_name: &'static str
    ,pub font_data: &'static [u8]
}

pub const FONT_REGISTER: [Font; 1] = [
    Font {
        font_name: "JetBrainsMono"
        ,font_data: include_bytes!("data/JetBrainsMonoNerdFontMono-Regular.ttf")
    }
];