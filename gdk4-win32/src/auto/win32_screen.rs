// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
#[cfg(feature = "v4_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkWin32Screen")]
    pub struct Win32Screen(Object<ffi::GdkWin32Screen, ffi::GdkWin32ScreenClass>);

    match fn {
        type_ => || ffi::gdk_win32_screen_get_type(),
    }
}

impl Win32Screen {
    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn display(&self) -> Option<gdk::Display> {
        ObjectExt::property(self, "display")
    }
}
