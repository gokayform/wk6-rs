// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitEditorState")]
    pub struct EditorState(Object<ffi::WebKitEditorState, ffi::WebKitEditorStateClass>);

    match fn {
        type_ => || ffi::webkit_editor_state_get_type(),
    }
}

impl EditorState {
    #[doc(alias = "webkit_editor_state_get_typing_attributes")]
    #[doc(alias = "get_typing_attributes")]
    pub fn typing_attributes(&self) -> u32 {
        unsafe { ffi::webkit_editor_state_get_typing_attributes(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_editor_state_is_copy_available")]
    pub fn is_copy_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_copy_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_editor_state_is_cut_available")]
    pub fn is_cut_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_cut_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_editor_state_is_paste_available")]
    pub fn is_paste_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_paste_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_editor_state_is_redo_available")]
    pub fn is_redo_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_redo_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_editor_state_is_undo_available")]
    pub fn is_undo_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_undo_available(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "typing-attributes")]
    pub fn connect_typing_attributes_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_typing_attributes_trampoline<F: Fn(&EditorState) + 'static>(
            this: *mut ffi::WebKitEditorState,
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
                b"notify::typing-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_typing_attributes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EditorState")
    }
}
