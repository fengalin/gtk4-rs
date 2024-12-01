// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use crate::ShortcutsSection;
use crate::{
    ffi, Accessible, AccessibleRole, Align, Application, Buildable, ConstraintTarget,
    LayoutManager, Native, Overflow, Root, ShortcutManager, Widget, Window,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkShortcutsWindow")]
    pub struct ShortcutsWindow(Object<ffi::GtkShortcutsWindow>) @extends Window, Widget, @implements Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;

    match fn {
        type_ => || ffi::gtk_shortcuts_window_get_type(),
    }
}

impl ShortcutsWindow {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ShortcutsWindow`] objects.
    ///
    /// This method returns an instance of [`ShortcutsWindowBuilder`](crate::builders::ShortcutsWindowBuilder) which can be used to create [`ShortcutsWindow`] objects.
    pub fn builder() -> ShortcutsWindowBuilder {
        ShortcutsWindowBuilder::new()
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_shortcuts_window_add_section")]
    pub fn add_section(&self, section: &ShortcutsSection) {
        unsafe {
            ffi::gtk_shortcuts_window_add_section(self.to_glib_none().0, section.to_glib_none().0);
        }
    }

    #[doc(alias = "section-name")]
    pub fn section_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "section-name")
    }

    #[doc(alias = "section-name")]
    pub fn set_section_name<'a>(&self, section_name: impl Into<Option<&'a str>>) {
        ObjectExt::set_property(self, "section-name", section_name.into())
    }

    #[doc(alias = "view-name")]
    pub fn view_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "view-name")
    }

    #[doc(alias = "view-name")]
    pub fn set_view_name<'a>(&self, view_name: impl Into<Option<&'a str>>) {
        ObjectExt::set_property(self, "view-name", view_name.into())
    }

    #[doc(alias = "close")]
    pub fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&ShortcutsWindow) + 'static>(
            this: *mut ffi::GtkShortcutsWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_close(&self) {
        self.emit_by_name::<()>("close", &[]);
    }

    #[doc(alias = "search")]
    pub fn connect_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_trampoline<F: Fn(&ShortcutsWindow) + 'static>(
            this: *mut ffi::GtkShortcutsWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_search(&self) {
        self.emit_by_name::<()>("search", &[]);
    }

    #[doc(alias = "section-name")]
    pub fn connect_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_section_name_trampoline<F: Fn(&ShortcutsWindow) + 'static>(
            this: *mut ffi::GtkShortcutsWindow,
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
                b"notify::section-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_section_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "view-name")]
    pub fn connect_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_name_trampoline<F: Fn(&ShortcutsWindow) + 'static>(
            this: *mut ffi::GtkShortcutsWindow,
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
                b"notify::view-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_view_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ShortcutsWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShortcutsWindowBuilder {
    builder: glib::object::ObjectBuilder<'static, ShortcutsWindow>,
}

impl ShortcutsWindowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn section_name<'a>(self, section_name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("section-name", section_name.into()),
        }
    }

    pub fn view_name<'a>(self, view_name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("view-name", view_name.into()),
        }
    }

    pub fn application<'a, P: IsA<Application>>(
        self,
        application: impl Into<Option<&'a P>>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "application",
                application.into().as_ref().map(|p| p.as_ref()),
            ),
        }
    }

    pub fn child<'a, P: IsA<Widget>>(self, child: impl Into<Option<&'a P>>) -> Self {
        Self {
            builder: self
                .builder
                .property("child", child.into().as_ref().map(|p| p.as_ref())),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget<'a, P: IsA<Widget>>(
        self,
        default_widget: impl Into<Option<&'a P>>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "default-widget",
                default_widget.into().as_ref().map(|p| p.as_ref()),
            ),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    pub fn display<'a, P: IsA<gdk::Display>>(self, display: impl Into<Option<&'a P>>) -> Self {
        Self {
            builder: self
                .builder
                .property("display", display.into().as_ref().map(|p| p.as_ref())),
        }
    }

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget<'a, P: IsA<Widget>>(self, focus_widget: impl Into<Option<&'a P>>) -> Self {
        Self {
            builder: self.builder.property(
                "focus-widget",
                focus_widget.into().as_ref().map(|p| p.as_ref()),
            ),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name<'a>(self, icon_name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id<'a>(self, startup_id: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title<'a>(self, title: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn titlebar<'a, P: IsA<Widget>>(self, titlebar: impl Into<Option<&'a P>>) -> Self {
        Self {
            builder: self
                .builder
                .property("titlebar", titlebar.into().as_ref().map(|p| p.as_ref())),
        }
    }

    pub fn transient_for<'a, P: IsA<Window>>(
        self,
        transient_for: impl Into<Option<&'a P>>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "transient-for",
                transient_for.into().as_ref().map(|p| p.as_ref()),
            ),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`ShortcutsWindow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ShortcutsWindow {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
