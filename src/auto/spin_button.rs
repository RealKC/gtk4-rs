// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Adjustment;
use Align;
use Buildable;
use Editable;
use LayoutManager;
use Orientable;
use Overflow;
use ScrollType;
use SpinButtonUpdatePolicy;
use SpinType;
use Widget;

glib_wrapper! {
    pub struct SpinButton(Object<gtk_sys::GtkSpinButton, gtk_sys::GtkSpinButtonClass, SpinButtonClass>) @extends Widget, @implements Buildable, Editable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    pub fn new<P: IsA<Adjustment>>(
        adjustment: Option<&P>,
        climb_rate: f64,
        digits: u32,
    ) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_spin_button_new(
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            ))
            .unsafe_cast()
        }
    }

    pub fn new_with_range(min: f64, max: f64, step: f64) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_spin_button_new_with_range(min, max, step))
                .unsafe_cast()
        }
    }
}

pub struct SpinButtonBuilder {
    adjustment: Option<Adjustment>,
    climb_rate: Option<f64>,
    digits: Option<u32>,
    numeric: Option<bool>,
    snap_to_ticks: Option<bool>,
    update_policy: Option<SpinButtonUpdatePolicy>,
    value: Option<f64>,
    wrap: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl SpinButtonBuilder {
    pub fn new() -> Self {
        Self {
            adjustment: None,
            climb_rate: None,
            digits: None,
            numeric: None,
            snap_to_ticks: None,
            update_policy: None,
            value: None,
            wrap: None,
            can_focus: None,
            can_target: None,
            css_name: None,
            cursor: None,
            expand: None,
            focus_on_click: None,
            halign: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            layout_manager: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            opacity: None,
            overflow: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> SpinButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref climb_rate) = self.climb_rate {
            properties.push(("climb-rate", climb_rate));
        }
        if let Some(ref digits) = self.digits {
            properties.push(("digits", digits));
        }
        if let Some(ref numeric) = self.numeric {
            properties.push(("numeric", numeric));
        }
        if let Some(ref snap_to_ticks) = self.snap_to_ticks {
            properties.push(("snap-to-ticks", snap_to_ticks));
        }
        if let Some(ref update_policy) = self.update_policy {
            properties.push(("update-policy", update_policy));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref wrap) = self.wrap {
            properties.push(("wrap", wrap));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(SpinButton::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn adjustment(mut self, adjustment: &Adjustment) -> Self {
        self.adjustment = Some(adjustment.clone());
        self
    }

    pub fn climb_rate(mut self, climb_rate: f64) -> Self {
        self.climb_rate = Some(climb_rate);
        self
    }

    pub fn digits(mut self, digits: u32) -> Self {
        self.digits = Some(digits);
        self
    }

    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = Some(numeric);
        self
    }

    pub fn snap_to_ticks(mut self, snap_to_ticks: bool) -> Self {
        self.snap_to_ticks = Some(snap_to_ticks);
        self
    }

    pub fn update_policy(mut self, update_policy: SpinButtonUpdatePolicy) -> Self {
        self.update_policy = Some(update_policy);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &LayoutManager) -> Self {
        self.layout_manager = Some(layout_manager.clone());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_SPIN_BUTTON: Option<&SpinButton> = None;

pub trait SpinButtonExt: 'static {
    fn configure<P: IsA<Adjustment>>(&self, adjustment: Option<&P>, climb_rate: f64, digits: u32);

    fn get_adjustment(&self) -> Adjustment;

    fn get_digits(&self) -> u32;

    fn get_increments(&self) -> (f64, f64);

    fn get_numeric(&self) -> bool;

    fn get_range(&self) -> (f64, f64);

    fn get_snap_to_ticks(&self) -> bool;

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy;

    fn get_value(&self) -> f64;

    fn get_value_as_int(&self) -> i32;

    fn get_wrap(&self) -> bool;

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    fn set_digits(&self, digits: u32);

    fn set_increments(&self, step: f64, page: f64);

    fn set_numeric(&self, numeric: bool);

    fn set_range(&self, min: f64, max: f64);

    fn set_snap_to_ticks(&self, snap_to_ticks: bool);

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy);

    fn set_value(&self, value: f64);

    fn set_wrap(&self, wrap: bool);

    fn spin(&self, direction: SpinType, increment: f64);

    fn update(&self);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn connect_change_value<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_change_value(&self, scroll: ScrollType);

    //fn connect_input<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_output<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_wrapped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpinButton>> SpinButtonExt for O {
    fn configure<P: IsA<Adjustment>>(&self, adjustment: Option<&P>, climb_rate: f64, digits: u32) {
        unsafe {
            gtk_sys::gtk_spin_button_configure(
                self.as_ref().to_glib_none().0,
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            );
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(gtk_sys::gtk_spin_button_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_digits(&self) -> u32 {
        unsafe { gtk_sys::gtk_spin_button_get_digits(self.as_ref().to_glib_none().0) }
    }

    fn get_increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::MaybeUninit::uninit();
            let mut page = mem::MaybeUninit::uninit();
            gtk_sys::gtk_spin_button_get_increments(
                self.as_ref().to_glib_none().0,
                step.as_mut_ptr(),
                page.as_mut_ptr(),
            );
            let step = step.assume_init();
            let page = page.assume_init();
            (step, page)
        }
    }

    fn get_numeric(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_numeric(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            gtk_sys::gtk_spin_button_get_range(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    fn get_snap_to_ticks(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_snap_to_ticks(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_update_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe { gtk_sys::gtk_spin_button_get_value(self.as_ref().to_glib_none().0) }
    }

    fn get_value_as_int(&self) -> i32 {
        unsafe { gtk_sys::gtk_spin_button_get_value_as_int(self.as_ref().to_glib_none().0) }
    }

    fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_spin_button_get_wrap(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            gtk_sys::gtk_spin_button_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_digits(&self, digits: u32) {
        unsafe {
            gtk_sys::gtk_spin_button_set_digits(self.as_ref().to_glib_none().0, digits);
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_numeric(&self, numeric: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_numeric(self.as_ref().to_glib_none().0, numeric.to_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_snap_to_ticks(
                self.as_ref().to_glib_none().0,
                snap_to_ticks.to_glib(),
            );
        }
    }

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            gtk_sys::gtk_spin_button_set_update_policy(
                self.as_ref().to_glib_none().0,
                policy.to_glib(),
            );
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, wrap: bool) {
        unsafe {
            gtk_sys::gtk_spin_button_set_wrap(self.as_ref().to_glib_none().0, wrap.to_glib());
        }
    }

    fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            gtk_sys::gtk_spin_button_spin(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
                increment,
            );
        }
    }

    fn update(&self) {
        unsafe {
            gtk_sys::gtk_spin_button_update(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"climb-rate\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `climb-rate` getter")
                .unwrap()
        }
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"climb-rate\0".as_ptr() as *const _,
                Value::from(&climb_rate).to_glib_none().0,
            );
        }
    }

    fn connect_change_value<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn change_value_trampoline<P, F: Fn(&P, ScrollType) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            scroll: gtk_sys::GtkScrollType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SpinButton::from_glib_borrow(this).unsafe_cast(),
                from_glib(scroll),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-value\0".as_ptr() as *const _,
                Some(transmute(change_value_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_change_value(&self, scroll: ScrollType) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("change-value", &[&scroll])
                .unwrap()
        };
    }

    //fn connect_input<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Out new_value: *.Double
    //}

    fn connect_output<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn output_trampoline<P, F: Fn(&P) -> Inhibit + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"output\0".as_ptr() as *const _,
                Some(transmute(output_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute(value_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_wrapped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn wrapped_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"wrapped\0".as_ptr() as *const _,
                Some(transmute(wrapped_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_climb_rate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute(notify_climb_rate_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute(notify_digits_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_numeric_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::numeric\0".as_ptr() as *const _,
                Some(transmute(notify_numeric_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_snap_to_ticks_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::snap-to-ticks\0".as_ptr() as *const _,
                Some(transmute(
                    notify_snap_to_ticks_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_policy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::update-policy\0".as_ptr() as *const _,
                Some(transmute(
                    notify_update_policy_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSpinButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SpinButton>,
        {
            let f: &F = &*(f as *const F);
            f(&SpinButton::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute(notify_wrap_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SpinButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpinButton")
    }
}
