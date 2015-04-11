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

//! The widget used for item in menus

use ffi;
use glib::translate::ToGlibPtr;

/// CheckMenuItem — The widget used for item in menus
struct_Widget!(CheckMenuItem);

impl CheckMenuItem {
    pub fn new() -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_check_menu_item_new() };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_label(label: &str) -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_menu_item_new_with_label(label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }

    pub fn new_with_mnemonic(mnemonic: &str) -> Option<CheckMenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_menu_item_new_with_mnemonic(
                    mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, CheckMenuItem)
    }
}

impl_drop!(CheckMenuItem);
impl_TraitWidget!(CheckMenuItem);

impl ::ContainerTrait for CheckMenuItem {}
impl ::BinTrait for CheckMenuItem {}
impl ::MenuItemTrait for CheckMenuItem {}
impl ::CheckMenuItemTrait for CheckMenuItem {}

impl_widget_events!(CheckMenuItem);
