pub fn draw_channel_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc, server_index: usize) {
    if let Some(server) = app.servers.get(server_index) {
        ui.heading(&server.name);
        ui.separator();

        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            for (channel_index, channel) in server.channels.iter().enumerate() {
                let this = crate::Selection::Channel { server: server_index, channel: channel_index };
                if super::list_button(ui, &channel.name, app.selected == Some(this)) {
                    app.selected = Some(this);
                }
            }
        });
    }
}

// Bottom half of the sidebar, resizable against the channel list above it.
pub fn draw_dm_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc, server_index: usize) {
    let half_height = ui.available_height() / 2.0;
    eframe::egui::Panel::bottom("dm_list")
        .resizable(true)
        .default_size(half_height)
        .frame(eframe::egui::Frame::NONE)
        .show(ui, |ui| {
            if let Some(server) = app.servers.get(server_index) {
                ui.heading("Direct Messages");
                ui.separator();

                eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                    for (dm_index, dm) in server.direct_messages.iter().enumerate() {
                        let this = crate::Selection::Dm { server: server_index, dm: dm_index };
                        if super::list_button(ui, &dm.nickname, app.selected == Some(this)) {
                            app.selected = Some(this);
                        }
                    }
                });
            }
        });
}
