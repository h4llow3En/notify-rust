//! **Experimental** server taking the place of your Desktop Environments Notification Server.
//!
//! This is not nearly meant for anything but testing, as it only prints notifications to stdout.
//! It does not respond properly either yet.
//!
//! This server will not replace an already running notification server.
//!

#![allow(unused_imports, unused_variables, dead_code)]

use std::cell::Cell;
use std::collections::HashSet;

use dbus::tree::{self, Factory, Interface, MTFn, Tree};
use dbus::{arg, BusType, Connection, NameFlag, Path};

use super::{Notification, NotificationHint, Timeout};
use util::*;
use xdg::{NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH};

static DBUS_ERROR_FAILED: &str = "org.freedesktop.DBus.Error.Failed";
/// Version of the crate equals the version server.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// An **experimental** notification server.
/// See [the module level documentation](index.html) for more.
#[derive(Debug, Default)]
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

    // pub fn notify_mothod<F>(&mut self, closure: F)
    //    -> Method
    //    where F: Fn(&Notification)
    // {

    // fn handle_notification

    /// Start listening for incoming notifications
    pub fn start<F>(&mut self, closure: F)
    where
        F: Fn(&Notification),
    {
        let connection = Connection::get_private(BusType::Session).unwrap();

        connection.release_name(NOTIFICATION_NAMESPACE).unwrap();
        connection.register_name(NOTIFICATION_NAMESPACE, NameFlag::ReplaceExisting as u32).unwrap();
        connection.register_object_path(NOTIFICATION_OBJECTPATH).unwrap();

        let factory = Factory::new_fn::<()>(); // D::Tree = ()
        let tree = factory.tree(()).add(
            factory.object_path(NOTIFICATION_OBJECTPATH, ())
                .introspectable()
                .add(factory
                    .interface(NOTIFICATION_NAMESPACE, ())
                    .add_m(method_notify(&factory))
                    .add_m(method_close_notification(&factory))
                    // .add_m(stop_server(&factory))
                    // .add_signal(method_notification_closed(&factory))
                    // .add_signal(method_action_invoked(&factory))
                    .add_m(method_get_capabilities(&factory))
                    .add_m(method_get_server_information(&factory))
                ),
        );

        connection.add_handler(tree);

        loop {
            // Wait for incoming messages. This will block up to one second.
            // Discard the result - relevant messages have already been handled.
            if let Some(received) = connection.incoming(1000).next() {
                println!("RECEIVED {:?}", received);
            }
        }
    }
}

fn method_notify(factory: &Factory<MTFn>) -> tree::Method<MTFn<()>, ()> {
    factory.method("Notify", (), |minfo| {
        let mut i = minfo.msg.iter_init();
        let appname: String = i.read()?;
        let replaces_id: u32 = i.read()?;
        let icon: String = i.read()?;
        let summary: String = i.read()?;
        let body: String = i.read()?;
        let actions: Vec<String> = i.read()?;
        let hints: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> = i.read()?;
        let timeout: i32 = i.read()?;
        println!(
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} ",
            appname, replaces_id, icon, summary, body, actions, hints, timeout
        );
        // let arg0 = try!(d.notify(app_name, replaces_id, app_icon, summary, body, actions, hints, timeout));
        let notification = Notification{
            appname,
            icon,
            summary,
            body,
            actions,
            hints: Default::default(),
            timeout: Timeout::from(timeout),
            id: if replaces_id == 0 { None } else { Some(replaces_id) },
            subtitle: None,
        };
        println!("{:#?}", notification);
        let arg0 = 43;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(arg0);
        Ok(vec![rm])
    })
    .in_arg(("app_name", "s"))
    .in_arg(("replaces_id", "u"))
    .in_arg(("app_icon", "s"))
    .in_arg(("summary", "s"))
    .in_arg(("body", "s"))
    .in_arg(("actions", "as"))
    .in_arg(("hints", "a{sv}"))
    .in_arg(("timeout", "i"))
    .out_arg(("", "u"))
}

fn method_close_notification(factory: &Factory<MTFn>) -> tree::Method<MTFn<()>, ()> {
    factory.method("CloseNotification", (), |minfo| {
        let mut i = minfo.msg.iter_init();
        let id: u32 = try!(i.read());

        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    })
    .in_arg(("id", "u"))
}

fn method_get_capabilities(factory: &Factory<MTFn>) -> tree::Method<MTFn<()>, ()> {
    factory.method("GetCapabilities", (), |minfo| {
        let caps: Vec<String> = vec![];
        let rm = minfo.msg.method_return();
        let rm = rm.append1(caps);
        Ok(vec!(rm))
    })
    .out_arg(("caps", "as"))
}

fn method_get_server_information(factory: &Factory<MTFn>) -> tree::Method<MTFn<()>, ()> {
    factory.method("GetServerInformation", (), |minfo| {
        let (name, vendor, version, spec_version) = (
            "notify-rust", "notify-rust", env!("CARGO_PKG_VERSION"), "0.0.0"
        );
        let rm = minfo.msg.method_return();
        let rm = rm.append1(name);
        let rm = rm.append1(vendor);
        let rm = rm.append1(version);
        let rm = rm.append1(spec_version);
        Ok(vec!(rm))
    })
    .out_arg(("name", "s"))
    .out_arg(("vendor", "s"))
    .out_arg(("version", "s"))
    .out_arg(("spec_version", "s"))
}
