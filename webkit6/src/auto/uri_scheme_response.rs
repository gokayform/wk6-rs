// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "WebKitURISchemeResponse")]
    pub struct URISchemeResponse(Object<ffi::WebKitURISchemeResponse, ffi::WebKitURISchemeResponseClass>);

    match fn {
        type_ => || ffi::webkit_uri_scheme_response_get_type(),
    }
}

impl URISchemeResponse {
    #[doc(alias = "webkit_uri_scheme_response_new")]
    pub fn new(input_stream: &impl IsA<gio::InputStream>, stream_length: i64) -> URISchemeResponse {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_uri_scheme_response_new(
                input_stream.as_ref().to_glib_none().0,
                stream_length,
            ))
        }
    }

    #[doc(alias = "webkit_uri_scheme_response_set_content_type")]
    pub fn set_content_type(&self, content_type: &str) {
        unsafe {
            ffi::webkit_uri_scheme_response_set_content_type(
                self.to_glib_none().0,
                content_type.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_uri_scheme_response_set_http_headers")]
    pub fn set_http_headers(&self, headers: soup::MessageHeaders) {
        unsafe {
            ffi::webkit_uri_scheme_response_set_http_headers(
                self.to_glib_none().0,
                headers.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "webkit_uri_scheme_response_set_status")]
    pub fn set_status(&self, status_code: u32, reason_phrase: Option<&str>) {
        unsafe {
            ffi::webkit_uri_scheme_response_set_status(
                self.to_glib_none().0,
                status_code,
                reason_phrase.to_glib_none().0,
            );
        }
    }
}
