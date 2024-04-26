mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};
use crate::daemon_object::DaemonObject;

glib::wrapper! {
    pub struct DaemonRow(ObjectSubclass<imp::DaemonRow>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DaemonRow {
    pub fn new(daemon_object: DaemonObject) -> Self {
        Object::builder()
            .property("daemon", daemon_object)
            .build()
    }
}
