use notify_rust::Notification;

fn main() {}

fn notif() {
    Notification::new()
        .summary("Test")
        .body("This is the body")
        .show();
}
