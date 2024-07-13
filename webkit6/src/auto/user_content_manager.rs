// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::{ffi, ScriptMessageReply, UserContentFilter, UserScript, UserStyleSheet};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WebKitUserContentManager")]
    pub struct UserContentManager(Object<ffi::WebKitUserContentManager, ffi::WebKitUserContentManagerClass>);

    match fn {
        type_ => || ffi::webkit_user_content_manager_get_type(),
    }
}

impl UserContentManager {
    #[doc(alias = "webkit_user_content_manager_new")]
    pub fn new() -> UserContentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_user_content_manager_new()) }
    }

    #[doc(alias = "webkit_user_content_manager_add_filter")]
    pub fn add_filter(&self, filter: &UserContentFilter) {
        unsafe {
            ffi::webkit_user_content_manager_add_filter(
                self.to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_add_script")]
    pub fn add_script(&self, script: &UserScript) {
        unsafe {
            ffi::webkit_user_content_manager_add_script(
                self.to_glib_none().0,
                script.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_add_style_sheet")]
    pub fn add_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            ffi::webkit_user_content_manager_add_style_sheet(
                self.to_glib_none().0,
                stylesheet.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_register_script_message_handler")]
    pub fn register_script_message_handler(&self, name: &str, world_name: Option<&str>) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_user_content_manager_register_script_message_handler(
                    self.to_glib_none().0,
                    name.to_glib_none().0,
                    world_name.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "webkit_user_content_manager_register_script_message_handler_with_reply")]
    pub fn register_script_message_handler_with_reply(&self, name: &str, world_name: &str) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_user_content_manager_register_script_message_handler_with_reply(
                    self.to_glib_none().0,
                    name.to_glib_none().0,
                    world_name.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_all_filters")]
    pub fn remove_all_filters(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_filters(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_all_scripts")]
    pub fn remove_all_scripts(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_scripts(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_all_style_sheets")]
    pub fn remove_all_style_sheets(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_style_sheets(self.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_filter")]
    pub fn remove_filter(&self, filter: &UserContentFilter) {
        unsafe {
            ffi::webkit_user_content_manager_remove_filter(
                self.to_glib_none().0,
                filter.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_filter_by_id")]
    pub fn remove_filter_by_id(&self, filter_id: &str) {
        unsafe {
            ffi::webkit_user_content_manager_remove_filter_by_id(
                self.to_glib_none().0,
                filter_id.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_script")]
    pub fn remove_script(&self, script: &UserScript) {
        unsafe {
            ffi::webkit_user_content_manager_remove_script(
                self.to_glib_none().0,
                script.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_remove_style_sheet")]
    pub fn remove_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            ffi::webkit_user_content_manager_remove_style_sheet(
                self.to_glib_none().0,
                stylesheet.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_user_content_manager_unregister_script_message_handler")]
    pub fn unregister_script_message_handler(&self, name: &str, world_name: Option<&str>) {
        unsafe {
            ffi::webkit_user_content_manager_unregister_script_message_handler(
                self.to_glib_none().0,
                name.to_glib_none().0,
                world_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "script-message-received")]
    pub fn connect_script_message_received<F: Fn(&Self, &javascriptcore::Value) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn script_message_received_trampoline<
            F: Fn(&UserContentManager, &javascriptcore::Value) + 'static,
        >(
            this: *mut ffi::WebKitUserContentManager,
            value: *mut javascriptcore::ffi::JSCValue,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(value))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("script-message-received::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"script-message-received\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    script_message_received_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "script-message-with-reply-received")]
    pub fn connect_script_message_with_reply_received<
        F: Fn(&Self, &javascriptcore::Value, &ScriptMessageReply) -> bool + 'static,
    >(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn script_message_with_reply_received_trampoline<
            F: Fn(&UserContentManager, &javascriptcore::Value, &ScriptMessageReply) -> bool + 'static,
        >(
            this: *mut ffi::WebKitUserContentManager,
            value: *mut javascriptcore::ffi::JSCValue,
            reply: *mut ffi::WebKitScriptMessageReply,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(value),
                &from_glib_borrow(reply),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("script-message-with-reply-received::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"script-message-with-reply-received\0"[..], |n| {
                    n.as_bytes()
                });
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    script_message_with_reply_received_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for UserContentManager {
    fn default() -> Self {
        Self::new()
    }
}
