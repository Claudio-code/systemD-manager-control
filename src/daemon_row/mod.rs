mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{ glib, pango };

use crate::daemon::{ self, Daemon };

glib::wrapper! {
    pub struct DaemonRow(ObjectSubclass<imp::DaemonRow>)
    @extends gtk::Widget, adw::ActionRow, gtk::ListBoxRow,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DaemonRow {
    pub fn new(daemon: Daemon) -> Self {
        Object::builder()
            .property("daemon", daemon.clone())
            .property("title", daemon.title())
            .build()
    }
}
