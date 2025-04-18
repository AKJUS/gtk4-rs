mod imp;

use gtk::{glib, prelude::*};

use crate::custom_tag::CustomTag;

glib::wrapper! {
    pub struct CustomEditable(ObjectSubclass<imp::CustomEditable>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Editable;
}

impl Default for CustomEditable {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl CustomEditable {
    pub fn add_tag(&self, tag: &CustomTag) {
        tag.set_parent(self);
    }

    pub fn remove_tag(&self, tag: &CustomTag) {
        tag.unparent();
    }
}
