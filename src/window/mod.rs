mod imp;

use adw::prelude::ActionRowExt;
use gio::Settings;
use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CustomFilter, FilterListModel, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};

use crate::daemon_object::{DaemonData, DaemonObject};
use crate::daemon_row::DaemonRow;


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

    fn daemons(&self) -> gio::ListStore {
        self.imp()
            .daemons
            .borrow()
            .clone()
            .expect("Could not get current Daemons.")
    }

    fn filter(&self) -> Option<CustomFilter> {
        let filter_state: String = self.settings().get("filter");
        let filter_open = CustomFilter::new(|obj| {
            !obj.downcast_ref::<DaemonObject>()
                .expect("The object needs to be of type `TaskObject`.")
                .is_enabled_on_startup()
        });
        let filter_done = CustomFilter::new(|obj| {
            obj.downcast_ref::<DaemonObject>()
                .expect("The object needs to be of type `TaskObject`.")
                .is_enabled_on_startup()
        });
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unimplemented!(),
        }
    }

    fn setup_daemons(&self) {
        let model = gio::ListStore::new::<DaemonObject>();
        self.imp().daemons.replace(Some(model));
        let filter_model = FilterListModel::new(Some(self.daemons()), self.filter());
        let selection_model = NoSelection::new(Some(filter_model.clone()));


        let mut daemon_data2 = DaemonRow::new();
        daemon_data2.set_subtitle("dqwdqw");
        self.imp().daemons_list.append(&daemon_data2);
    }

    fn restore_data(&self) {
        let mut vec: Vec<DaemonObject> = Vec::new();
        let daemon_data = DaemonData { title: String::from("teste"), startup_enabled: false };
        let daemon_data2 = DaemonData { title: String::from("teste2"), startup_enabled: true };
        vec.push(DaemonObject::from_daemon_data(daemon_data));
        vec.push(DaemonObject::from_daemon_data(daemon_data2));
        self.daemons().extend_from_slice(&vec);
    }
}
