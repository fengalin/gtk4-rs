// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::CairoContext;
use crate::Cursor;
use crate::Device;
use crate::Display;
use crate::Event;
use crate::FrameClock;
use crate::GLContext;
use crate::ModifierType;
use crate::Monitor;
use crate::VulkanContext;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Surface(Object<ffi::GdkSurface, ffi::GdkSurfaceClass>);

    match fn {
        get_type => || ffi::gdk_surface_get_type(),
    }
}

impl Surface {
    #[doc(alias = "gdk_surface_new_popup")]
    pub fn new_popup(parent: &Surface, autohide: bool) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_surface_new_popup(
                parent.to_glib_none().0,
                autohide.to_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_surface_new_toplevel")]
    pub fn new_toplevel(display: &Display) -> Surface {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gdk_surface_new_toplevel(display.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_beep")]
    pub fn beep(&self) {
        unsafe {
            ffi::gdk_surface_beep(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_create_cairo_context")]
    pub fn create_cairo_context(&self) -> Option<CairoContext> {
        unsafe { from_glib_full(ffi::gdk_surface_create_cairo_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_create_gl_context")]
    pub fn create_gl_context(&self) -> Result<GLContext, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_surface_create_gl_context(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_surface_create_vulkan_context")]
    pub fn create_vulkan_context(&self) -> Result<VulkanContext, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_surface_create_vulkan_context(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_surface_destroy")]
    pub fn destroy(&self) {
        unsafe {
            ffi::gdk_surface_destroy(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_get_cursor")]
    pub fn cursor(&self) -> Option<Cursor> {
        unsafe { from_glib_none(ffi::gdk_surface_get_cursor(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_device_cursor")]
    pub fn get_device_cursor(&self, device: &Device) -> Option<Cursor> {
        unsafe {
            from_glib_none(ffi::gdk_surface_get_device_cursor(
                self.to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_surface_get_device_position")]
    pub fn get_device_position(&self, device: &Device) -> Option<(f64, f64, ModifierType)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut mask = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_surface_get_device_position(
                self.to_glib_none().0,
                device.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                mask.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            let mask = mask.assume_init();
            if ret {
                Some((x, y, from_glib(mask)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_surface_get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_surface_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_frame_clock")]
    pub fn frame_clock(&self) -> Option<FrameClock> {
        unsafe { from_glib_none(ffi::gdk_surface_get_frame_clock(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_get_mapped")]
    pub fn is_mapped(&self) -> bool {
        unsafe { from_glib(ffi::gdk_surface_get_mapped(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_scale_factor")]
    pub fn scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_scale_factor(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_hide")]
    pub fn hide(&self) {
        unsafe {
            ffi::gdk_surface_hide(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_is_destroyed")]
    pub fn is_destroyed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_surface_is_destroyed(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_queue_render")]
    pub fn queue_render(&self) {
        unsafe {
            ffi::gdk_surface_queue_render(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_request_layout")]
    pub fn request_layout(&self) {
        unsafe {
            ffi::gdk_surface_request_layout(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_set_cursor")]
    pub fn set_cursor(&self, cursor: Option<&Cursor>) {
        unsafe {
            ffi::gdk_surface_set_cursor(self.to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_set_device_cursor")]
    pub fn set_device_cursor(&self, device: &Device, cursor: &Cursor) {
        unsafe {
            ffi::gdk_surface_set_device_cursor(
                self.to_glib_none().0,
                device.to_glib_none().0,
                cursor.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_surface_set_input_region")]
    pub fn set_input_region(&self, region: &mut cairo::Region) {
        unsafe {
            ffi::gdk_surface_set_input_region(self.to_glib_none().0, region.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gdk_surface_set_opaque_region")]
    pub fn set_opaque_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gdk_surface_set_opaque_region(
                self.to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    pub fn connect_enter_monitor<F: Fn(&Surface, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_monitor_trampoline<F: Fn(&Surface, &Monitor) + 'static>(
            this: *mut ffi::GdkSurface,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter-monitor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_monitor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_event<F: Fn(&Surface, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<F: Fn(&Surface, &Event) -> bool + 'static>(
            this: *mut ffi::GdkSurface,
            event: *mut ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(event)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_layout<F: Fn(&Surface, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_trampoline<F: Fn(&Surface, i32, i32) + 'static>(
            this: *mut ffi::GdkSurface,
            width: libc::c_int,
            height: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), width, height)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_leave_monitor<F: Fn(&Surface, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn leave_monitor_trampoline<F: Fn(&Surface, &Monitor) + 'static>(
            this: *mut ffi::GdkSurface,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave-monitor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_monitor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_render<F: Fn(&Surface, &cairo::Region) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            F: Fn(&Surface, &cairo::Region) -> bool + 'static,
        >(
            this: *mut ffi::GdkSurface,
            region: *mut cairo::ffi::cairo_region_t,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(region)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    render_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_cursor_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut ffi::GdkSurface,
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
                b"notify::cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_height_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut ffi::GdkSurface,
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
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mapped_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mapped_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut ffi::GdkSurface,
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
                b"notify::mapped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mapped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scale_factor_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut ffi::GdkSurface,
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
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_width_notify<F: Fn(&Surface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<F: Fn(&Surface) + 'static>(
            this: *mut ffi::GdkSurface,
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
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Surface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Surface")
    }
}
