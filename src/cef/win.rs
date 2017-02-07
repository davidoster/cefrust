/* automatically generated by rust-bindgen */

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use cef::cef_string_t;

pub type DWORD = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HWND__() {
    assert_eq!(::std::mem::size_of::<HWND__>() , 4usize);
    assert_eq!(::std::mem::align_of::<HWND__>() , 4usize);
}
impl Clone for HWND__ {
    fn clone(&self) -> Self { *self }
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct HMENU__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HMENU__() {
    assert_eq!(::std::mem::size_of::<HMENU__>() , 4usize);
    assert_eq!(::std::mem::align_of::<HMENU__>() , 4usize);
}
impl Clone for HMENU__ {
    fn clone(&self) -> Self { *self }
}
pub type HMENU = *mut HMENU__;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct HINSTANCE__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HINSTANCE__() {
    assert_eq!(::std::mem::size_of::<HINSTANCE__>() , 4usize);
    assert_eq!(::std::mem::align_of::<HINSTANCE__>() , 4usize);
}
impl Clone for HINSTANCE__ {
    fn clone(&self) -> Self { *self }
}
pub type HINSTANCE = *mut HINSTANCE__;
///
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _cef_main_args_t {
    pub instance: HINSTANCE,
}
#[test]
fn bindgen_test_layout__cef_main_args_t() {
    assert_eq!(::std::mem::size_of::<_cef_main_args_t>() , 8usize);
    assert_eq!(::std::mem::align_of::<_cef_main_args_t>() , 8usize);
}
impl Clone for _cef_main_args_t {
    fn clone(&self) -> Self { *self }
}
///
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _cef_window_info_t {
    pub ex_style: DWORD,
    pub window_name: cef_string_t,
    pub style: DWORD,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub parent_window: HWND,
    pub menu: HMENU,
    ///
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    ///
    pub transparent_painting_enabled: ::std::os::raw::c_int,
    ///
    pub window: HWND,
}
#[test]
fn bindgen_test_layout__cef_window_info_t() {
    assert_eq!(::std::mem::size_of::<_cef_window_info_t>() , 88usize);
    assert_eq!(::std::mem::align_of::<_cef_window_info_t>() , 8usize);
}
impl Clone for _cef_window_info_t {
    fn clone(&self) -> Self { *self }
}
