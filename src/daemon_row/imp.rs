use std::cell::{OnceCell, RefCell};

use super::*;
use gtk::glib::Binding;
use gtk::subclass::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label, TemplateChild};
use crate::application::SystemdcontrolApplication;
use crate::daemon_object::DaemonObject;

#[derive(Default, CompositeTemplate, glib::Properties)]
#[template(resource = "/org/systemd/control/daemon_row.ui")]
#[properties(wrapper_type = super::DaemonRow)]
pub struct DaemonRow {
    #[template_child]
    daemon_row: TemplateChild<adw::ActionRow>,
    // #[template_child]
    // daemon_icon: TemplateChild<gtk::Image>,
    #[template_child]
    daemon_title: TemplateChild<Label>,
    // #[template_child]
    // daemon_subtitle: TemplateChild<gtk::Label>,
    #[property(get, set, construct_only)]
    daemon: OnceCell<DaemonObject>,
}

#[glib::object_subclass]
impl ObjectSubclass for DaemonRow {
    const NAME: &'static str = "SystemDManagerControlDaemonRow";
    type Type = super::DaemonRow;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template()
    }
}

#[glib::derived_properties]
impl ObjectImpl for DaemonRow {
    fn constructed(&self) {
        self.parent_constructed();
        let daemon = self.daemon
            .get()
            .unwrap();

        self.daemon_title.set_label(&*daemon.title());
    }
}

impl WidgetImpl for DaemonRow {}

impl BoxImpl for DaemonRow {}

impl ListBoxRowImpl for DaemonRow {}

