// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, InputHints, InputPurpose};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitInputMethodContext")]
    pub struct InputMethodContext(Object<ffi::WebKitInputMethodContext, ffi::WebKitInputMethodContextClass>);

    match fn {
        type_ => || ffi::webkit_input_method_context_get_type(),
    }
}

impl InputMethodContext {
    pub const NONE: Option<&'static InputMethodContext> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::InputMethodContext>> Sealed for T {}
}

pub trait InputMethodContextExt: IsA<InputMethodContext> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_input_method_context_filter_key_event")]
    fn filter_key_event(&self, key_event: impl AsRef<gdk::Event>) -> bool {
        unsafe {
            from_glib(ffi::webkit_input_method_context_filter_key_event(
                self.as_ref().to_glib_none().0,
                key_event.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_input_method_context_get_input_hints")]
    #[doc(alias = "get_input_hints")]
    #[doc(alias = "input-hints")]
    fn input_hints(&self) -> InputHints {
        unsafe {
            from_glib(ffi::webkit_input_method_context_get_input_hints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_input_method_context_get_input_purpose")]
    #[doc(alias = "get_input_purpose")]
    #[doc(alias = "input-purpose")]
    fn input_purpose(&self) -> InputPurpose {
        unsafe {
            from_glib(ffi::webkit_input_method_context_get_input_purpose(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "webkit_input_method_context_get_preedit")]
    //#[doc(alias = "get_preedit")]
    //fn preedit(&self, underlines: /*Unimplemented*/Vec<InputMethodUnderline>) -> (Option<glib::GString>, u32) {
    //    unsafe { TODO: call ffi:webkit_input_method_context_get_preedit() }
    //}

    #[doc(alias = "webkit_input_method_context_notify_cursor_area")]
    fn notify_cursor_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::webkit_input_method_context_notify_cursor_area(
                self.as_ref().to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    #[doc(alias = "webkit_input_method_context_notify_focus_in")]
    fn notify_focus_in(&self) {
        unsafe {
            ffi::webkit_input_method_context_notify_focus_in(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_input_method_context_notify_focus_out")]
    fn notify_focus_out(&self) {
        unsafe {
            ffi::webkit_input_method_context_notify_focus_out(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_input_method_context_notify_surrounding")]
    fn notify_surrounding(&self, text: &str, cursor_index: u32, selection_index: u32) {
        let length = text.len() as _;
        unsafe {
            ffi::webkit_input_method_context_notify_surrounding(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                length,
                cursor_index,
                selection_index,
            );
        }
    }

    #[doc(alias = "webkit_input_method_context_reset")]
    fn reset(&self) {
        unsafe {
            ffi::webkit_input_method_context_reset(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_input_method_context_set_enable_preedit")]
    fn set_enable_preedit(&self, enabled: bool) {
        unsafe {
            ffi::webkit_input_method_context_set_enable_preedit(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "webkit_input_method_context_set_input_hints")]
    #[doc(alias = "input-hints")]
    fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::webkit_input_method_context_set_input_hints(
                self.as_ref().to_glib_none().0,
                hints.into_glib(),
            );
        }
    }

    #[doc(alias = "webkit_input_method_context_set_input_purpose")]
    #[doc(alias = "input-purpose")]
    fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::webkit_input_method_context_set_input_purpose(
                self.as_ref().to_glib_none().0,
                purpose.into_glib(),
            );
        }
    }

    #[doc(alias = "committed")]
    fn connect_committed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn committed_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            text: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                InputMethodContext::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"committed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    committed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "delete-surrounding")]
    fn connect_delete_surrounding<F: Fn(&Self, i32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_surrounding_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P, i32, u32) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            offset: libc::c_int,
            n_chars: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                InputMethodContext::from_glib_borrow(this).unsafe_cast_ref(),
                offset,
                n_chars,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-surrounding\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    delete_surrounding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preedit-changed")]
    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_changed_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    preedit_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preedit-finished")]
    fn connect_preedit_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_finished_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-finished\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    preedit_finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preedit-started")]
    fn connect_preedit_started<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_started_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-started\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    preedit_started_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-hints")]
    fn connect_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_hints_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-hints\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_hints_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-purpose")]
    fn connect_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_purpose_trampoline<
            P: IsA<InputMethodContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitInputMethodContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InputMethodContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-purpose\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_purpose_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<InputMethodContext>> InputMethodContextExt for O {}
