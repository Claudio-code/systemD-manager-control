mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};
use crate::daemon_object::DaemonObject;

glib::wrapper! {
    pub struct DaemonRow(ObjectSubclass<imp::DaemonRow>)
    @extends adw::ActionRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl DaemonRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, daemon_object: &DaemonObject) {
        // let startup_enabled_button = self.imp().startup_enabled_button.get();
        // let name_label = self.imp().name_label.get();
        // let mut binding = self.imp().binding.borrow_mut();

        // let completed_button_binding = daemon_object
        //     .bind_property("completed", &startup_enabled_button, "active")
        //     .bidirectional()
        //     .sync_create()
        //     .build();
        // binding.push(completed_button_binding);

        // let content_label_binding = daemon_object
        //     .bind_property("content", &name_label, "label")
        //     .sync_create()
        //     .build();
        // binding.push(content_label_binding);

        // let content_label_binding = daemon_object
        //     .bind_property("completed", &startup_enabled_button, "attributes")
        //     .sync_create()
        //     .transform_to(|_, active| {
        //         let attribute_list = AttrList::new();
        //         if active {
        //             let attribute = AttrInt::new_strikethrough(true);
        //             attribute_list.insert(attribute);
        //         }
        //         Some(attribute_list.to_value())
        //     })
        //     .build();
        // binding.push(content_label_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().binding.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
