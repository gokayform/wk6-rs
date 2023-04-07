// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from webkit-gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDeviceInfoPermissionRequest")]
    pub struct DeviceInfoPermissionRequest(Object<ffi::WebKitDeviceInfoPermissionRequest, ffi::WebKitDeviceInfoPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_device_info_permission_request_get_type(),
    }
}

impl DeviceInfoPermissionRequest {}

impl fmt::Display for DeviceInfoPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceInfoPermissionRequest")
    }
}
