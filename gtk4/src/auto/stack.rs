// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    SelectionModel, StackPage, StackTransitionType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkStack")]
    pub struct Stack(Object<ffi::GtkStack>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[doc(alias = "gtk_stack_new")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_stack_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Stack`] objects.
    ///
    /// This method returns an instance of [`StackBuilder`](crate::builders::StackBuilder) which can be used to create [`Stack`] objects.
    pub fn builder() -> StackBuilder {
        StackBuilder::new()
    }

    #[doc(alias = "gtk_stack_add_child")]
    pub fn add_child(&self, child: &impl IsA<Widget>) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_add_named")]
    pub fn add_named<'a>(
        &self,
        child: &impl IsA<Widget>,
        name: impl Into<Option<&'a str>>,
    ) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_named(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.into().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_add_titled")]
    pub fn add_titled<'a>(
        &self,
        child: &impl IsA<Widget>,
        name: impl Into<Option<&'a str>>,
        title: &str,
    ) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_titled(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.into().to_glib_none().0,
                title.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_child_by_name")]
    #[doc(alias = "get_child_by_name")]
    pub fn child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_hhomogeneous")]
    #[doc(alias = "get_hhomogeneous")]
    #[doc(alias = "hhomogeneous")]
    pub fn is_hhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_interpolate_size")]
    #[doc(alias = "get_interpolate_size")]
    #[doc(alias = "interpolate-size")]
    pub fn interpolates_size(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_interpolate_size(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<Widget>) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> SelectionModel {
        unsafe { from_glib_full(ffi::gtk_stack_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_transition_duration")]
    #[doc(alias = "get_transition_duration")]
    #[doc(alias = "transition-duration")]
    pub fn transition_duration(&self) -> u32 {
        unsafe { ffi::gtk_stack_get_transition_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_stack_get_transition_running")]
    #[doc(alias = "get_transition_running")]
    #[doc(alias = "transition-running")]
    pub fn is_transition_running(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_transition_type")]
    #[doc(alias = "get_transition_type")]
    #[doc(alias = "transition-type")]
    pub fn transition_type(&self) -> StackTransitionType {
        unsafe { from_glib(ffi::gtk_stack_get_transition_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_vhomogeneous")]
    #[doc(alias = "get_vhomogeneous")]
    #[doc(alias = "vhomogeneous")]
    pub fn is_vhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    #[doc(alias = "visible-child")]
    pub fn visible_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_visible_child_name")]
    #[doc(alias = "get_visible_child_name")]
    #[doc(alias = "visible-child-name")]
    pub fn visible_child_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_remove")]
    pub fn remove(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_stack_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_stack_set_hhomogeneous")]
    #[doc(alias = "hhomogeneous")]
    pub fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_interpolate_size")]
    #[doc(alias = "interpolate-size")]
    pub fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(
                self.to_glib_none().0,
                interpolate_size.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_stack_set_transition_duration")]
    #[doc(alias = "transition-duration")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "gtk_stack_set_transition_type")]
    #[doc(alias = "transition-type")]
    pub fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_vhomogeneous")]
    #[doc(alias = "vhomogeneous")]
    pub fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child")]
    #[doc(alias = "visible-child")]
    pub fn set_visible_child(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_stack_set_visible_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child_full")]
    pub fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(
                self.to_glib_none().0,
                name.to_glib_none().0,
                transition.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child_name")]
    #[doc(alias = "visible-child-name")]
    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "hhomogeneous")]
    pub fn connect_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hhomogeneous_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::hhomogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interpolate-size")]
    pub fn connect_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_interpolate_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-duration")]
    pub fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-duration\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transition_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-running")]
    pub fn connect_transition_running_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-running\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transition_running_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-type")]
    pub fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transition_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vhomogeneous")]
    pub fn connect_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vhomogeneous_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::vhomogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child")]
    pub fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child-name")]
    pub fn connect_visible_child_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_name_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::visible-child-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_child_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Stack`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StackBuilder {
    builder: glib::object::ObjectBuilder<'static, Stack>,
}

impl StackBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn hhomogeneous(self, hhomogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("hhomogeneous", hhomogeneous),
        }
    }

    pub fn interpolate_size(self, interpolate_size: bool) -> Self {
        Self {
            builder: self.builder.property("interpolate-size", interpolate_size),
        }
    }

    pub fn transition_duration(self, transition_duration: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("transition-duration", transition_duration),
        }
    }

    pub fn transition_type(self, transition_type: StackTransitionType) -> Self {
        Self {
            builder: self.builder.property("transition-type", transition_type),
        }
    }

    pub fn vhomogeneous(self, vhomogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("vhomogeneous", vhomogeneous),
        }
    }

    pub fn visible_child<'a, P: IsA<Widget>>(
        self,
        visible_child: impl Into<Option<&'a P>>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "visible-child",
                visible_child.into().as_ref().map(|p| p.as_ref()),
            ),
        }
    }

    pub fn visible_child_name<'a>(self, visible_child_name: impl Into<Option<&'a str>>) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-child-name", visible_child_name.into()),
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
    /// Build the [`Stack`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Stack {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
