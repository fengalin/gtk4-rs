// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, CellRenderer, CellRendererAccelMode, CellRendererMode, CellRendererText, TreePath,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCellRendererAccel")]
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel>) @extends CellRendererText, CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_accel_new")]
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererAccel`] objects.
    ///
    /// This method returns an instance of [`CellRendererAccelBuilder`](crate::builders::CellRendererAccelBuilder) which can be used to create [`CellRendererAccel`] objects.
    pub fn builder() -> CellRendererAccelBuilder {
        CellRendererAccelBuilder::new()
    }

    #[doc(alias = "accel-key")]
    pub fn accel_key(&self) -> u32 {
        ObjectExt::property(self, "accel-key")
    }

    #[doc(alias = "accel-key")]
    pub fn set_accel_key(&self, accel_key: u32) {
        ObjectExt::set_property(self, "accel-key", accel_key)
    }

    #[doc(alias = "accel-mode")]
    pub fn accel_mode(&self) -> CellRendererAccelMode {
        ObjectExt::property(self, "accel-mode")
    }

    #[doc(alias = "accel-mode")]
    pub fn set_accel_mode(&self, accel_mode: CellRendererAccelMode) {
        ObjectExt::set_property(self, "accel-mode", accel_mode)
    }

    #[doc(alias = "accel-mods")]
    pub fn accel_mods(&self) -> gdk::ModifierType {
        ObjectExt::property(self, "accel-mods")
    }

    #[doc(alias = "accel-mods")]
    pub fn set_accel_mods(&self, accel_mods: gdk::ModifierType) {
        ObjectExt::set_property(self, "accel-mods", accel_mods)
    }

    pub fn keycode(&self) -> u32 {
        ObjectExt::property(self, "keycode")
    }

    pub fn set_keycode(&self, keycode: u32) {
        ObjectExt::set_property(self, "keycode", keycode)
    }

    #[doc(alias = "accel-cleared")]
    pub fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accel_cleared_trampoline<
            F: Fn(&CellRendererAccel, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellRendererAccel,
            path_string: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path_string));
            f(&from_glib_borrow(this), path)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-cleared\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    accel_cleared_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-edited")]
    pub fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_edited_trampoline<
            F: Fn(&CellRendererAccel, TreePath, u32, gdk::ModifierType, u32) + 'static,
        >(
            this: *mut ffi::GtkCellRendererAccel,
            path_string: *mut std::ffi::c_char,
            accel_key: std::ffi::c_uint,
            accel_mods: gdk::ffi::GdkModifierType,
            hardware_keycode: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path_string));
            f(
                &from_glib_borrow(this),
                path,
                accel_key,
                from_glib(accel_mods),
                hardware_keycode,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accel-edited\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    accel_edited_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-key")]
    pub fn connect_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_key_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-key\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accel_key_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-mode")]
    pub fn connect_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_mode_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accel_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-mods")]
    pub fn connect_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accel_mods_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::accel-mods\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accel_mods_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "keycode")]
    pub fn connect_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_keycode_trampoline<F: Fn(&CellRendererAccel) + 'static>(
            this: *mut ffi::GtkCellRendererAccel,
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
                b"notify::keycode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_keycode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererAccel {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererAccel`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererAccelBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererAccel>,
}

