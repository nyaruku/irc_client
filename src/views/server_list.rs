pub fn draw_server_list(ui: &mut eframe::egui::Ui, app: &mut crate::Irc) {
    // pinned to the bottom; only a gap above, the outer panel has the rest
    eframe::egui::Panel::bottom("server_list_actions")
        .frame(eframe::egui::Frame::NONE.inner_margin(eframe::egui::Margin { top: 4, ..Default::default() }))
        .show(ui, |ui| {
            if super::list_button(ui, "Add server", false) {
                app.servers.push(crate::Server {
                    name: format!("New server {}", app.servers.len() + 1),
                    channels: Vec::new(),
                    direct_messages: Vec::new(),
                });
                app.open_server = Some(app.servers.len() - 1);
            }
        });

    eframe::egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, server) in app.servers.iter().enumerate() {
            if super::list_button(ui, &server.name, app.open_server == Some(i)) {
                app.open_server = Some(i);
            }
        }
    });
}
