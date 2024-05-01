use std::{collections::HashMap, sync::OnceLock};

use adw::subclass::prelude::*;
use gio::Settings;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, CompositeTemplate, ListBox, ScrolledWindow, Spinner};
use gtk::{glib::clone, prelude::*};
use std::cell::OnceCell;
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/systemd/control/window.ui")]
pub struct SystemdControlWindow {
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
        let filter = obj.filter_data();
        let (sender, receiver) = async_channel::bounded::<HashMap<String, systemctl::Unit>>(1);

        obj.connect_show(move |_| {
            runtime().spawn(
                clone!(@strong sender, @strong filter => async move {
                    let mut daemons: HashMap<String, systemctl::Unit> = HashMap::new();
                    for item_name in systemctl::list_units(Some(&filter.filter_type), Some(&filter.state), None).unwrap() {
                        if let Ok(unit) = systemctl::Unit::from_systemctl(&*item_name) {
                            daemons.insert(item_name, unit);
                        }
                    }
                    let _ = sender.send(daemons).await;
                })
            );
        });

        glib::spawn_future_local(clone!(@weak obj => async move {
            while let Ok(daemons) = receiver.recv().await {
                obj.set_daemons_in_list(daemons);
            }
        }));
    }
}

impl WidgetImpl for SystemdControlWindow {}
impl WindowImpl for SystemdControlWindow {}
impl ApplicationWindowImpl for SystemdControlWindow {}
impl AdwApplicationWindowImpl for SystemdControlWindow {}
