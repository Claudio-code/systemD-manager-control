use gtk::glib;
use systemctl::AutoStartStatus;

#[derive(Default, Clone, Debug, PartialEq, glib::Boxed)]
#[boxed_type(name = "Daemon")]
pub struct Daemon {
    unit_name: String,
    unit: systemctl::Unit,
}

impl Daemon {
    pub fn new(unit_name: &String, unit: &systemctl::Unit) -> Self {
        Self {
            unit_name: unit_name.to_string(),
            unit: unit.clone(),
        }
    }

    pub fn title(&self) -> String {
        self.unit.name.to_string()
    }

    pub fn subtitle(&self) -> String {
        let description = self.unit.description.clone().unwrap();
        description.replace("&", "&amp;")
    }

    pub fn has_subtitle(&self) -> bool {
        let description = self.unit.description.clone();
        description.is_some()
    }

    pub fn active(&self) -> bool {
        self.unit.active
    }

    pub fn is_auto_start(&self) -> bool {
        self.unit.auto_start == AutoStartStatus::Enabled
    }

    pub fn stop(&self) {
        let _ = self.unit.stop();
    }

    pub fn start(&self) {
        let _ = self.unit.start();
    }

    pub fn restart(&self) {
        let _ = self.unit.restart();
    }

    pub fn change_auto_start(&self) {
        if self.is_auto_start() {
            let _ = self.unit.disable();
        } else {
            let _ = self.unit.enable();
        }
    }
}
