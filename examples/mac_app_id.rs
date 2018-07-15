extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::{get_bundle_identifier_or_default, set_application};

fn main() -> Result<(), String> {

    let safari_id = get_bundle_identifier_or_default("Safari");
    set_application(&safari_id)
        .map_err(|f| format!("{}", f))?;

    set_application(&safari_id)
        .map_err(|f| format!("{}", f))?;

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Safari")
        .icon("Safari")
        .show()
        .map_err(|f| format!("{}", f))?;

    Ok(())
}
