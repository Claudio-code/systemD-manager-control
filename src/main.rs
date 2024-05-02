/* main.rs
 *
 * Copyright 2024 soneca
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod daemon;
mod daemon_row;
mod application;
mod config;
mod window;

use self::application::SystemdcontrolApplication;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::prelude::*;
use gtk::{gio, glib};
use gvdb_macros::include_gresource_from_dir;


const APP_ID: &str = "org.systemd.control";

const DBUS_API_PATH: &str = const_str::concat!("/", const_str::replace!(APP_ID, ".", "/"));

static GRESOURCE_BYTES: &[u8] =
    if const_str::equal!("/org/systemd/control", DBUS_API_PATH) {
        include_gresource_from_dir!("/org/systemd/control", "data/resources")
    } else if const_str::equal!("/org/systemd/control/Devel", DBUS_API_PATH) {
        include_gresource_from_dir!("/org/systemd/control/Devel", "data/resources")
    } else {
        panic!("Invalid DBUS_API_PATH")
    };

fn main() -> glib::ExitCode {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/systemdcontrol.gresource")
        .expect("Could not load resources");
    let resource_css = gio::Resource::from_data(&glib::Bytes::from_static(GRESOURCE_BYTES))
        .expect("Could not load resources");
    gio::resources_register(&resources);
    gio::resources_register(&resource_css);

    let app = SystemdcontrolApplication::new(APP_ID, &gio::ApplicationFlags::empty());
    app.run()
}
