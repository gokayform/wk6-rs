// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::Frame;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitWebFormManager")]
    pub struct WebFormManager(Object<ffi::WebKitWebFormManager, ffi::WebKitWebFormManagerClass>);

    match fn {
        type_ => || ffi::webkit_web_form_manager_get_type(),
    }
}

impl WebFormManager {
    pub const NONE: Option<&'static WebFormManager> = None;

    #[doc(alias = "webkit_web_form_manager_input_element_auto_fill")]
    pub fn input_element_auto_fill(element: &javascriptcore::Value, value: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::webkit_web_form_manager_input_element_auto_fill(
                element.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_web_form_manager_input_element_is_auto_filled")]
    pub fn input_element_is_auto_filled(element: &javascriptcore::Value) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::webkit_web_form_manager_input_element_is_auto_filled(
                element.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_web_form_manager_input_element_is_user_edited")]
    pub fn input_element_is_user_edited(element: &javascriptcore::Value) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::webkit_web_form_manager_input_element_is_user_edited(
                element.to_glib_none().0,
            ))
        }
    }
}

pub trait WebFormManagerExt: 'static {
    //#[doc(alias = "form-controls-associated")]
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "will-send-submit-event")]
    fn connect_will_send_submit_event<
        F: Fn(&Self, &javascriptcore::Value, &Frame, &Frame) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "will-submit-form")]
    fn connect_will_submit_form<F: Fn(&Self, &javascriptcore::Value, &Frame, &Frame) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<WebFormManager>> WebFormManagerExt for O {
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype elements: *.PtrArray TypeId { ns_id: 16, id: 2 }
    //}

    fn connect_will_send_submit_event<
        F: Fn(&Self, &javascriptcore::Value, &Frame, &Frame) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn will_send_submit_event_trampoline<
            P: IsA<WebFormManager>,
            F: Fn(&P, &javascriptcore::Value, &Frame, &Frame) + 'static,
        >(
            this: *mut ffi::WebKitWebFormManager,
            form: *mut javascriptcore::ffi::JSCValue,
            source_frame: *mut ffi::WebKitFrame,
            target_frame: *mut ffi::WebKitFrame,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                WebFormManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(form),
                &from_glib_borrow(source_frame),
                &from_glib_borrow(target_frame),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"will-send-submit-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    will_send_submit_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_will_submit_form<F: Fn(&Self, &javascriptcore::Value, &Frame, &Frame) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn will_submit_form_trampoline<
            P: IsA<WebFormManager>,
            F: Fn(&P, &javascriptcore::Value, &Frame, &Frame) + 'static,
        >(
            this: *mut ffi::WebKitWebFormManager,
            form: *mut javascriptcore::ffi::JSCValue,
            source_frame: *mut ffi::WebKitFrame,
            target_frame: *mut ffi::WebKitFrame,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                WebFormManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(form),
                &from_glib_borrow(source_frame),
                &from_glib_borrow(target_frame),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"will-submit-form\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    will_submit_form_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WebFormManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebFormManager")
    }
}
