// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryPressureSettings(Boxed<ffi::WebKitMemoryPressureSettings>);

    match fn {
        copy => |ptr| ffi::webkit_memory_pressure_settings_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_memory_pressure_settings_free(ptr),
        type_ => || ffi::webkit_memory_pressure_settings_get_type(),
    }
}

impl MemoryPressureSettings {
    #[doc(alias = "webkit_memory_pressure_settings_new")]
    pub fn new() -> MemoryPressureSettings {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_memory_pressure_settings_new()) }
    }

    #[doc(alias = "webkit_memory_pressure_settings_get_conservative_threshold")]
    #[doc(alias = "get_conservative_threshold")]
    pub fn conservative_threshold(&mut self) -> f64 {
        unsafe {
            ffi::webkit_memory_pressure_settings_get_conservative_threshold(
                self.to_glib_none_mut().0,
            )
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_get_kill_threshold")]
    #[doc(alias = "get_kill_threshold")]
    pub fn kill_threshold(&mut self) -> f64 {
        unsafe {
            ffi::webkit_memory_pressure_settings_get_kill_threshold(self.to_glib_none_mut().0)
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_get_memory_limit")]
    #[doc(alias = "get_memory_limit")]
    pub fn memory_limit(&mut self) -> u32 {
        unsafe { ffi::webkit_memory_pressure_settings_get_memory_limit(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "webkit_memory_pressure_settings_get_poll_interval")]
    #[doc(alias = "get_poll_interval")]
    pub fn poll_interval(&mut self) -> f64 {
        unsafe { ffi::webkit_memory_pressure_settings_get_poll_interval(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "webkit_memory_pressure_settings_get_strict_threshold")]
    #[doc(alias = "get_strict_threshold")]
    pub fn strict_threshold(&mut self) -> f64 {
        unsafe {
            ffi::webkit_memory_pressure_settings_get_strict_threshold(self.to_glib_none_mut().0)
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_set_conservative_threshold")]
    pub fn set_conservative_threshold(&mut self, value: f64) {
        unsafe {
            ffi::webkit_memory_pressure_settings_set_conservative_threshold(
                self.to_glib_none_mut().0,
                value,
            );
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_set_kill_threshold")]
    pub fn set_kill_threshold(&mut self, value: f64) {
        unsafe {
            ffi::webkit_memory_pressure_settings_set_kill_threshold(
                self.to_glib_none_mut().0,
                value,
            );
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_set_memory_limit")]
    pub fn set_memory_limit(&mut self, memory_limit: u32) {
        unsafe {
            ffi::webkit_memory_pressure_settings_set_memory_limit(
                self.to_glib_none_mut().0,
                memory_limit,
            );
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_set_poll_interval")]
    pub fn set_poll_interval(&mut self, value: f64) {
        unsafe {
            ffi::webkit_memory_pressure_settings_set_poll_interval(
                self.to_glib_none_mut().0,
                value,
            );
        }
    }

    #[doc(alias = "webkit_memory_pressure_settings_set_strict_threshold")]
    pub fn set_strict_threshold(&mut self, value: f64) {
        unsafe {
            ffi::webkit_memory_pressure_settings_set_strict_threshold(
                self.to_glib_none_mut().0,
                value,
            );
        }
    }
}

impl Default for MemoryPressureSettings {
    fn default() -> Self {
        Self::new()
    }
}