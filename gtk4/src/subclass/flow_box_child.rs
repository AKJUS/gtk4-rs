// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`FlowBoxChild`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, FlowBoxChild};

pub trait FlowBoxChildImpl: WidgetImpl + ObjectSubclass<Type: IsA<FlowBoxChild>> {
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait FlowBoxChildImplExt: FlowBoxChildImpl {
    fn parent_activate(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkFlowBoxChildClass;
            if let Some(f) = (*parent_class).activate {
                f(self
                    .obj()
                    .unsafe_cast_ref::<FlowBoxChild>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

impl<T: FlowBoxChildImpl> FlowBoxChildImplExt for T {}

unsafe impl<T: FlowBoxChildImpl> IsSubclassable<T> for FlowBoxChild {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(child_activate::<T>);
    }
}

unsafe extern "C" fn child_activate<T: FlowBoxChildImpl>(ptr: *mut ffi::GtkFlowBoxChild) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}
