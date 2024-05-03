mod imp;

use crate::daemon::Daemon;
use glib::Object;
use gtk::glib;

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
