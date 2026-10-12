const SERVER_LIST_WIDTH: f32 = 180.0;
const SIDEBAR_WIDTH: f32 = 200.0;
const USER_LIST_WIDTH: f32 = 180.0;

fn selected_channel(app: &crate::Irc) -> Option<&crate::JoinedChannel> {
    if let Some(crate::Selection::Channel { server, channel }) = app.selected {
        app.servers.get(server)?.channels.get(channel)
    } else {
        None
    }
}

fn selected_dm(app: &crate::Irc) -> Option<&crate::DirectMessage> {
    if let Some(crate::Selection::Dm { server, dm }) = app.selected {
        app.servers.get(server)?.direct_messages.get(dm)
    } else {
        None
    }
}

/// Full-width button, highlighted when selected. Returns true when clicked.
fn list_button(ui: &mut eframe::egui::Ui, text: &str, selected: bool) -> bool {
    let button = eframe::egui::Button::new(text)
        .selected(selected)
        .truncate()
        .min_size(eframe::egui::vec2(ui.available_width(), 0.0));
    ui.add(button).clicked()
}

pub fn draw_ui(ui: &mut eframe::egui::Ui, app: &mut crate::Irc) {
    eframe::egui::Panel::left("server_list")
        .resizable(false)
        .exact_size(SERVER_LIST_WIDTH)
        .show(ui, |ui| draw_server_list(ui, app));

    eframe::egui::Panel::left("sidebar")
        .resizable(false)
        .exact_size(SIDEBAR_WIDTH)
        .show(ui, |ui| {
            if let Some(server_index) = app.open_server {
                draw_channel_list(ui, app, server_index);
            } else {
                draw_dm_list(ui, app);
            }
        });

    // copied out because the closure needs all of `app`
    let mut show_users = app.show_users;
    eframe::egui::Panel::right("user_list")
        .resizable(false)
        .exact_size(USER_LIST_WIDTH)
        .show_collapsible(ui, &mut show_users, |ui| draw_user_list(ui, app));
    app.show_users = show_users;

    eframe::egui::Panel::top("chat_header").show(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(channel) = selected_channel(app) {
                ui.heading(&channel.name);
            } else if let Some(dm) = selected_dm(app) {
                ui.heading(&dm.nickname);
            }
            ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                ui.toggle_value(&mut app.show_users, "Users");
            });
        });
    });

    eframe::egui::CentralPanel::default().show(ui, |ui| {
        ui.label("Hello World");
    });
}

fn draw_server_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc) {
    // pinned to the bottom; no side margin, the outer panel already has one
    eframe::egui::Panel::bottom("server_list_actions")
        .frame(eframe::egui::Frame::NONE.inner_margin(eframe::egui::Margin::symmetric(0, 4)))
        .show(ui, |ui| {
            if list_button(ui, "Add server", false) {
                app.servers.push(crate::Server {
                    name: format!("New server {}", app.servers.len() + 1),
                    channels: Vec::new(),
                    direct_messages: Vec::new(),
                });
                app.open_server = Some(app.servers.len() - 1);
            }
        });

    eframe::egui::ScrollArea::vertical().show(ui, |ui| {
        if list_button(ui, "Direct Messages", app.open_server.is_none()) {
            app.open_server = None;
        }
        ui.separator();

        for (i, server) in app.servers.iter().enumerate() {
            if list_button(ui, &server.name, app.open_server == Some(i)) {
                app.open_server = Some(i);
            }
        }
    });
}

// DMs from every server in one list.
fn draw_dm_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc) {
    ui.heading("Direct Messages");
    ui.separator();

    eframe::egui::ScrollArea::vertical().show(ui, |ui| {
        for (server_index, server) in app.servers.iter().enumerate() {
            for (dm_index, dm) in server.direct_messages.iter().enumerate() {
                let this = crate::Selection::Dm { server: server_index, dm: dm_index };
                let text = format!("{} ({})", dm.nickname, server.name);
                if list_button(ui, &text, app.selected == Some(this)) {
                    app.selected = Some(this);
                }
            }
        }
    });
}

fn draw_channel_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc, server_index: usize) {
    if let Some(server) = app.servers.get(server_index) {
        ui.heading(&server.name);
        ui.separator();

        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            for (channel_index, channel) in server.channels.iter().enumerate() {
                let this = crate::Selection::Channel { server: server_index, channel: channel_index };
                if list_button(ui, &channel.name, app.selected == Some(this)) {
                    app.selected = Some(this);
                }
            }
        });
    }
}

fn draw_user_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc) {
    ui.add(
        eframe::egui::TextEdit::singleline(&mut app.user_search)
            .hint_text("Search users")
            .desired_width(f32::INFINITY),
    );
    ui.separator();

    let query = app.user_search.to_lowercase();
    if let Some(channel) = selected_channel(app) {
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            for user in &channel.users {
                if user.to_lowercase().contains(&query) {
                    ui.label(user);
                }
            }
        });
    }
}
