extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::{get_bundle_identifier_or_default, set_application};

fn main() -> Result<(), Box<std::error::Error>> {

    let safari_id = get_bundle_identifier_or_default("Safari");
    set_application(&safari_id)?;
    set_application(&safari_id)?;

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Safari")
        .icon("Safari")
        .show()?;

    Ok(())
}
