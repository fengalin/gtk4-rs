// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Sorter;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct TreeListRowSorter(Object<ffi::GtkTreeListRowSorter, ffi::GtkTreeListRowSorterClass>) @extends Sorter;

    match fn {
        get_type => || ffi::gtk_tree_list_row_sorter_get_type(),
    }
}

impl TreeListRowSorter {
    #[doc(alias = "gtk_tree_list_row_sorter_new")]
    pub fn new<P: IsA<Sorter>>(sorter: Option<&P>) -> TreeListRowSorter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_list_row_sorter_new(
                sorter.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_row_sorter_get_sorter")]
    pub fn sorter(&self) -> Option<Sorter> {
        unsafe {
            from_glib_none(ffi::gtk_tree_list_row_sorter_get_sorter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tree_list_row_sorter_set_sorter")]
    pub fn set_sorter<P: IsA<Sorter>>(&self, sorter: Option<&P>) {
        unsafe {
            ffi::gtk_tree_list_row_sorter_set_sorter(
                self.to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_sorter_notify<F: Fn(&TreeListRowSorter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<F: Fn(&TreeListRowSorter) + 'static>(
            this: *mut ffi::GtkTreeListRowSorter,
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
                b"notify::sorter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sorter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct TreeListRowSorterBuilder {
    sorter: Option<Sorter>,
}

impl TreeListRowSorterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeListRowSorter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref sorter) = self.sorter {
            properties.push(("sorter", sorter));
        }
        let ret = glib::Object::new::<TreeListRowSorter>(&properties).expect("object new");
        ret
    }

    pub fn sorter<P: IsA<Sorter>>(mut self, sorter: &P) -> Self {
        self.sorter = Some(sorter.clone().upcast());
        self
    }
}

impl fmt::Display for TreeListRowSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeListRowSorter")
    }
}
