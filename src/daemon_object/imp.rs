use std::cell::RefCell;
use gtk::glib;
use gtk::glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::daemon_object::DaemonData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::DaemonObject)]
pub struct DaemonObject {
    #[property(name = "completed", get, set, type = bool, member = startup_enabled)]
    #[property(name = "title", get,  set, type = String, member = title)]
    pub data: RefCell<DaemonData>,
}

#[glib::object_subclass]
impl ObjectSubclass for DaemonObject {
    const NAME: &'static str = "SystemDManagerControlDaemonObject";
    type Type = super::DaemonObject;
}

#[glib::derived_properties]
impl ObjectImpl for DaemonObject {}
