pub struct Message {
    username: String,
    message: String,
    timestamp: i64,
}

pub fn add_message(messages: &mut Vec<Message>, username: String, message: String, timestamp: i64) {
    messages.push(Message {
        username,
        message,
        timestamp,
    });
}

// HH:MM:SS in local time, empty if the timestamp is out of range
fn format_time(secs: i64) -> String {
    chrono::DateTime::from_timestamp(secs, 0)
        .map(|time| time.with_timezone(&chrono::Local).format("%H:%M:%S").to_string())
        .unwrap_or_default()
}

/// Returns the typed text when Enter is pressed.
pub fn draw_chat(ui: &mut eframe::egui::Ui, messages: &[Message], input: &mut String) -> Option<String> {
    let mut sent = None;
    eframe::egui::Panel::bottom("chat_input").frame(super::panel_frame(ui)).show(ui, |ui| {
        let response = ui.add(
            eframe::egui::TextEdit::singleline(input)
                .hint_text("Message")
                .desired_width(f32::INFINITY),
        );
        // singleline loses focus on Enter
        if response.lost_focus() && ui.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
            if !input.trim().is_empty() {
                sent = Some(std::mem::take(input));
            }
            response.request_focus();
        }
    });

    eframe::egui::CentralPanel::default().frame(super::panel_frame(ui)).show(ui, |ui| {
        // bottom-up so the scroll area sits at the bottom while it is shorter than the panel
        ui.with_layout(eframe::egui::Layout::bottom_up(eframe::egui::Align::LEFT), |ui| {
            eframe::egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in messages {
                        ui.label(format!(
                            "[{}] <{}> {}",
                            format_time(message.timestamp),
                            message.username,
                            message.message
                        ));
                    }
                });
        });
    });
    sent
}
