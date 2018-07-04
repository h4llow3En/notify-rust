extern crate notify_rust;
use notify_rust::Notification;

fn main() {

    let mut n:Notification = Default::default();

    n.summary("default").body("foo");

    let n = n; // make immutable
    n.show().unwrap();

}
