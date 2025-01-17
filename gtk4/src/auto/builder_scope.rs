// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use std::fmt;

glib::wrapper! {
    pub struct BuilderScope(Interface<ffi::GtkBuilderScope, ffi::GtkBuilderScopeInterface>);

    match fn {
        get_type => || ffi::gtk_builder_scope_get_type(),
    }
}

pub const NONE_BUILDER_SCOPE: Option<&BuilderScope> = None;

pub trait BuilderScopeExt: 'static {}

impl<O: IsA<BuilderScope>> BuilderScopeExt for O {}

impl fmt::Display for BuilderScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BuilderScope")
    }
}
