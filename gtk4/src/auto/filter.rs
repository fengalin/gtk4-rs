// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FilterChange;
use crate::FilterMatch;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Filter(Object<ffi::GtkFilter, ffi::GtkFilterClass>);

    match fn {
        get_type => || ffi::gtk_filter_get_type(),
    }
}

pub const NONE_FILTER: Option<&Filter> = None;

pub trait FilterExt: 'static {
    #[doc(alias = "gtk_filter_changed")]
    fn changed(&self, change: FilterChange);

    #[doc(alias = "gtk_filter_get_strictness")]
    fn strictness(&self) -> FilterMatch;

    #[doc(alias = "gtk_filter_match")]
    fn match_<P: IsA<glib::Object>>(&self, item: &P) -> bool;

    fn connect_changed<F: Fn(&Self, FilterChange) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Filter>> FilterExt for O {
    fn changed(&self, change: FilterChange) {
        unsafe {
            ffi::gtk_filter_changed(self.as_ref().to_glib_none().0, change.to_glib());
        }
    }

    fn strictness(&self) -> FilterMatch {
        unsafe {
            from_glib(ffi::gtk_filter_get_strictness(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn match_<P: IsA<glib::Object>>(&self, item: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_filter_match(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_changed<F: Fn(&Self, FilterChange) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P, FilterChange) + 'static>(
            this: *mut ffi::GtkFilter,
            change: ffi::GtkFilterChange,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Filter>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Filter::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(change),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Filter")
    }
}
