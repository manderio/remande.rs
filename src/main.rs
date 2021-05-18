use notify_rust::Notification;

fn main() {
    notif()
}

fn notif() {
    Notification::new()
        .summary("Test")
        .body("This is the body")
        .show()
        .execpt("Failed to send notification");
}
