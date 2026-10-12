// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_screen;

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
}

struct DirectMessage {
    nickname: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Selection {
    Channel { server: usize, channel: usize },
    Dm { server: usize, dm: usize },
}

#[derive(Default)]
struct Irc {
    servers: Vec<Server>,
    // Server whose channels the sidebar lists, dm list when no server.
    open_server: Option<usize>,
    selected: Option<Selection>,
    user_search: String,
    show_users: bool,
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let mut app = Irc {
        show_users: true,
        ..Default::default()
    };

    eframe::run_ui_native("irc-client", options, move |ui, _frame| {
        font_loader::load_fonts(ui.ctx());
        main_screen::draw_ui(ui, &mut app);
    })
}