// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::StringFilterMatchMode;
use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Expression, LayoutManager,
    ListItemFactory, Overflow, Widget,
};
#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
use glib::object::ObjectType as _;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkDropDown")]
    pub struct DropDown(Object<ffi::GtkDropDown, ffi::GtkDropDownClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_drop_down_get_type(),
    }
}

impl DropDown {
    #[doc(alias = "gtk_drop_down_new")]
    pub fn new(
        model: Option<impl IsA<gio::ListModel>>,
        expression: Option<impl AsRef<Expression>>,
    ) -> DropDown {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drop_down_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_drop_down_new_from_strings")]
    #[doc(alias = "new_from_strings")]
    pub fn from_strings(strings: &[&str]) -> DropDown {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drop_down_new_from_strings(
                strings.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DropDown`] objects.
    ///
    /// This method returns an instance of [`DropDownBuilder`](crate::builders::DropDownBuilder) which can be used to create [`DropDown`] objects.
    pub fn builder() -> DropDownBuilder {
        DropDownBuilder::new()
    }

    #[doc(alias = "gtk_drop_down_get_enable_search")]
    #[doc(alias = "get_enable_search")]
    #[doc(alias = "enable-search")]
    pub fn enables_search(&self) -> bool {
        unsafe { from_glib(ffi::gtk_drop_down_get_enable_search(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_down_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_down_get_factory")]
    #[doc(alias = "get_factory")]
    pub fn factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_factory(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_drop_down_get_header_factory")]
    #[doc(alias = "get_header_factory")]
    #[doc(alias = "header-factory")]
    pub fn header_factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_header_factory(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_down_get_list_factory")]
    #[doc(alias = "get_list_factory")]
    #[doc(alias = "list-factory")]
    pub fn list_factory(&self) -> Option<ListItemFactory> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_list_factory(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_down_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_model(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_drop_down_get_search_match_mode")]
    #[doc(alias = "get_search_match_mode")]
    #[doc(alias = "search-match-mode")]
    pub fn search_match_mode(&self) -> StringFilterMatchMode {
        unsafe {
            from_glib(ffi::gtk_drop_down_get_search_match_mode(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drop_down_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn selected(&self) -> u32 {
        unsafe { ffi::gtk_drop_down_get_selected(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_drop_down_get_selected_item")]
    #[doc(alias = "get_selected_item")]
    #[doc(alias = "selected-item")]
    pub fn selected_item(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_drop_down_get_selected_item(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_drop_down_get_show_arrow")]
    #[doc(alias = "get_show_arrow")]
    #[doc(alias = "show-arrow")]
    pub fn shows_arrow(&self) -> bool {
        unsafe { from_glib(ffi::gtk_drop_down_get_show_arrow(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_drop_down_set_enable_search")]
    #[doc(alias = "enable-search")]
    pub fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            ffi::gtk_drop_down_set_enable_search(self.to_glib_none().0, enable_search.into_glib());
        }
    }

    #[doc(alias = "gtk_drop_down_set_expression")]
    #[doc(alias = "expression")]
    pub fn set_expression(&self, expression: Option<impl AsRef<Expression>>) {
        unsafe {
            ffi::gtk_drop_down_set_expression(
                self.to_glib_none().0,
                expression.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_drop_down_set_factory")]
    #[doc(alias = "factory")]
    pub fn set_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_drop_down_set_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_drop_down_set_header_factory")]
    #[doc(alias = "header-factory")]
    pub fn set_header_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_drop_down_set_header_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_drop_down_set_list_factory")]
    #[doc(alias = "list-factory")]
    pub fn set_list_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_drop_down_set_list_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_drop_down_set_model")]
    #[doc(alias = "model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_drop_down_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_drop_down_set_search_match_mode")]
    #[doc(alias = "search-match-mode")]
    pub fn set_search_match_mode(&self, search_match_mode: StringFilterMatchMode) {
        unsafe {
            ffi::gtk_drop_down_set_search_match_mode(
                self.to_glib_none().0,
                search_match_mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_drop_down_set_selected")]
    #[doc(alias = "selected")]
    pub fn set_selected(&self, position: u32) {
        unsafe {
            ffi::gtk_drop_down_set_selected(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_drop_down_set_show_arrow")]
    #[doc(alias = "show-arrow")]
    pub fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_drop_down_set_show_arrow(self.to_glib_none().0, show_arrow.into_glib());
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"activate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "enable-search")]
    pub fn connect_enable_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_search_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::enable-search".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::expression".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "factory")]
    pub fn connect_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::factory".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "header-factory")]
    pub fn connect_header_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_factory_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::header-factory".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_header_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "list-factory")]
    pub fn connect_list_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_list_factory_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::list-factory".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_list_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::model".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "search-match-mode")]
    pub fn connect_search_match_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_match_mode_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::search-match-mode".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_search_match_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::selected".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-item")]
    pub fn connect_selected_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::selected-item".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "show-arrow")]
    pub fn connect_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_arrow_trampoline<F: Fn(&DropDown) + 'static>(
            this: *mut ffi::GtkDropDown,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-arrow".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_arrow_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DropDown {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DropDown`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DropDownBuilder {
    builder: glib::object::ObjectBuilder<'static, DropDown>,
}

impl DropDownBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn enable_search(self, enable_search: bool) -> Self {
        Self {
            builder: self.builder.property("enable-search", enable_search),
        }
    }

    pub fn expression(self, expression: impl AsRef<Expression>) -> Self {
        Self {
            builder: self
                .builder
                .property("expression", expression.as_ref().clone()),
        }
    }

    pub fn factory(self, factory: &impl IsA<ListItemFactory>) -> Self {
        Self {
            builder: self.builder.property("factory", factory.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn header_factory(self, header_factory: &impl IsA<ListItemFactory>) -> Self {
        Self {
            builder: self
                .builder
                .property("header-factory", header_factory.clone().upcast()),
        }
    }

    pub fn list_factory(self, list_factory: &impl IsA<ListItemFactory>) -> Self {
        Self {
            builder: self
                .builder
                .property("list-factory", list_factory.clone().upcast()),
        }
    }

    pub fn model(self, model: &impl IsA<gio::ListModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn search_match_mode(self, search_match_mode: StringFilterMatchMode) -> Self {
        Self {
            builder: self
                .builder
                .property("search-match-mode", search_match_mode),
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Self {
            builder: self.builder.property("selected", selected),
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    pub fn show_arrow(self, show_arrow: bool) -> Self {
        Self {
            builder: self.builder.property("show-arrow", show_arrow),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DropDown`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DropDown {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
