// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::NumberUpLayout;
use crate::PageRange;
use crate::PageSet;
use crate::PageSetup;
use crate::PrintPages;
use crate::PrintSettings;
use crate::PrintStatus;
use crate::Printer;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct PrintJob(Object<ffi::GtkPrintJob>);

    match fn {
        get_type => || ffi::gtk_print_job_get_type(),
    }
}

impl PrintJob {
    #[doc(alias = "gtk_print_job_new")]
    pub fn new(
        title: &str,
        printer: &Printer,
        settings: &PrintSettings,
        page_setup: &PageSetup,
    ) -> PrintJob {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_print_job_new(
                title.to_glib_none().0,
                printer.to_glib_none().0,
                settings.to_glib_none().0,
                page_setup.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_job_get_collate")]
    pub fn is_collate(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_job_get_collate(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_n_up")]
    pub fn n_up(&self) -> u32 {
        unsafe { ffi::gtk_print_job_get_n_up(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_job_get_n_up_layout")]
    pub fn n_up_layout(&self) -> NumberUpLayout {
        unsafe { from_glib(ffi::gtk_print_job_get_n_up_layout(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_num_copies")]
    pub fn num_copies(&self) -> i32 {
        unsafe { ffi::gtk_print_job_get_num_copies(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_job_get_page_ranges")]
    pub fn page_ranges(&self) -> Vec<PageRange> {
        unsafe {
            let mut n_ranges = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::gtk_print_job_get_page_ranges(self.to_glib_none().0, n_ranges.as_mut_ptr()),
                n_ranges.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gtk_print_job_get_page_set")]
    pub fn page_set(&self) -> PageSet {
        unsafe { from_glib(ffi::gtk_print_job_get_page_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_pages")]
    pub fn pages(&self) -> PrintPages {
        unsafe { from_glib(ffi::gtk_print_job_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_printer")]
    pub fn printer(&self) -> Printer {
        unsafe { from_glib_none(ffi::gtk_print_job_get_printer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_reverse")]
    pub fn is_reverse(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_job_get_reverse(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_rotate")]
    pub fn is_rotate(&self) -> bool {
        unsafe { from_glib(ffi::gtk_print_job_get_rotate(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_scale")]
    pub fn scale(&self) -> f64 {
        unsafe { ffi::gtk_print_job_get_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_job_get_settings")]
    pub fn settings(&self) -> PrintSettings {
        unsafe { from_glib_none(ffi::gtk_print_job_get_settings(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_status")]
    pub fn status(&self) -> PrintStatus {
        unsafe { from_glib(ffi::gtk_print_job_get_status(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_surface")]
    pub fn surface(&self) -> Result<cairo::Surface, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_print_job_get_surface(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_job_get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_print_job_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_job_get_track_print_status")]
    pub fn tracks_print_status(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_print_job_get_track_print_status(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_job_set_collate")]
    pub fn set_collate(&self, collate: bool) {
        unsafe {
            ffi::gtk_print_job_set_collate(self.to_glib_none().0, collate.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_n_up")]
    pub fn set_n_up(&self, n_up: u32) {
        unsafe {
            ffi::gtk_print_job_set_n_up(self.to_glib_none().0, n_up);
        }
    }

    #[doc(alias = "gtk_print_job_set_n_up_layout")]
    pub fn set_n_up_layout(&self, layout: NumberUpLayout) {
        unsafe {
            ffi::gtk_print_job_set_n_up_layout(self.to_glib_none().0, layout.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_num_copies")]
    pub fn set_num_copies(&self, num_copies: i32) {
        unsafe {
            ffi::gtk_print_job_set_num_copies(self.to_glib_none().0, num_copies);
        }
    }

    #[doc(alias = "gtk_print_job_set_page_set")]
    pub fn set_page_set(&self, page_set: PageSet) {
        unsafe {
            ffi::gtk_print_job_set_page_set(self.to_glib_none().0, page_set.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_pages")]
    pub fn set_pages(&self, pages: PrintPages) {
        unsafe {
            ffi::gtk_print_job_set_pages(self.to_glib_none().0, pages.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_reverse")]
    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            ffi::gtk_print_job_set_reverse(self.to_glib_none().0, reverse.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_rotate")]
    pub fn set_rotate(&self, rotate: bool) {
        unsafe {
            ffi::gtk_print_job_set_rotate(self.to_glib_none().0, rotate.to_glib());
        }
    }

    #[doc(alias = "gtk_print_job_set_scale")]
    pub fn set_scale(&self, scale: f64) {
        unsafe {
            ffi::gtk_print_job_set_scale(self.to_glib_none().0, scale);
        }
    }

    #[doc(alias = "gtk_print_job_set_source_fd")]
    pub fn set_source_fd(&self, fd: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_job_set_source_fd(self.to_glib_none().0, fd, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_job_set_source_file")]
    pub fn set_source_file<P: AsRef<std::path::Path>>(
        &self,
        filename: P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_print_job_set_source_file(
                self.to_glib_none().0,
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_print_job_set_track_print_status")]
    pub fn set_track_print_status(&self, track_status: bool) {
        unsafe {
            ffi::gtk_print_job_set_track_print_status(
                self.to_glib_none().0,
                track_status.to_glib(),
            );
        }
    }

    #[doc(alias = "get_property_page_setup")]
    pub fn page_setup(&self) -> Option<PageSetup> {
        unsafe {
            let mut value = glib::Value::from_type(<PageSetup as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"page-setup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `page-setup` getter")
        }
    }

    pub fn connect_status_changed<F: Fn(&PrintJob) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn status_changed_trampoline<F: Fn(&PrintJob) + 'static>(
            this: *mut ffi::GtkPrintJob,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"status-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    status_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_track_print_status_notify<F: Fn(&PrintJob) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_print_status_trampoline<F: Fn(&PrintJob) + 'static>(
            this: *mut ffi::GtkPrintJob,
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
                b"notify::track-print-status\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_print_status_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct PrintJobBuilder {
    page_setup: Option<PageSetup>,
    printer: Option<Printer>,
    settings: Option<PrintSettings>,
    title: Option<String>,
    track_print_status: Option<bool>,
}

impl PrintJobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> PrintJob {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref page_setup) = self.page_setup {
            properties.push(("page-setup", page_setup));
        }
        if let Some(ref printer) = self.printer {
            properties.push(("printer", printer));
        }
        if let Some(ref settings) = self.settings {
            properties.push(("settings", settings));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref track_print_status) = self.track_print_status {
            properties.push(("track-print-status", track_print_status));
        }
        let ret = glib::Object::new::<PrintJob>(&properties).expect("object new");
        ret
    }

    pub fn page_setup(mut self, page_setup: &PageSetup) -> Self {
        self.page_setup = Some(page_setup.clone());
        self
    }

    pub fn printer(mut self, printer: &Printer) -> Self {
        self.printer = Some(printer.clone());
        self
    }

    pub fn settings(mut self, settings: &PrintSettings) -> Self {
        self.settings = Some(settings.clone());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn track_print_status(mut self, track_print_status: bool) -> Self {
        self.track_print_status = Some(track_print_status);
        self
    }
}

impl fmt::Display for PrintJob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintJob")
    }
}
