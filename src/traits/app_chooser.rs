// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use glib::translate::{FromGlibPtr};
use ffi;
use cast::GTK_APP_CHOOSER;

pub trait AppChooserTrait: ::WidgetTrait {
    fn get_app_info(&self) -> Option<::AppInfo> {
        let tmp_pointer = unsafe { ffi::gtk_app_chooser_get_app_info(GTK_APP_CHOOSER(self.unwrap_widget())) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    fn get_content_info(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_app_chooser_get_content_type(GTK_APP_CHOOSER(self.unwrap_widget())))
        }
    }

    fn refresh(&self) -> () {
        unsafe { ffi::gtk_app_chooser_refresh(GTK_APP_CHOOSER(self.unwrap_widget())) }
    }
}
