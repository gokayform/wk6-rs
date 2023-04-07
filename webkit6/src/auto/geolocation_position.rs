// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GeolocationPosition(Boxed<ffi::WebKitGeolocationPosition>);

    match fn {
        copy => |ptr| ffi::webkit_geolocation_position_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_geolocation_position_free(ptr),
        type_ => || ffi::webkit_geolocation_position_get_type(),
    }
}

impl GeolocationPosition {
    #[doc(alias = "webkit_geolocation_position_new")]
    pub fn new(latitude: f64, longitude: f64, accuracy: f64) -> GeolocationPosition {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_geolocation_position_new(
                latitude, longitude, accuracy,
            ))
        }
    }

    #[doc(alias = "webkit_geolocation_position_set_altitude")]
    pub fn set_altitude(&mut self, altitude: f64) {
        unsafe {
            ffi::webkit_geolocation_position_set_altitude(self.to_glib_none_mut().0, altitude);
        }
    }

    #[doc(alias = "webkit_geolocation_position_set_altitude_accuracy")]
    pub fn set_altitude_accuracy(&mut self, altitude_accuracy: f64) {
        unsafe {
            ffi::webkit_geolocation_position_set_altitude_accuracy(
                self.to_glib_none_mut().0,
                altitude_accuracy,
            );
        }
    }

    #[doc(alias = "webkit_geolocation_position_set_heading")]
    pub fn set_heading(&mut self, heading: f64) {
        unsafe {
            ffi::webkit_geolocation_position_set_heading(self.to_glib_none_mut().0, heading);
        }
    }

    #[doc(alias = "webkit_geolocation_position_set_speed")]
    pub fn set_speed(&mut self, speed: f64) {
        unsafe {
            ffi::webkit_geolocation_position_set_speed(self.to_glib_none_mut().0, speed);
        }
    }

    #[doc(alias = "webkit_geolocation_position_set_timestamp")]
    pub fn set_timestamp(&mut self, timestamp: u64) {
        unsafe {
            ffi::webkit_geolocation_position_set_timestamp(self.to_glib_none_mut().0, timestamp);
        }
    }
}
