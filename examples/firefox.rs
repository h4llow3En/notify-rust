extern crate notify_rust;
use notify_rust::Notification;
fn main() {
    Notification::new()
        .summary("I am not Firefox")
        .body("but I have it's icon :D")
        .auto_icon()
        .show();
}

