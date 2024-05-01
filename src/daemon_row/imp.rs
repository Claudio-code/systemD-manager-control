use std::{ borrow::Borrow, cell::{ OnceCell, RefCell }, sync::OnceLock };

use super::*;
use gtk::{glib::clone, Button};
use gtk::subclass::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::subclass::prelude::*;
use gtk::{ glib, CheckButton, CompositeTemplate, Label, TemplateChild };
use tokio::runtime::Runtime;
use crate::{ application::SystemdcontrolApplication, daemon::Daemon };

#[derive(Default, CompositeTemplate, glib::Properties)]
#[template(resource = "/org/systemd/control/daemon_row.ui")]
#[properties(wrapper_type = super::DaemonRow)]
pub struct DaemonRow {
    #[template_child]
    auto_start_button: TemplateChild<CheckButton>,
    #[template_child]
    start_button: TemplateChild<Button>,
    #[template_child]
    stop_button: TemplateChild<Button>,
    #[template_child]
    restart_button: TemplateChild<Button>,

    #[property(get, set, construct_only)]
    title: OnceCell<String>,
    #[property(get, set, construct_only)]
    pub daemon: OnceCell<Daemon>,
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

#[glib::derived_properties]
impl ObjectImpl for DaemonRow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj().clone();
        let daemon = obj.daemon();

        if daemon.active() {
            obj.imp().start_button.set_sensitive(false);    
        } else {
            obj.imp().stop_button.set_sensitive(false);
        }
        if daemon.has_subtitle() {
            obj.set_subtitle(&daemon.subtitle());
        }
        obj.imp().auto_start_button.set_active(daemon.is_auto_start());
    }
}

impl WidgetImpl for DaemonRow {}
impl BoxImpl for DaemonRow {}
impl ListBoxRowImpl for DaemonRow {}
impl ActionRowImpl for DaemonRow {}
impl PreferencesRowImpl for DaemonRow {}
