// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, FontChooser,
    FontChooserLevel, LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkFontButton")]
    pub struct FontButton(Object<ffi::GtkFontButton>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, FontChooser;

    match fn {
        type_ => || ffi::gtk_font_button_get_type(),
    }
}

impl FontButton {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_new")]
    pub fn new() -> FontButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_font_button_new()).unsafe_cast() }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_new_with_font")]
    #[doc(alias = "new_with_font")]
    pub fn with_font(fontname: &str) -> FontButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new_with_font(
                fontname.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FontButton`] objects.
    ///
    /// This method returns an instance of [`FontButtonBuilder`](crate::builders::FontButtonBuilder) which can be used to create [`FontButton`] objects.
    pub fn builder() -> FontButtonBuilder {
        FontButtonBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_get_modal")]
    #[doc(alias = "get_modal")]
    #[doc(alias = "modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_modal(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_font_button_get_title(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_get_use_font")]
    #[doc(alias = "get_use_font")]
    #[doc(alias = "use-font")]
    pub fn uses_font(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_use_font(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_get_use_size")]
    #[doc(alias = "get_use_size")]
    #[doc(alias = "use-size")]
    pub fn uses_size(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_button_get_use_size(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_set_modal")]
    #[doc(alias = "modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_font_button_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_set_title")]
    #[doc(alias = "title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_font_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_set_use_font")]
    #[doc(alias = "use-font")]
    pub fn set_use_font(&self, use_font: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_font(self.to_glib_none().0, use_font.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_font_button_set_use_size")]
    #[doc(alias = "use-size")]
    pub fn set_use_size(&self, use_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_size(self.to_glib_none().0, use_size.into_glib());
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "font-set")]
    pub fn connect_font_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_set_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"font-set\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    font_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
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
                b"notify::modal\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-font")]
    pub fn connect_use_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_font_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
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
                b"notify::use-font\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_font_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-size")]
    pub fn connect_use_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_size_trampoline<F: Fn(&FontButton) + 'static>(
            this: *mut ffi::GtkFontButton,
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
                b"notify::use-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FontButton {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FontButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FontButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, FontButton>,
}

impl FontButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn title<'a>(self, title: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    pub fn use_font(self, use_font: bool) -> Self {
        Self {
            builder: self.builder.property("use-font", use_font),
        }
    }

    pub fn use_size(self, use_size: bool) -> Self {
        Self {
            builder: self.builder.property("use-size", use_size),
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
    pub fn font<'a>(self, font: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("font", font.into()),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn font_desc<'a>(self, font_desc: impl Into<Option<&'a pango::FontDescription>>) -> Self {
        Self {
            builder: self.builder.property("font-desc", font_desc.into()),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn language<'a>(self, language: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("language", language.into()),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn level(self, level: FontChooserLevel) -> Self {
        Self {
            builder: self.builder.property("level", level),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn preview_text<'a>(self, preview_text: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("preview-text", preview_text.into()),
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    pub fn show_preview_entry(self, show_preview_entry: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-preview-entry", show_preview_entry),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FontButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FontButton {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
