use adw::subclass::prelude::*;
use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, CompositeTemplate, ListBox, MenuButton, ScrolledWindow, Spinner};
use gtk::{prelude::*, SearchEntry};
use std::cell::{OnceCell, RefCell};

use crate::daemon::Daemon;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/systemd/control/window/window.ui")]
pub struct SystemdControlWindow {
    // Template widgets
    #[template_child]
    pub daemon_type_button: TemplateChild<MenuButton>,
    #[template_child]
    pub daemons_list: TemplateChild<ListBox>,
    #[template_child]
    pub scrolled_window: TemplateChild<ScrolledWindow>,
    #[template_child]
    pub spinner: TemplateChild<Spinner>,
    #[template_child]
    pub daemon_search_entry: TemplateChild<SearchEntry>,

    pub settings: OnceCell<Settings>,
    pub daemons: RefCell<Vec<Daemon>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SystemdControlWindow {
    const NAME: &'static str = "SystemdControlWindow";
    type Type = super::SystemdControlWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SystemdControlWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.set_actions();
        obj.set_search();
        obj.connect_show(glib::clone!(@weak obj => move |_| obj.filter(None, None)));
    }
}

impl WidgetImpl for SystemdControlWindow {}
impl WindowImpl for SystemdControlWindow {}
impl ApplicationWindowImpl for SystemdControlWindow {}
impl AdwApplicationWindowImpl for SystemdControlWindow {}
