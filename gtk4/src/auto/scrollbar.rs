// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::AccessibleRange;
use crate::{
    ffi, Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, LayoutManager,
    Orientable, Orientation, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkScrollbar")]
    pub struct Scrollbar(Object<ffi::GtkScrollbar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AccessibleRange, Orientable;

    match fn {
        type_ => || ffi::gtk_scrollbar_get_type(),
    }
}

#[cfg(not(any(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkScrollbar")]
    pub struct Scrollbar(Object<ffi::GtkScrollbar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    #[doc(alias = "gtk_scrollbar_new")]
    pub fn new<'a, P: IsA<Adjustment>>(
        orientation: Orientation,
        adjustment: impl Into<Option<&'a P>>,
    ) -> Scrollbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(
                orientation.into_glib(),
                adjustment
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Scrollbar`] objects.
    ///
    /// This method returns an instance of [`ScrollbarBuilder`](crate::builders::ScrollbarBuilder) which can be used to create [`Scrollbar`] objects.
    pub fn builder() -> ScrollbarBuilder {
        ScrollbarBuilder::new()
    }

    #[doc(alias = "gtk_scrollbar_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    pub fn adjustment(&self) -> Adjustment {
        unsafe { from_glib_none(ffi::gtk_scrollbar_get_adjustment(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_scrollbar_set_adjustment")]
    #[doc(alias = "adjustment")]
    pub fn set_adjustment<'a, P: IsA<Adjustment>>(&self, adjustment: impl Into<Option<&'a P>>) {
        unsafe {
            ffi::gtk_scrollbar_set_adjustment(
                self.to_glib_none().0,
                adjustment
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
            );
        }
    }

    #[doc(alias = "adjustment")]
    pub fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<F: Fn(&Scrollbar) + 'static>(
            this: *mut ffi::GtkScrollbar,
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
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Scrollbar {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Scrollbar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ScrollbarBuilder {
    builder: glib::object::ObjectBuilder<'static, Scrollbar>,
}

impl ScrollbarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn adjustment<'a, P: IsA<Adjustment>>(self, adjustment: impl Into<Option<&'a P>>) -> Self {
        Self {
            builder: self
                .builder
                .property("adjustment", adjustment.into().as_ref().map(|p| p.as_ref())),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Scrollbar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Scrollbar {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
