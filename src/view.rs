#[path = "views/chat.rs"]
pub mod chat;
#[path = "views/server_list.rs"]
mod server_list;
#[path = "views/channel_list.rs"]
mod channel_list;

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

/// Same margin on every panel; egui's defaults differ per panel type.
fn panel_frame(ui: &eframe::egui::Ui) -> eframe::egui::Frame {
    eframe::egui::Frame::side_top_panel(ui.style()).inner_margin(4)
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
        .frame(panel_frame(ui))
        .resizable(false)
        .exact_size(180.0)
        .show(ui, |ui| server_list::draw_server_list(ui, app));

    eframe::egui::Panel::left("sidebar")
        .frame(panel_frame(ui))
        .resizable(false)
        .exact_size(200.0)
        .show(ui, |ui| {
            if let Some(server_index) = app.open_server {
                channel_list::draw_dm_list(ui, app, server_index);
                channel_list::draw_channel_list(ui, app, server_index);
            }
        });

    // copied out because the closure needs all of `app`
    let mut show_users = app.show_users;
    eframe::egui::Panel::right("user_list")
        .frame(panel_frame(ui))
        .resizable(false)
        .exact_size(180.0)
        .show_collapsible(ui, &mut show_users, |ui| draw_user_list(ui, app));
    app.show_users = show_users;

    eframe::egui::Panel::top("chat_header").frame(panel_frame(ui)).show(ui, |ui| {
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
    // taken out because `messages` borrows all of `app`
    let mut chat_input = std::mem::take(&mut app.chat_input);
    let messages = if let Some(channel) = selected_channel(app) {
        &channel.messages[..]
    } else if let Some(dm) = selected_dm(app) {
        &dm.messages[..]
    } else {
        &[]
    };
    let _sent = chat::draw_chat(ui, messages, &mut chat_input);
    app.chat_input = chat_input;
    // TODO: hand `_sent` to irc_library once it can send
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
