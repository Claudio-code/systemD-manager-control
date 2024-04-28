mod imp;

use std::sync::OnceLock;

use adw::prelude::ActionRowExt;
use gio::Settings;
use glib::{clone, Object};
use gtk::glib::Variant;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ActionBar, CheckButton, CustomFilter, FilterListModel, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};
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

    fn filter(&self) -> Option<CustomFilter> {
        let filter_state: String = self.settings().get("filter");

        match filter_state.as_str() {
            "All" => None,
            _ => unimplemented!(),
        }
    }

    fn set_daemons_in_list(&self, list_daemons: Vec<String>) {
        for item_name in list_daemons {
            if let Ok(unit) = systemctl::Unit::from_systemctl(&*item_name) {
                let row = adw::ActionRow::builder()
                    .use_markup(false)
                    .activatable(false)
                    .title(&unit.name)
                    .build();

                if unit.description.is_some() {
                    row.set_subtitle(&unit.description.unwrap());
                }

                let start_button = gtk::Button::builder()
                    .icon_name("media-playback-start-symbolic")
                    .valign(gtk::Align::Center)
                    .tooltip_text("Start")
                    .build();
                start_button.add_css_class("flat");

                let stop_button = gtk::Button::builder()
                    .icon_name("media-playback-stop-symbolic")
                    .valign(gtk::Align::Center)
                    .tooltip_text("Stop")
                    .build();
                stop_button.add_css_class("flat");

                let restart_button = gtk::Button::builder()
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
        }
    }


    fn setup_default_list(&self) {
        let (sender, receiver) = async_channel::bounded::<Vec<String>>(1);
        runtime().spawn(clone!(@strong sender => async move {
            let list_daemons = systemctl::list_units(Some("path"), None, None).unwrap();
            let _ = sender.send_blocking(list_daemons);
        }));
        glib::spawn_future_local(clone!(@weak self as window => async move {
            while let Ok(list_daemons) = receiver.recv().await {
                window.set_daemons_in_list(list_daemons);
            }
        }));
    }
}