impl CellRendererAccelBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn accel_key(self, accel_key: u32) -> Self {
        Self {
            builder: self.builder.property("accel-key", accel_key),
        }
    }

    pub fn accel_mode(self, accel_mode: CellRendererAccelMode) -> Self {
        Self {
            builder: self.builder.property("accel-mode", accel_mode),
        }
    }

    pub fn accel_mods(self, accel_mods: gdk::ModifierType) -> Self {
        Self {
            builder: self.builder.property("accel-mods", accel_mods),
        }
    }

    pub fn keycode(self, keycode: u32) -> Self {
        Self {
            builder: self.builder.property("keycode", keycode),
        }
    }

    pub fn align_set(self, align_set: bool) -> Self {
        Self {
            builder: self.builder.property("align-set", align_set),
        }
    }

    pub fn alignment(self, alignment: pango::Alignment) -> Self {
        Self {
            builder: self.builder.property("alignment", alignment),
        }
    }

    pub fn attributes<'a>(self, attributes: impl Into<Option<&'a pango::AttrList>>) -> Self {
        Self {
            builder: self.builder.property("attributes", attributes.into()),
        }
    }

    pub fn background<'a>(self, background: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("background", background.into()),
        }
    }

    pub fn background_rgba<'a>(self, background_rgba: impl Into<Option<&'a gdk::RGBA>>) -> Self {
        Self {
            builder: self
                .builder
                .property("background-rgba", background_rgba.into()),
        }
    }

    pub fn background_set(self, background_set: bool) -> Self {
        Self {
            builder: self.builder.property("background-set", background_set),
        }
    }

    pub fn editable(self, editable: bool) -> Self {
        Self {
            builder: self.builder.property("editable", editable),
        }
    }

    pub fn editable_set(self, editable_set: bool) -> Self {
        Self {
            builder: self.builder.property("editable-set", editable_set),
        }
    }

    pub fn ellipsize(self, ellipsize: pango::EllipsizeMode) -> Self {
        Self {
            builder: self.builder.property("ellipsize", ellipsize),
        }
    }

    pub fn ellipsize_set(self, ellipsize_set: bool) -> Self {
        Self {
            builder: self.builder.property("ellipsize-set", ellipsize_set),
        }
    }

    pub fn family<'a>(self, family: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("family", family.into()),
        }
    }

    pub fn family_set(self, family_set: bool) -> Self {
        Self {
            builder: self.builder.property("family-set", family_set),
        }
    }

    pub fn font<'a>(self, font: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("font", font.into()),
        }
    }

    pub fn font_desc<'a>(self, font_desc: impl Into<Option<&'a pango::FontDescription>>) -> Self {
        Self {
            builder: self.builder.property("font-desc", font_desc.into()),
        }
    }

    pub fn foreground<'a>(self, foreground: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("foreground", foreground.into()),
        }
    }

    pub fn foreground_rgba<'a>(self, foreground_rgba: impl Into<Option<&'a gdk::RGBA>>) -> Self {
        Self {
            builder: self
                .builder
                .property("foreground-rgba", foreground_rgba.into()),
        }
    }

    pub fn foreground_set(self, foreground_set: bool) -> Self {
        Self {
            builder: self.builder.property("foreground-set", foreground_set),
        }
    }

    pub fn language<'a>(self, language: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("language", language.into()),
        }
    }

    pub fn language_set(self, language_set: bool) -> Self {
        Self {
            builder: self.builder.property("language-set", language_set),
        }
    }

    pub fn markup<'a>(self, markup: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("markup", markup.into()),
        }
    }

    pub fn max_width_chars(self, max_width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("max-width-chars", max_width_chars),
        }
    }

    pub fn placeholder_text<'a>(self, placeholder_text: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder-text", placeholder_text.into()),
        }
    }

    pub fn rise(self, rise: i32) -> Self {
        Self {
            builder: self.builder.property("rise", rise),
        }
    }

    pub fn rise_set(self, rise_set: bool) -> Self {
        Self {
            builder: self.builder.property("rise-set", rise_set),
        }
    }

    pub fn scale(self, scale: f64) -> Self {
        Self {
            builder: self.builder.property("scale", scale),
        }
    }

    pub fn scale_set(self, scale_set: bool) -> Self {
        Self {
            builder: self.builder.property("scale-set", scale_set),
        }
    }

    pub fn single_paragraph_mode(self, single_paragraph_mode: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("single-paragraph-mode", single_paragraph_mode),
        }
    }

    pub fn size(self, size: i32) -> Self {
        Self {
            builder: self.builder.property("size", size),
        }
    }

    pub fn size_points(self, size_points: f64) -> Self {
        Self {
            builder: self.builder.property("size-points", size_points),
        }
    }

    pub fn size_set(self, size_set: bool) -> Self {
        Self {
            builder: self.builder.property("size-set", size_set),
        }
    }

    pub fn stretch(self, stretch: pango::Stretch) -> Self {
        Self {
            builder: self.builder.property("stretch", stretch),
        }
    }

    pub fn stretch_set(self, stretch_set: bool) -> Self {
        Self {
            builder: self.builder.property("stretch-set", stretch_set),
        }
    }

    pub fn strikethrough(self, strikethrough: bool) -> Self {
        Self {
            builder: self.builder.property("strikethrough", strikethrough),
        }
    }

    pub fn strikethrough_set(self, strikethrough_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("strikethrough-set", strikethrough_set),
        }
    }

    pub fn style(self, style: pango::Style) -> Self {
        Self {
            builder: self.builder.property("style", style),
        }
    }

    pub fn style_set(self, style_set: bool) -> Self {
        Self {
            builder: self.builder.property("style-set", style_set),
        }
    }

    pub fn text<'a>(self, text: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn underline(self, underline: pango::Underline) -> Self {
        Self {
            builder: self.builder.property("underline", underline),
        }
    }

    pub fn underline_set(self, underline_set: bool) -> Self {
        Self {
            builder: self.builder.property("underline-set", underline_set),
        }
    }

    pub fn variant(self, variant: pango::Variant) -> Self {
        Self {
            builder: self.builder.property("variant", variant),
        }
    }

    pub fn variant_set(self, variant_set: bool) -> Self {
        Self {
            builder: self.builder.property("variant-set", variant_set),
        }
    }

    pub fn weight(self, weight: i32) -> Self {
        Self {
            builder: self.builder.property("weight", weight),
        }
    }

    pub fn weight_set(self, weight_set: bool) -> Self {
        Self {
            builder: self.builder.property("weight-set", weight_set),
        }
    }

    pub fn width_chars(self, width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("width-chars", width_chars),
        }
    }

    pub fn wrap_mode(self, wrap_mode: pango::WrapMode) -> Self {
        Self {
            builder: self.builder.property("wrap-mode", wrap_mode),
        }
    }

    pub fn wrap_width(self, wrap_width: i32) -> Self {
        Self {
            builder: self.builder.property("wrap-width", wrap_width),
        }
    }

    pub fn cell_background<'a>(self, cell_background: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba<'a>(
        self,
        cell_background_rgba: impl Into<Option<&'a gdk::RGBA>>,
    ) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba.into()),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererAccel`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererAccel {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
