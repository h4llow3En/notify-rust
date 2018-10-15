#![allow(unused_must_use)]
extern crate notify_rust;
use std::thread;
use std::time::Duration;

#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::server::NotificationServer;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationUrgency::*;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

fn freeze(message: &str) {
    println!("{}", message);
    // let mut _devnull = String::new();
    // let _ = std::io::stdin().read_line(&mut _devnull);
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    let mut server = NotificationServer::new();
    // thread::spawn(move || server.start(|notification| println!(" -- {:#?} --", notification)));
    thread::spawn(move || server.start(
        |notification|
        println!(" --> {:?}\n", notification.hints)
    ));

    std::thread::sleep(Duration::from_millis(500));

    freeze("actionicons");
    Notification::new().hint(Hint::ActionIcons(true)).show();
    Notification::new().hint(Hint::ActionIcons(false)).show();

    freeze("urgency: low, medium, high");
    Notification::new().hint(Hint::Urgency(Low)).show();
    Notification::new().hint(Hint::Urgency(Normal)).show();
    Notification::new().hint(Hint::Urgency(Critical)).show();

    freeze("category");
    Notification::new().hint(Hint::Category("device.removed".into())).show();

    freeze("DesktopEntry");
    Notification::new().hint(Hint::DesktopEntry("firefox".into())).show();

    freeze("ImagePath");
    Notification::new().hint(Hint::ImagePath("/usr/share/icons/hicolor/128x128/apps/firefox.png".into()))
                       .show();

    freeze("Resident");
    Notification::new().hint(Hint::Resident(true)).show();

    freeze("SoundFile");
    Notification::new().hint(Hint::SoundFile("/usr/share/sounds/alsa/Front_Left.wav".to_owned()))
                       .hint(Hint::SoundName("system sound".to_owned()))
                       .hint(Hint::SuppressSound(false))
                       .show();

    freeze("Transient");
    Notification::new().hint(Hint::Transient(false)).show();

    freeze("X and Y");
    Notification::new().hint(Hint::X(200)).hint(Hint::Y(200)).show();




    // println!("Press enter to exit.\n");
    // let mut _devnull = String::new();
    // let _ = std::io::stdin().read_line(&mut _devnull);
    // println!("Thank you for choosing notify-rust.");
}
