// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Range`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, Border, Orientable, Range, ScrollType};

pub trait RangeImpl: WidgetImpl + ObjectSubclass<Type: IsA<Range> + IsA<Orientable>> {
    fn adjust_bounds(&self, new_value: f64) {
        self.parent_adjust_bounds(new_value)
    }

    fn change_value(&self, scroll_type: ScrollType, new_value: f64) -> glib::Propagation {
        self.parent_change_value(scroll_type, new_value)
    }

    #[doc(alias = "get_range_border")]
    fn range_border(&self) -> Border {
        self.parent_range_border()
    }

    fn move_slider(&self, scroll_type: ScrollType) {
        self.parent_move_slider(scroll_type)
    }

    fn value_changed(&self) {
        self.parent_value_changed()
    }
}

pub trait RangeImplExt: RangeImpl {
    fn parent_adjust_bounds(&self, new_value: f64) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).adjust_bounds {
                f(
                    self.obj().unsafe_cast_ref::<Range>().to_glib_none().0,
                    new_value,
                )
            }
        }
    }

    fn parent_change_value(&self, scroll_type: ScrollType, new_value: f64) -> glib::Propagation {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRangeClass;
            let f = (*parent_class)
                .change_value
                .expect("No parent class impl for \"change_value\"");
            from_glib(f(
                self.obj().unsafe_cast_ref::<Range>().to_glib_none().0,
                scroll_type.into_glib(),
                new_value,
            ))
        }
    }

    fn parent_range_border(&self) -> Border {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRangeClass;
            let mut border = Border::default();
            if let Some(f) = (*parent_class).get_range_border {
                f(
                    self.obj().unsafe_cast_ref::<Range>().to_glib_none().0,
                    border.to_glib_none_mut().0,
                );
            }
            border
        }
    }

    fn parent_move_slider(&self, scroll_type: ScrollType) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).move_slider {
                f(
                    self.obj().unsafe_cast_ref::<Range>().to_glib_none().0,
                    scroll_type.into_glib(),
                )
            }
        }
    }

    fn parent_value_changed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkRangeClass;
            if let Some(f) = (*parent_class).value_changed {
                f(self.obj().unsafe_cast_ref::<Range>().to_glib_none().0)
            }
        }
    }
}

impl<T: RangeImpl> RangeImplExt for T {}

unsafe impl<T: RangeImpl> IsSubclassable<T> for Range {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.adjust_bounds = Some(range_adjust_bounds::<T>);
        klass.change_value = Some(range_change_value::<T>);
        klass.get_range_border = Some(range_get_range_border::<T>);
        klass.move_slider = Some(range_move_slider::<T>);
        klass.value_changed = Some(range_value_changed::<T>);
    }
}

unsafe extern "C" fn range_adjust_bounds<T: RangeImpl>(ptr: *mut ffi::GtkRange, new_value: f64) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.adjust_bounds(new_value)
}

unsafe extern "C" fn range_change_value<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    scroll_type: ffi::GtkScrollType,
    new_value: f64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.change_value(from_glib(scroll_type), new_value)
        .into_glib()
}

unsafe extern "C" fn range_get_range_border<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    borderptr: *mut ffi::GtkBorder,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let border = imp.range_border();
    *borderptr = *border.to_glib_none().0;
}

unsafe extern "C" fn range_move_slider<T: RangeImpl>(
    ptr: *mut ffi::GtkRange,
    scroll_type: ffi::GtkScrollType,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.move_slider(from_glib(scroll_type))
}

unsafe extern "C" fn range_value_changed<T: RangeImpl>(ptr: *mut ffi::GtkRange) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.value_changed()
}
