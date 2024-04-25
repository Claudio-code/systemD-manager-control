mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct DaemonObject(ObjectSubclass<imp::DaemonObject>);
}

impl DaemonObject {
    pub fn new(startup_enabled: bool, title: String) -> Self {
        Object::builder()
            // .property("completed", startup_enabled)
            .property("title", title)
            .build()
    }

    pub fn is_enabled_on_startup(&self) -> bool {
        self.imp().data.borrow().startup_enabled
    }

    pub fn daemon_data(&self) -> DaemonData {
        self.imp().data.borrow().clone()
    }

    pub fn from_daemon_data(daemon_data: DaemonData) -> Self {
        Self::new(daemon_data.startup_enabled, daemon_data.title)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DaemonData {
    pub startup_enabled: bool,
    pub title: String,
}
