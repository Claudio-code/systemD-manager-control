mod imp;

use std::collections::HashMap;

use gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CustomFilter};
use serde::{Deserialize, Serialize};

use crate::daemon::Daemon;
use crate::daemon_row::DaemonRow;

glib::wrapper! {
    pub struct SystemdControlWindow(ObjectSubclass<imp::SystemdControlWindow>)
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
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

    fn filter_data(&self) -> FilterData {
        let filter_type: String = self.settings().get("type");
        let filter_state: String = self.settings().get("state");
        FilterData {
            filter_type: filter_type,
            state: filter_state,
        }
    }

    fn filter(&self) -> Option<CustomFilter> {
        let filter_state: String = self.settings().get("type");

        match filter_state.as_str() {
            "All" => None,
            _ => unimplemented!(),
        }
    }

    fn set_daemons_in_list(&self, daemons: HashMap<String, systemctl::Unit>) {
        for unit_name in daemons.keys() {
            let unit = daemons.get(unit_name).unwrap();
            let daemon_row = DaemonRow::new(Daemon::new(unit_name, unit));
            self.imp().daemons_list.append(&daemon_row);
        }
        self.imp().spinner.set_visible(false);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterData {
    pub state: String,
    pub filter_type: String,
}
