// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GMenuAttributeIter")]
    pub struct MenuAttributeIter(Object<ffi::GMenuAttributeIter, ffi::GMenuAttributeIterClass>);

    match fn {
        type_ => || ffi::g_menu_attribute_iter_get_type(),
    }
}

pub const NONE_MENU_ATTRIBUTE_ITER: Option<&MenuAttributeIter> = None;

pub trait MenuAttributeIterExt: 'static {
    #[doc(alias = "g_menu_attribute_iter_get_next")]
    #[doc(alias = "get_next")]
    fn next(&self) -> Option<(glib::GString, glib::Variant)>;
}

impl<O: IsA<MenuAttributeIter>> MenuAttributeIterExt for O {
    fn next(&self) -> Option<(glib::GString, glib::Variant)> {
        unsafe {
            let mut out_name = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_attribute_iter_get_next(
                self.as_ref().to_glib_none().0,
                &mut out_name,
                &mut value,
            ));
            if ret {
                Some((from_glib_none(out_name), from_glib_full(value)))
            } else {
                None
            }
        }
    }
}

impl fmt::Display for MenuAttributeIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuAttributeIter")
    }
}
