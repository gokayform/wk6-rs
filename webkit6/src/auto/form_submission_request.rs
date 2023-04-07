// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::translate::*;
use std::{fmt, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitFormSubmissionRequest")]
    pub struct FormSubmissionRequest(Object<ffi::WebKitFormSubmissionRequest, ffi::WebKitFormSubmissionRequestClass>);

    match fn {
        type_ => || ffi::webkit_form_submission_request_get_type(),
    }
}

impl FormSubmissionRequest {
    #[doc(alias = "webkit_form_submission_request_list_text_fields")]
    pub fn list_text_fields(&self) -> Option<(Vec<glib::GString>, Vec<glib::GString>)> {
        unsafe {
            let mut field_names = ptr::null_mut();
            let mut field_values = ptr::null_mut();
            let ret = from_glib(ffi::webkit_form_submission_request_list_text_fields(
                self.to_glib_none().0,
                &mut field_names,
                &mut field_values,
            ));
            if ret {
                Some((
                    FromGlibPtrContainer::from_glib_none(field_names),
                    FromGlibPtrContainer::from_glib_none(field_values),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "webkit_form_submission_request_submit")]
    pub fn submit(&self) {
        unsafe {
            ffi::webkit_form_submission_request_submit(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for FormSubmissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FormSubmissionRequest")
    }
}