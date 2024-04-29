mod imp;

use std::sync::OnceLock;

use adw::prelude::ActionRowExt;
use gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    gio,
    glib,
    CheckButton,
    CustomFilter,
    FilterListModel,
    NoSelection,
    SignalListItemFactory,
};
use serde::{Deserialize, Serialize};
use systemctl::AutoStartStatus;
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

glib::wrapper! {
    pub struct SystemdcontrolWindow(ObjectSubclass<imp::SystemdcontrolWindow>)
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SystemdcontrolWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder().property("application", application).build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new("org.systemd.control");
        self.imp()
            .settings.set(settings)
            .expect("`settings` should be set before calling `setup_settings`");
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("`settings` should be set int `setup_settings`.")
    }

    fn filter_data(&self) -> FilterData {
        let filter_type: String = self.settings().get("type");
        let filter_state: String = self.settings().get("state");
        FilterData { filter_type: filter_type, state: filter_state }
    }

    fn filter(&self) -> Option<CustomFilter> {
        let filter_state: String = self.settings().get("type");

        match filter_state.as_str() {
            "All" => None,
            _ => unimplemented!(),
        }
    }

    fn set_daemons_in_list(&self, list_daemons: Vec<systemctl::Unit>) {
        for unit in list_daemons {
            let row = adw::ActionRow
                ::builder()
                .use_markup(false)
                .activatable(false)
                .title(&unit.name)
                .build();

            if unit.description.is_some() {
                row.set_subtitle(&unit.description.unwrap());
            }

            let start_button = gtk::Button
                ::builder()
                .icon_name("media-playback-start-symbolic")
                .valign(gtk::Align::Center)
                .tooltip_text("Start")
                .build();
            start_button.add_css_class("flat");

            let stop_button = gtk::Button
                ::builder()
                .icon_name("media-playback-stop-symbolic")
                .valign(gtk::Align::Center)
                .tooltip_text("Stop")
                .build();
            stop_button.add_css_class("flat");

            let restart_button = gtk::Button
                ::builder()
                .icon_name("object-rotate-right-symbolic")
                .valign(gtk::Align::Center)
                .tooltip_text("Restart")
                .build();
            restart_button.add_css_class("flat");

            let auto_startup = CheckButton::builder()
                .tooltip_text("Disabled on startup system")
                .sensitive(true)
                .build();
            auto_startup.add_css_class("flat");

            if unit.auto_start == AutoStartStatus::Enabled {
                auto_startup.set_active(true);
                auto_startup.set_tooltip_text(Some("Enabled on startup system"));
            }

            if unit.active {
                start_button.set_sensitive(false);
            } else {
                stop_button.set_sensitive(false);
            }

            row.add_suffix(&auto_startup);
            row.add_suffix(&start_button);
            row.add_suffix(&stop_button);
            row.add_suffix(&restart_button);
            self.imp().daemons_list.append(&row);
        }
        self.imp().spinner.set_visible(false);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterData {
    pub state: String,
    pub filter_type: String,
}

