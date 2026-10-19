// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod view;

#[path = "fonts/font_loader.rs"]
mod font_loader;

struct Server {
    name: String,
    channels: Vec<JoinedChannel>,
    direct_messages: Vec<DirectMessage>,
}
struct JoinedChannel {
    name: String,
    users: Vec<String>,
    messages: Vec<view::chat::Message>,
}

struct DirectMessage {
    nickname: String,
    messages: Vec<view::chat::Message>,
}

#[derive(Clone, Copy, PartialEq)]
enum Selection {
    Channel { server: usize, channel: usize },
    Dm { server: usize, dm: usize },
}

#[derive(Default)]
struct Irc {
    servers: Vec<Server>,
    open_server: Option<usize>,
    selected: Option<Selection>,
    user_search: String,
    chat_input: String,
    show_users: bool,
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([940.0, 500.0]),
        ..Default::default()
    };

    let mut app = Irc {
        show_users: true,
        ..Default::default()
    };

    eframe::run_ui_native("irc-client", options, move |ui, _frame| {
        font_loader::load_fonts(ui.ctx());
        view::draw_ui(ui, &mut app);
    })
}