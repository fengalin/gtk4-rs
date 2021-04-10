// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct WaylandDisplay(Object<ffi::GdkWaylandDisplay, ffi::GdkWaylandDisplayClass>) @extends gdk::Display;

    match fn {
        get_type => || ffi::gdk_wayland_display_get_type(),
    }
}

impl WaylandDisplay {
    #[doc(alias = "gdk_wayland_display_get_startup_notification_id")]
    pub fn startup_notification_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_wayland_display_get_startup_notification_id(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_wayland_display_query_registry")]
    pub fn query_registry(&self, global: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_wayland_display_query_registry(
                self.to_glib_none().0,
                global.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_wayland_display_set_cursor_theme")]
    pub fn set_cursor_theme(&self, name: &str, size: i32) {
        unsafe {
            ffi::gdk_wayland_display_set_cursor_theme(
                self.to_glib_none().0,
                name.to_glib_none().0,
                size,
            );
        }
    }

    #[doc(alias = "gdk_wayland_display_set_startup_notification_id")]
    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_wayland_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for WaylandDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WaylandDisplay")
    }
}
