// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, PermissionState, SecurityOrigin};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PermissionStateQuery(Shared<ffi::WebKitPermissionStateQuery>);

    match fn {
        ref => |ptr| ffi::webkit_permission_state_query_ref(ptr),
        unref => |ptr| ffi::webkit_permission_state_query_unref(ptr),
        type_ => || ffi::webkit_permission_state_query_get_type(),
    }
}

impl PermissionStateQuery {
    #[doc(alias = "webkit_permission_state_query_finish")]
    pub fn finish(&self, state: PermissionState) {
        unsafe {
            ffi::webkit_permission_state_query_finish(self.to_glib_none().0, state.into_glib());
        }
    }

    #[doc(alias = "webkit_permission_state_query_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_permission_state_query_get_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_permission_state_query_get_security_origin")]
    #[doc(alias = "get_security_origin")]
    pub fn security_origin(&self) -> Option<SecurityOrigin> {
        unsafe {
            from_glib_none(ffi::webkit_permission_state_query_get_security_origin(
                self.to_glib_none().0,
            ))
        }
    }
}
