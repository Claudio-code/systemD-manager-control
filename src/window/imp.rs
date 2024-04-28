use std::{cell::RefCell, sync::OnceLock};

use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, ListBox, Spinner, ScrolledWindow};
use tokio::runtime::Runtime;
use std::cell::OnceCell;


#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/systemd/control/window.ui")]
pub struct SystemdcontrolWindow {
    // Template widgets
    #[template_child]
    pub daemons_list: TemplateChild<ListBox>,
    #[template_child]
    pub scrolled_window: TemplateChild<ScrolledWindow>,
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for SystemdcontrolWindow {
    const NAME: &'static str = "SystemdcontrolWindow";
    type Type = super::SystemdcontrolWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SystemdcontrolWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.setup_default_list();
    }
}

impl WidgetImpl for SystemdcontrolWindow {}
impl WindowImpl for SystemdcontrolWindow {}
impl ApplicationWindowImpl for SystemdcontrolWindow {}

impl AdwApplicationWindowImpl for SystemdcontrolWindow  {
}