// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, FileChooser,
    FileChooserAction, FileFilter, LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkFileChooserWidget")]
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, FileChooser;

    match fn {
        type_ => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_file_chooser_widget_new")]
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileChooserWidget`] objects.
    ///
    /// This method returns an instance of [`FileChooserWidgetBuilder`](crate::builders::FileChooserWidgetBuilder) which can be used to create [`FileChooserWidget`] objects.
    pub fn builder() -> FileChooserWidgetBuilder {
        FileChooserWidgetBuilder::new()
    }

    #[doc(alias = "search-mode")]
    pub fn is_search_mode(&self) -> bool {
        ObjectExt::property(self, "search-mode")
    }

    #[doc(alias = "search-mode")]
    pub fn set_search_mode(&self, search_mode: bool) {
        ObjectExt::set_property(self, "search-mode", search_mode)
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "show-time")]
    pub fn shows_time(&self) -> bool {
        ObjectExt::property(self, "show-time")
    }

    pub fn subtitle(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "subtitle")
    }

    #[doc(alias = "desktop-folder")]
    pub fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn desktop_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"desktop-folder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    desktop_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_desktop_folder(&self) {
        self.emit_by_name::<()>("desktop-folder", &[]);
    }

    #[doc(alias = "down-folder")]
    pub fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down-folder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    down_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_down_folder(&self) {
        self.emit_by_name::<()>("down-folder", &[]);
    }

    #[doc(alias = "home-folder")]
    pub fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn home_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"home-folder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    home_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_home_folder(&self) {
        self.emit_by_name::<()>("home-folder", &[]);
    }

    #[doc(alias = "location-popup")]
    pub fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_trampoline<
            F: Fn(&FileChooserWidget, &str) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            path: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(path),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    location_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_popup(&self, path: &str) {
        self.emit_by_name::<()>("location-popup", &[&path]);
    }

    #[doc(alias = "location-popup-on-paste")]
    pub fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_on_paste_trampoline<
            F: Fn(&FileChooserWidget) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup-on-paste\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    location_popup_on_paste_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_popup_on_paste(&self) {
        self.emit_by_name::<()>("location-popup-on-paste", &[]);
    }

    #[doc(alias = "location-toggle-popup")]
    pub fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_toggle_popup_trampoline<
            F: Fn(&FileChooserWidget) + 'static,
        >(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-toggle-popup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    location_toggle_popup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_location_toggle_popup(&self) {
        self.emit_by_name::<()>("location-toggle-popup", &[]);
    }

    #[doc(alias = "places-shortcut")]
    pub fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn places_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"places-shortcut\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    places_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_places_shortcut(&self) {
        self.emit_by_name::<()>("places-shortcut", &[]);
    }

    #[doc(alias = "quick-bookmark")]
    pub fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn quick_bookmark_trampoline<F: Fn(&FileChooserWidget, i32) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            bookmark_index: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), bookmark_index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quick-bookmark\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    quick_bookmark_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_quick_bookmark(&self, bookmark_index: i32) {
        self.emit_by_name::<()>("quick-bookmark", &[&bookmark_index]);
    }

    #[doc(alias = "recent-shortcut")]
    pub fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn recent_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"recent-shortcut\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    recent_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_recent_shortcut(&self) {
        self.emit_by_name::<()>("recent-shortcut", &[]);
    }

    #[doc(alias = "search-shortcut")]
    pub fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_shortcut_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-shortcut\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    search_shortcut_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_search_shortcut(&self) {
        self.emit_by_name::<()>("search-shortcut", &[]);
    }

    #[doc(alias = "show-hidden")]
    pub fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_hidden_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-hidden\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    show_hidden_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_show_hidden(&self) {
        self.emit_by_name::<()>("show-hidden", &[]);
    }

    #[doc(alias = "up-folder")]
    pub fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_folder_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up-folder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    up_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_up_folder(&self) {
        self.emit_by_name::<()>("up-folder", &[]);
    }

    #[doc(alias = "search-mode")]
    pub fn connect_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_search_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "show-time")]
    pub fn connect_show_time_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_time_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-time\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subtitle")]
    pub fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<F: Fn(&FileChooserWidget) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FileChooserWidget {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileChooserWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileChooserWidgetBuilder {
    builder: glib::object::ObjectBuilder<'static, FileChooserWidget>,
}

impl FileChooserWidgetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn search_mode(self, search_mode: bool) -> Self {
        Self {
            builder: self.builder.property("search-mode", search_mode),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name<'a>(self, css_name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor<'a>(self, cursor: impl Into<Option<&'a gdk::Cursor>>) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.into()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager<'a, P: IsA<LayoutManager>>(
        self,
        layout_manager: impl Into<Option<&'a P>>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "layout-manager",
                layout_manager.into().as_ref().map(|p| p.as_ref()),
            ),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name<'a>(self, name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup<'a>(self, tooltip_markup: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text<'a>(self, tooltip_text: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn action(self, action: FileChooserAction) -> Self {
        Self {
            builder: self.builder.property("action", action),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn create_folders(self, create_folders: bool) -> Self {
        Self {
            builder: self.builder.property("create-folders", create_folders),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn filter<'a>(self, filter: impl Into<Option<&'a FileFilter>>) -> Self {
        Self {
            builder: self.builder.property("filter", filter.into()),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn select_multiple(self, select_multiple: bool) -> Self {
        Self {
            builder: self.builder.property("select-multiple", select_multiple),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileChooserWidget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileChooserWidget {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
