// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString};

use super::widget::WidgetImpl;
use crate::{ComboBox, Widget};

pub trait ComboBoxImpl: ComboBoxImplExt + WidgetImpl {
    fn changed(&self, combo_box: &Self::Type) {
        self.parent_changed(combo_box)
    }
    fn format_entry_text(&self, combo_box: &Self::Type, path: &str) -> Option<GString> {
        self.parent_format_entry_text(combo_box, path)
    }
}

pub trait ComboBoxImplExt: ObjectSubclass {
    fn parent_changed(&self, combo_box: &Self::Type);

    fn parent_format_entry_text(&self, combo_box: &Self::Type, path: &str) -> Option<GString>;
}

impl<T: ComboBoxImpl> ComboBoxImplExt for T {
    fn parent_changed(&self, combo_box: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkComboBoxClass;
            if let Some(f) = (*parent_class).changed {
                f(combo_box.unsafe_cast_ref::<ComboBox>().to_glib_none().0)
            }
        }
    }
    fn parent_format_entry_text(&self, combo_box: &Self::Type, path: &str) -> Option<GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkComboBoxClass;
            if let Some(f) = (*parent_class).format_entry_text {
                return Some(from_glib_full(f(
                    combo_box.unsafe_cast_ref::<ComboBox>().to_glib_none().0,
                    path.to_glib_none().0,
                )));
            }
            None
        }
    }
}

unsafe impl<T: ComboBoxImpl> IsSubclassable<T> for ComboBox {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.changed = Some(combo_box_changed::<T>);
        klass.format_entry_text = Some(combo_box_format_entry_text::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn combo_box_changed<T: ComboBoxImpl>(ptr: *mut ffi::GtkComboBox) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ComboBox> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn combo_box_format_entry_text<T: ComboBoxImpl>(
    ptr: *mut ffi::GtkComboBox,
    pathptr: *const libc::c_char,
) -> *mut libc::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ComboBox> = from_glib_borrow(ptr);

    let path: Borrowed<GString> = from_glib_borrow(pathptr);

    imp.format_entry_text(wrap.unsafe_cast_ref(), &path.as_str())
        .to_glib_full()
}
