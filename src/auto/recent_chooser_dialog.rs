// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use RecentChooser;
use Widget;
use Window;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct RecentChooserDialog(Object<ffi::GtkRecentChooserDialog>): Dialog, Window, Bin, Container, Widget, RecentChooser;

    match fn {
        get_type => || ffi::gtk_recent_chooser_dialog_get_type(),
    }
}

impl RecentChooserDialog {
    //pub fn new<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> RecentChooserDialog {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_dialog_new() }
    //}

    //pub fn new_for_manager<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, manager: &RecentManager, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> RecentChooserDialog {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_dialog_new_for_manager() }
    //}
}
