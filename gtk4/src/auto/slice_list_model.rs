// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

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
    pub struct SliceListModel(Object<ffi::GtkSliceListModel, ffi::GtkSliceListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_slice_list_model_get_type(),
    }
}

impl SliceListModel {
    #[doc(alias = "gtk_slice_list_model_new")]
    pub fn new<P: IsA<gio::ListModel>>(
        model: Option<&P>,
        offset: u32,
        size: u32,
    ) -> SliceListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_slice_list_model_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                offset,
                size,
            ))
        }
    }

    #[doc(alias = "gtk_slice_list_model_get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_slice_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_slice_list_model_get_offset")]
    pub fn offset(&self) -> u32 {
        unsafe { ffi::gtk_slice_list_model_get_offset(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_slice_list_model_get_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::gtk_slice_list_model_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_slice_list_model_set_model")]
    pub fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_slice_list_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_slice_list_model_set_offset")]
    pub fn set_offset(&self, offset: u32) {
        unsafe {
            ffi::gtk_slice_list_model_set_offset(self.to_glib_none().0, offset);
        }
    }

    #[doc(alias = "gtk_slice_list_model_set_size")]
    pub fn set_size(&self, size: u32) {
        unsafe {
            ffi::gtk_slice_list_model_set_size(self.to_glib_none().0, size);
        }
    }

    pub fn connect_property_model_notify<F: Fn(&SliceListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&SliceListModel) + 'static>(
            this: *mut ffi::GtkSliceListModel,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_offset_notify<F: Fn(&SliceListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<F: Fn(&SliceListModel) + 'static>(
            this: *mut ffi::GtkSliceListModel,
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
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_size_notify<F: Fn(&SliceListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<F: Fn(&SliceListModel) + 'static>(
            this: *mut ffi::GtkSliceListModel,
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
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct SliceListModelBuilder {
    model: Option<gio::ListModel>,
    offset: Option<u32>,
    size: Option<u32>,
}

impl SliceListModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SliceListModel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref offset) = self.offset {
            properties.push(("offset", offset));
        }
        if let Some(ref size) = self.size {
            properties.push(("size", size));
        }
        let ret = glib::Object::new::<SliceListModel>(&properties).expect("object new");
        ret
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }
}

impl fmt::Display for SliceListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SliceListModel")
    }
}
