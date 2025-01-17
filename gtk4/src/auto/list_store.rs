// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::TreeDragDest;
use crate::TreeDragSource;
use crate::TreeIter;
use crate::TreeModel;
use crate::TreeSortable;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ListStore(Object<ffi::GtkListStore, ffi::GtkListStoreClass>) @implements Buildable, TreeDragDest, TreeDragSource, TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_list_store_get_type(),
    }
}

pub const NONE_LIST_STORE: Option<&ListStore> = None;

pub trait GtkListStoreExt: 'static {
    #[doc(alias = "gtk_list_store_append")]
    fn append(&self) -> TreeIter;

    #[doc(alias = "gtk_list_store_clear")]
    fn clear(&self);

    #[doc(alias = "gtk_list_store_insert")]
    fn insert(&self, position: i32) -> TreeIter;

    #[doc(alias = "gtk_list_store_insert_after")]
    fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter;

    #[doc(alias = "gtk_list_store_insert_before")]
    fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter;

    #[doc(alias = "gtk_list_store_iter_is_valid")]
    fn iter_is_valid(&self, iter: &TreeIter) -> bool;

    #[doc(alias = "gtk_list_store_move_after")]
    fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>);

    #[doc(alias = "gtk_list_store_move_before")]
    fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>);

    #[doc(alias = "gtk_list_store_prepend")]
    fn prepend(&self) -> TreeIter;

    #[doc(alias = "gtk_list_store_remove")]
    fn remove(&self, iter: &TreeIter) -> bool;

    #[doc(alias = "gtk_list_store_swap")]
    fn swap(&self, a: &TreeIter, b: &TreeIter);
}

impl<O: IsA<ListStore>> GtkListStoreExt for O {
    fn append(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_append(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_list_store_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn insert(&self, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                position,
            );
            iter
        }
    }

    fn insert_after(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_after(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    fn insert_before(&self, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_insert_before(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_iter_is_valid(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_after(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_list_store_move_before(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    fn prepend(&self) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_list_store_prepend(self.as_ref().to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_store_remove(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_list_store_swap(
                self.as_ref().to_glib_none().0,
                mut_override(a.to_glib_none().0),
                mut_override(b.to_glib_none().0),
            );
        }
    }
}

impl fmt::Display for ListStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListStore")
    }
}
