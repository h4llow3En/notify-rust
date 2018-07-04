//! **Experimental** server taking the place of your Desktop Environments Notification Server.
//!
//! This is not nearly meant for anything but testing, as it only prints notifications to stdout.
//! It does not respond properly either yet.
//!
//! This server will not replace an already running notification server.
//!

use std::cell::Cell;

use dbus::{Connection, BusType, NameFlag};
use dbus::tree::Factory;

use super::Notification;

static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";
/// Version of the crate equals the version server.
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// An **experimental** notification server.
/// See [the module level documentation](index.html) for more.
#[derive(Debug,Default)]
pub struct NotificationServer {
    /// Counter for generating notification ids
    pub counter: Cell<u32>,
    /// A flag that stops the server
    pub stop: Cell<bool>,
}

impl NotificationServer {
    fn count_up(&self) {
        self.counter.set(self.counter.get() + 1);
    }

    /// Create a new `NotificationServer` instance.
    pub fn new() -> NotificationServer {
        NotificationServer::default()
    }

    //pub fn notify_mothod<F>(&mut self, closure: F)
    //    -> Method
    //    where F: Fn(&Notification)
    //{

    //fn handle_notification

    /// Start listening for incoming notifications
    pub fn start<F>(&mut self, closure: F)
        where F: Fn(&Notification)
    {

        // org.freedesktop.Notifications
        // /org/freedesktop/Notifications
        // Notify
        //    app_name:    s
        //    replaces_id: u
        //    app_icon:    s
        //    summary:     s
        //    body:        s
        //    actions:    as
        //    hints:   a{sv}
        //    timeout:     i
        // Stop
        // CloseNotification
        //    id: u
        // GetCapabilities
        //    caps: {s}
        // GetServerInformation
        //
    // Let's start by starting up a connection to the session bus and register a name.
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("org.freedesktop.Notifications", NameFlag::ReplaceExisting as u32).unwrap();

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/org/freedesktop/Notifications", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("org.freedesktop.Notifications", ()).add_m(

            // ...and a method inside the interface.
            f.method("Notify", (), move |m| {

                // This is the callback that will be called when another peer on the bus calls our method.
                // the callback receives "MethodInfo" struct and can return either an error, or a list of
                // messages to send back.

                let name: &str = m.msg.read1()?;
                let s = format!("Hello {}!", name);
                let mret = m.msg.method_return().append1(s);


                // Two messages will be returned - one is the method return (and should always be there),
                // and in our case we also have a signal we want to send at the same time.
                Ok(vec!(mret))

            // Our method has one output argument and one input argument.
            }).outarg::<&str,_>("reply")
            .inarg::<&str,_>("name")

        // We also add the signal to the interface. This is mainly for introspection.
        )
    ));

    // We register all object paths in the tree.
    tree.set_registered(&c, true).unwrap();

    // We add the tree to the connection so that incoming method calls will be handled
    // automatically during calls to "incoming".
    c.add_handler(tree);

    // Serve other peers forever.
    loop { c.incoming(1000).next(); }

    }
}
