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
    pub struct MapListModel(Object<ffi::GtkMapListModel, ffi::GtkMapListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_map_list_model_get_type(),
    }
}

impl MapListModel {
    #[doc(alias = "gtk_map_list_model_new")]
    pub fn new<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> glib::Object + 'static>(
        model: Option<&P>,
        map_func: Q,
    ) -> MapListModel {
        assert_initialized_main_thread!();
        let map_func_data: Box_<Q> = Box_::new(map_func);
        unsafe extern "C" fn map_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> glib::Object + 'static,
        >(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::gobject_ffi::GObject {
            let item = from_glib_full(item);
            let callback: &Q = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let map_func = Some(map_func_func::<P, Q> as _);
        unsafe extern "C" fn user_destroy_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> glib::Object + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_destroy_func::<P, Q> as _);
        let super_callback0: Box_<Q> = map_func_data;
        unsafe {
            from_glib_full(ffi::gtk_map_list_model_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                map_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            ))
        }
    }

    #[doc(alias = "gtk_map_list_model_get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_map_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_map_list_model_has_map")]
    pub fn has_map(&self) -> bool {
        unsafe { from_glib(ffi::gtk_map_list_model_has_map(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_map_list_model_set_map_func")]
    pub fn set_map_func<P: Fn(&glib::Object) -> glib::Object + 'static>(&self, map_func: P) {
        let map_func_data: Box_<P> = Box_::new(map_func);
        unsafe extern "C" fn map_func_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::gobject_ffi::GObject {
            let item = from_glib_full(item);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let map_func = Some(map_func_func::<P> as _);
        unsafe extern "C" fn user_destroy_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<P> = map_func_data;
        unsafe {
            ffi::gtk_map_list_model_set_map_func(
                self.to_glib_none().0,
                map_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_map_list_model_set_model")]
    pub fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_map_list_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_has_map_notify<F: Fn(&MapListModel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_map_trampoline<F: Fn(&MapListModel) + 'static>(
            this: *mut ffi::GtkMapListModel,
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
                b"notify::has-map\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_map_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct MapListModelBuilder {
    model: Option<gio::ListModel>,
}

impl MapListModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MapListModel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        let ret = glib::Object::new::<MapListModel>(&properties).expect("object new");
        ret
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }
}

impl fmt::Display for MapListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MapListModel")
    }
}
