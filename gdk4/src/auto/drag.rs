// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ContentFormats;
use crate::ContentProvider;
use crate::Device;
use crate::Display;
use crate::DragAction;
use crate::DragCancelReason;
use crate::Surface;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Drag(Object<ffi::GdkDrag>);

    match fn {
        get_type => || ffi::gdk_drag_get_type(),
    }
}

impl Drag {
    #[doc(alias = "gdk_drag_drop_done")]
    pub fn drop_done(&self, success: bool) {
        unsafe {
            ffi::gdk_drag_drop_done(self.to_glib_none().0, success.to_glib());
        }
    }

    #[doc(alias = "gdk_drag_get_actions")]
    pub fn actions(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drag_get_actions(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_content")]
    pub fn content(&self) -> Option<ContentProvider> {
        unsafe { from_glib_none(ffi::gdk_drag_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_device")]
    pub fn device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_drag_get_device(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_drag_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_drag_surface")]
    pub fn drag_surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_drag_get_drag_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_formats")]
    pub fn formats(&self) -> Option<ContentFormats> {
        unsafe { from_glib_none(ffi::gdk_drag_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_selected_action")]
    pub fn selected_action(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drag_get_selected_action(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_get_surface")]
    pub fn surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_drag_get_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drag_set_hotspot")]
    pub fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_set_hotspot(self.to_glib_none().0, hot_x, hot_y);
        }
    }

    #[doc(alias = "set_property_actions")]
    pub fn set_actions(&self, actions: DragAction) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"actions\0".as_ptr() as *const _,
                glib::Value::from(&actions).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "set_property_selected_action")]
    pub fn set_selected_action(&self, selected_action: DragAction) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"selected-action\0".as_ptr() as *const _,
                glib::Value::from(&selected_action).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_drag_begin")]
    pub fn begin<P: IsA<ContentProvider>>(
        surface: &Surface,
        device: &Device,
        content: &P,
        actions: DragAction,
        dx: f64,
        dy: f64,
    ) -> Option<Drag> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin(
                surface.to_glib_none().0,
                device.to_glib_none().0,
                content.as_ref().to_glib_none().0,
                actions.to_glib(),
                dx,
                dy,
            ))
        }
    }

    pub fn connect_cancel<F: Fn(&Drag, DragCancelReason) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<F: Fn(&Drag, DragCancelReason) + 'static>(
            this: *mut ffi::GdkDrag,
            reason: ffi::GdkDragCancelReason,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reason))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_dnd_finished<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dnd_finished_trampoline<F: Fn(&Drag) + 'static>(
            this: *mut ffi::GdkDrag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"dnd-finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    dnd_finished_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_drop_performed<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drop_performed_trampoline<F: Fn(&Drag) + 'static>(
            this: *mut ffi::GdkDrag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop-performed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_performed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_actions_notify<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&Drag) + 'static>(
            this: *mut ffi::GdkDrag,
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
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_display_notify<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&Drag) + 'static>(
            this: *mut ffi::GdkDrag,
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
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_selected_action_notify<F: Fn(&Drag) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_action_trampoline<F: Fn(&Drag) + 'static>(
            this: *mut ffi::GdkDrag,
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
                b"notify::selected-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_action_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Drag")
    }
}
