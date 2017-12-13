// This file was generated by gir (13f739b) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use FontChooser;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontChooserDialog(Object<ffi::GtkFontChooserDialog, ffi::GtkFontChooserDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_chooser_dialog_get_type(),
    }
}

impl FontChooserDialog {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>>(title: P, parent: R) -> FontChooserDialog {
        assert_initialized_main_thread!();
        let title = title.into();
        let title = title.to_glib_none();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_dialog_new(title.0, parent.0)).downcast_unchecked()
        }
    }
}
