mod imp;

use std::collections::HashMap;
use std::sync::OnceLock;

use gio::Settings;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use tokio::runtime::Runtime;

use crate::daemon::Daemon;
use crate::daemon_row::DaemonRow;

glib::wrapper! {
    pub struct SystemdControlWindow(ObjectSubclass<imp::SystemdControlWindow>)
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

impl SystemdControlWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new("org.systemd.control");
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should be set before calling `setup_settings`");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set int `setup_settings`.")
    }

    fn filter(&self, daemon_type: Option<String>, auto_startup: Option<String>) {
        self.imp().spinner.set_visible(true);

        let filter_daemon: String = match daemon_type {
            Some(daemon_type) => daemon_type,
            None => self.settings().get("type"),
        };
        let filter_auto_startup = match auto_startup {
            Some(auto_startup) => auto_startup,
            None => self.settings().get("state"),
        };
        let (sender, receiver) = async_channel::bounded::<HashMap<String, systemctl::Unit>>(1);
        let _ = self.settings().set("state", filter_auto_startup.clone());
        let _ = self.settings().set("type", filter_daemon.clone());

        runtime().spawn(glib::clone!(@strong sender, @strong filter_daemon => async move {
            let mut daemons: HashMap<String, systemctl::Unit> = HashMap::new();
            let state_filter = match filter_auto_startup.as_str() {
                "All" => None,
                _ => Some(filter_auto_startup.as_str())
            };
            for item_name in systemctl::list_units(Some(&filter_daemon.as_str()), state_filter, None).unwrap() {
                if let Ok(unit) = systemctl::Unit::from_systemctl(&*item_name) {
                    daemons.insert(item_name, unit);
                }
            }
            let _ = sender.send(daemons).await;
        }));

        glib::spawn_future_local(clone!(@weak self as window => async move {
            while let Ok(daemons) = receiver.recv().await {
                window.set_daemons_in_list(daemons);
            }

        }));
    }

    fn set_daemons_in_list(&self, daemons: HashMap<String, systemctl::Unit>) {
        self.imp().daemons_list.remove_all();
        for unit_name in daemons.keys() {
            let unit = daemons.get(unit_name).unwrap();
            let daemon_row = DaemonRow::new(Daemon::new(unit_name, unit));
            self.imp().daemons_list.append(&daemon_row);
        }
        self.imp().spinner.set_visible(false);
    }

    fn set_actions(&self) {
        let action_filter_path = gio::ActionEntry::builder("filter-path")
            .activate(move |window: &Self, _, _| window.filter(Some(String::from("path")), None))
            .build();
        let action_filter_socket = gio::ActionEntry::builder("filter-socket")
            .activate(move |window: &Self, _, _| window.filter(Some(String::from("socket")), None))
            .build();
        let action_filter_timer = gio::ActionEntry::builder("filter-timer")
            .activate(move |window: &Self, _, _| window.filter(Some(String::from("timer")), None))
            .build();
        let action_filter_service = gio::ActionEntry::builder("filter-service")
            .activate(move |window: &Self, _, _| window.filter(Some(String::from("service")), None))
            .build();

        let action_filter_auto_start_disabled =
            gio::ActionEntry::builder("filter-auto-start-enabled")
                .activate(move |window: &Self, _, _| {
                    window.filter(None, Some(String::from("enabled")))
                })
                .build();
        let action_filter_auto_start_enabled =
            gio::ActionEntry::builder("filter-auto-start-disabled")
                .activate(move |window: &Self, _, _| {
                    window.filter(None, Some(String::from("disabled")))
                })
                .build();
        let action_filter_auto_start_all = gio::ActionEntry::builder("filter-auto-start-All")
            .activate(move |window: &Self, _, _| window.filter(None, Some(String::from("All"))))
            .build();

        self.add_action_entries([
            action_filter_path,
            action_filter_socket,
            action_filter_service,
            action_filter_timer,
            action_filter_auto_start_disabled,
            action_filter_auto_start_enabled,
            action_filter_auto_start_all,
        ]);
    }
}
