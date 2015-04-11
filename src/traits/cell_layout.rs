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

use glib::translate::ToGlibPtr;
use ffi;
use cast::{GTK_CELL_LAYOUT, GTK_CELL_RENDERER};
use glib;

pub trait CellLayoutTrait: ::WidgetTrait {
    fn pack_start<T: ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    fn pack_end<T: ::CellRendererTrait>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), match expand {
                true => 1,
                false => 0
            })
        }
    }

    /*fn get_area(&self) -> Option<::CellArea> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_area(GTK_CELL_LAYOUT(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }*/

    fn get_cells<T: ::CellRendererTrait>(&self) -> glib::List<T> {
        let tmp = unsafe { ffi::gtk_cell_layout_get_cells(GTK_CELL_LAYOUT(self.unwrap_widget())) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let list: glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);

            list.iter().map(|it| ::FFIWidget::wrap_widget(*it)).collect()
        }
    }

    fn reorder<T: ::CellRendererTrait>(&self, cell: &T, position: i32) {
        unsafe { ffi::gtk_cell_layout_reorder(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget()), position) }
    }

    fn clear(&self) {
        unsafe { ffi::gtk_cell_layout_clear(GTK_CELL_LAYOUT(self.unwrap_widget())) }
    }

    fn add_attribute<T: ::CellRendererTrait>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(
                GTK_CELL_LAYOUT(self.unwrap_widget()),
                GTK_CELL_RENDERER(cell.unwrap_widget()),
                attribute.borrow_to_glib().0,
                column)
            }
    }

    fn clear_attributes<T: ::CellRendererTrait>(&self, cell: &T) {
        unsafe { ffi::gtk_cell_layout_clear_attributes(GTK_CELL_LAYOUT(self.unwrap_widget()), GTK_CELL_RENDERER(cell.unwrap_widget())) }
    }
}
