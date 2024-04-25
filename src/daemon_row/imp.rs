use std::cell::RefCell;

use gtk::glib::Binding;
use gtk::subclass::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label, TemplateChild};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/systemd/control/daemon_row.ui")]
pub struct DaemonRow {
    pub binding: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DaemonRow {
    const NAME: &'static str = "SystemDManagerControlDaemonRow";
    type Type = super::DaemonRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template()
    }
}

impl ObjectImpl for DaemonRow {}

impl WidgetImpl for DaemonRow {}

impl BoxImpl for DaemonRow {}

impl ListBoxRowImpl for DaemonRow {}

impl PreferencesRowImpl for DaemonRow {}

impl ActionRowImpl for DaemonRow {}
