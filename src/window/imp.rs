use std::sync::OnceLock;

use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::{glib::{clone, PropertyGet}, prelude::*};
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, ListBox, Spinner, ScrolledWindow};
use tokio::runtime::Runtime;
use std::cell::OnceCell;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/systemd/control/window.ui")]
pub struct SystemdcontrolWindow {
    // Template widgets
    #[template_child]
    pub daemons_list: TemplateChild<ListBox>,
    #[template_child]
    pub scrolled_window: TemplateChild<ScrolledWindow>,
    #[template_child]
    pub spinner: TemplateChild<Spinner>,
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
        
        let filter = obj.filter_data();
        let (sender, receiver) = async_channel::bounded::<Vec<systemctl::Unit>>(1);
        obj.connect_show(move |_| {
            runtime().spawn(clone!(@strong sender, @strong filter => async move {
                let mut list_daemons: Vec<systemctl::Unit> = Vec::new();
                for item_name in systemctl::list_units(Some(&filter.filter_type), Some(&filter.state), None).unwrap() {
                    if let Ok(unit) = systemctl::Unit::from_systemctl(&*item_name) {
                        list_daemons.push(unit);
                    }
                }
                let _ = sender.send(list_daemons).await;
            }));
        });

        glib::spawn_future_local(clone!(@weak obj => async move {
            while let Ok(list_daemons) = receiver.recv().await {
                obj.set_daemons_in_list(list_daemons);
            }
        }));
    }
}

impl WidgetImpl for SystemdcontrolWindow {}
impl WindowImpl for SystemdcontrolWindow {}
impl ApplicationWindowImpl for SystemdcontrolWindow {}
impl AdwApplicationWindowImpl for SystemdcontrolWindow  {}