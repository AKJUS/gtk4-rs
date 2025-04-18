// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`StyleContext`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, StyleContext};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait StyleContextImpl: ObjectImpl + ObjectSubclass<Type: IsA<StyleContext>> {
    fn changed(&self) {
        self.parent_changed()
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait StyleContextImplExt: StyleContextImpl {
    fn parent_changed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkStyleContextClass;
            if let Some(f) = (*parent_class).changed {
                f(self
                    .obj()
                    .unsafe_cast_ref::<StyleContext>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

impl<T: StyleContextImpl> StyleContextImplExt for T {}

unsafe impl<T: StyleContextImpl> IsSubclassable<T> for StyleContext {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();
        let klass = class.as_mut();
        klass.changed = Some(style_context_changed::<T>);
    }
}

unsafe extern "C" fn style_context_changed<T: StyleContextImpl>(ptr: *mut ffi::GtkStyleContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}
