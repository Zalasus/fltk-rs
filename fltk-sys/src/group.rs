/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type custom_draw_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Group {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Group_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Group;
}
extern "C" {
    pub fn Fl_Group_x(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_y(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_width(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_height(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_label(arg1: *mut Fl_Group) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Group_set_label(arg1: *mut Fl_Group, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Group_redraw(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_show(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_hide(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_activate(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_deactivate(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_redraw_label(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_resize(
        arg1: *mut Fl_Group,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Group_tooltip(arg1: *mut Fl_Group) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Group_set_tooltip(arg1: *mut Fl_Group, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Group_get_type(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_type(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_color(arg1: *mut Fl_Group) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Group_set_color(arg1: *mut Fl_Group, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Group_label_color(arg1: *mut Fl_Group) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Group_set_label_color(arg1: *mut Fl_Group, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Group_label_font(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_font(arg1: *mut Fl_Group, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_size(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_size(arg1: *mut Fl_Group, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_label_type(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_label_type(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_box(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_box(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_changed(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_changed(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_clear_changed(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_align(arg1: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_set_align(arg1: *mut Fl_Group, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_delete(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_set_image(arg1: *mut Fl_Group, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Group_set_image_with_size(
        arg1: *mut Fl_Group,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Group_set_handler(
        self_: *mut Fl_Group,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Group_set_draw(
        self_: *mut Fl_Group,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Group_set_trigger(arg1: *mut Fl_Group, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_image(arg1: *const Fl_Group) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Group_parent(self_: *const Fl_Group) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Group_selection_color(arg1: *mut Fl_Group) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Group_set_selection_color(arg1: *mut Fl_Group, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Group_do_callback(arg1: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_inside(
        self_: *const Fl_Group,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_window(arg1: *const Fl_Group) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Group_top_window(arg1: *const Fl_Group) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Group_takes_events(arg1: *const Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_begin(self_: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_end(self_: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_find(
        self_: *mut Fl_Group,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_add(self_: *mut Fl_Group, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Group_insert(
        self_: *mut Fl_Group,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Group_remove(self_: *mut Fl_Group, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Group_clear(self_: *mut Fl_Group);
}
extern "C" {
    pub fn Fl_Group_children(self_: *mut Fl_Group) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Group_child(arg1: *mut Fl_Group, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Group_resizable(self_: *mut Fl_Group, arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Pack {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Pack_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Pack;
}
extern "C" {
    pub fn Fl_Pack_x(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_y(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_width(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_height(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_label(arg1: *mut Fl_Pack) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Pack_set_label(arg1: *mut Fl_Pack, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Pack_redraw(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_show(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_hide(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_activate(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_deactivate(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_redraw_label(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_resize(
        arg1: *mut Fl_Pack,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pack_tooltip(arg1: *mut Fl_Pack) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Pack_set_tooltip(arg1: *mut Fl_Pack, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Pack_get_type(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_type(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_color(arg1: *mut Fl_Pack) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Pack_set_color(arg1: *mut Fl_Pack, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Pack_label_color(arg1: *mut Fl_Pack) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Pack_set_label_color(arg1: *mut Fl_Pack, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Pack_label_font(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_font(arg1: *mut Fl_Pack, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_size(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_size(arg1: *mut Fl_Pack, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_label_type(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_label_type(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_box(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_box(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_changed(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_changed(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_clear_changed(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_align(arg1: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_set_align(arg1: *mut Fl_Pack, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_delete(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_set_image(arg1: *mut Fl_Pack, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Pack_set_image_with_size(
        arg1: *mut Fl_Pack,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pack_set_handler(
        self_: *mut Fl_Pack,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Pack_set_draw(
        self_: *mut Fl_Pack,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Pack_set_trigger(arg1: *mut Fl_Pack, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_image(arg1: *const Fl_Pack) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Pack_parent(self_: *const Fl_Pack) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Pack_selection_color(arg1: *mut Fl_Pack) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Pack_set_selection_color(arg1: *mut Fl_Pack, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Pack_do_callback(arg1: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_inside(
        self_: *const Fl_Pack,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_window(arg1: *const Fl_Pack) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Pack_top_window(arg1: *const Fl_Pack) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Pack_takes_events(arg1: *const Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_begin(self_: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_end(self_: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_find(
        self_: *mut Fl_Pack,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_add(self_: *mut Fl_Pack, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Pack_insert(
        self_: *mut Fl_Pack,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pack_remove(self_: *mut Fl_Pack, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Pack_clear(self_: *mut Fl_Pack);
}
extern "C" {
    pub fn Fl_Pack_children(self_: *mut Fl_Pack) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pack_child(arg1: *mut Fl_Pack, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Pack_resizable(self_: *mut Fl_Pack, arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Scroll {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Scroll_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Scroll;
}
extern "C" {
    pub fn Fl_Scroll_x(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_y(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_width(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_height(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_label(arg1: *mut Fl_Scroll) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Scroll_set_label(arg1: *mut Fl_Scroll, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scroll_redraw(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_show(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_hide(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_activate(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_deactivate(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_redraw_label(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_resize(
        arg1: *mut Fl_Scroll,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Scroll_tooltip(arg1: *mut Fl_Scroll) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Scroll_set_tooltip(arg1: *mut Fl_Scroll, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Scroll_get_type(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_type(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_color(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Scroll_set_color(arg1: *mut Fl_Scroll, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Scroll_label_color(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Scroll_set_label_color(arg1: *mut Fl_Scroll, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Scroll_label_font(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_font(arg1: *mut Fl_Scroll, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_size(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_size(arg1: *mut Fl_Scroll, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_label_type(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_label_type(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_box(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_box(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_changed(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_changed(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_clear_changed(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_align(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_set_align(arg1: *mut Fl_Scroll, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_delete(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_set_image(arg1: *mut Fl_Scroll, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Scroll_set_image_with_size(
        arg1: *mut Fl_Scroll,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Scroll_set_handler(
        self_: *mut Fl_Scroll,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Scroll_set_draw(
        self_: *mut Fl_Scroll,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Scroll_set_trigger(arg1: *mut Fl_Scroll, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_image(arg1: *const Fl_Scroll) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Scroll_parent(self_: *const Fl_Scroll) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Scroll_selection_color(arg1: *mut Fl_Scroll) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Scroll_set_selection_color(arg1: *mut Fl_Scroll, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Scroll_do_callback(arg1: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_inside(
        self_: *const Fl_Scroll,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_window(arg1: *const Fl_Scroll) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Scroll_top_window(arg1: *const Fl_Scroll) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Scroll_takes_events(arg1: *const Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_begin(self_: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_end(self_: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_find(
        self_: *mut Fl_Scroll,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_add(self_: *mut Fl_Scroll, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Scroll_insert(
        self_: *mut Fl_Scroll,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Scroll_remove(self_: *mut Fl_Scroll, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Scroll_clear(self_: *mut Fl_Scroll);
}
extern "C" {
    pub fn Fl_Scroll_children(self_: *mut Fl_Scroll) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Scroll_child(arg1: *mut Fl_Scroll, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Scroll_resizable(self_: *mut Fl_Scroll, arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tabs {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tabs_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tabs;
}
extern "C" {
    pub fn Fl_Tabs_x(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_y(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_width(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_height(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_label(arg1: *mut Fl_Tabs) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tabs_set_label(arg1: *mut Fl_Tabs, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tabs_redraw(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_show(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_hide(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_activate(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_deactivate(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_redraw_label(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_resize(
        arg1: *mut Fl_Tabs,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tabs_tooltip(arg1: *mut Fl_Tabs) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tabs_set_tooltip(arg1: *mut Fl_Tabs, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tabs_get_type(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_type(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_color(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tabs_set_color(arg1: *mut Fl_Tabs, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tabs_label_color(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tabs_set_label_color(arg1: *mut Fl_Tabs, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tabs_label_font(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_font(arg1: *mut Fl_Tabs, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_size(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_size(arg1: *mut Fl_Tabs, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_label_type(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_label_type(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_box(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_box(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_changed(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_changed(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_clear_changed(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_align(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_set_align(arg1: *mut Fl_Tabs, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_delete(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_set_image(arg1: *mut Fl_Tabs, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tabs_set_image_with_size(
        arg1: *mut Fl_Tabs,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tabs_set_handler(
        self_: *mut Fl_Tabs,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tabs_set_draw(
        self_: *mut Fl_Tabs,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tabs_set_trigger(arg1: *mut Fl_Tabs, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_image(arg1: *const Fl_Tabs) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tabs_parent(self_: *const Fl_Tabs) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tabs_selection_color(arg1: *mut Fl_Tabs) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tabs_set_selection_color(arg1: *mut Fl_Tabs, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tabs_do_callback(arg1: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_inside(
        self_: *const Fl_Tabs,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_window(arg1: *const Fl_Tabs) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tabs_top_window(arg1: *const Fl_Tabs) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tabs_takes_events(arg1: *const Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_begin(self_: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_end(self_: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_find(
        self_: *mut Fl_Tabs,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_add(self_: *mut Fl_Tabs, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tabs_insert(
        self_: *mut Fl_Tabs,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tabs_remove(self_: *mut Fl_Tabs, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tabs_clear(self_: *mut Fl_Tabs);
}
extern "C" {
    pub fn Fl_Tabs_children(self_: *mut Fl_Tabs) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tabs_child(arg1: *mut Fl_Tabs, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Tabs_resizable(self_: *mut Fl_Tabs, arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tile {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tile_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Tile;
}
extern "C" {
    pub fn Fl_Tile_x(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_y(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_width(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_height(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_label(arg1: *mut Fl_Tile) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tile_set_label(arg1: *mut Fl_Tile, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tile_redraw(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_show(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_hide(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_activate(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_deactivate(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_redraw_label(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_resize(
        arg1: *mut Fl_Tile,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tile_tooltip(arg1: *mut Fl_Tile) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tile_set_tooltip(arg1: *mut Fl_Tile, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Tile_get_type(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_type(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_color(arg1: *mut Fl_Tile) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tile_set_color(arg1: *mut Fl_Tile, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tile_label_color(arg1: *mut Fl_Tile) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tile_set_label_color(arg1: *mut Fl_Tile, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tile_label_font(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_font(arg1: *mut Fl_Tile, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_size(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_size(arg1: *mut Fl_Tile, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_label_type(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_label_type(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_box(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_box(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_changed(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_changed(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_clear_changed(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_align(arg1: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_set_align(arg1: *mut Fl_Tile, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_delete(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_set_image(arg1: *mut Fl_Tile, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tile_set_image_with_size(
        arg1: *mut Fl_Tile,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tile_set_handler(
        self_: *mut Fl_Tile,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tile_set_draw(
        self_: *mut Fl_Tile,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Tile_set_trigger(arg1: *mut Fl_Tile, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_image(arg1: *const Fl_Tile) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tile_parent(self_: *const Fl_Tile) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tile_selection_color(arg1: *mut Fl_Tile) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Tile_set_selection_color(arg1: *mut Fl_Tile, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Tile_do_callback(arg1: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_inside(
        self_: *const Fl_Tile,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_window(arg1: *const Fl_Tile) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tile_top_window(arg1: *const Fl_Tile) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Tile_takes_events(arg1: *const Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_begin(self_: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_end(self_: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_find(
        self_: *mut Fl_Tile,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_add(self_: *mut Fl_Tile, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Tile_insert(
        self_: *mut Fl_Tile,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tile_remove(self_: *mut Fl_Tile, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Tile_clear(self_: *mut Fl_Tile);
}
extern "C" {
    pub fn Fl_Tile_children(self_: *mut Fl_Tile) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tile_child(arg1: *mut Fl_Tile, index: ::std::os::raw::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Tile_resizable(self_: *mut Fl_Tile, arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Color_Chooser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Color_Chooser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Color_Chooser;
}
extern "C" {
    pub fn Fl_Color_Chooser_x(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_y(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_width(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_height(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_label(arg1: *mut Fl_Color_Chooser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_label(
        arg1: *mut Fl_Color_Chooser,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_redraw(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_show(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_hide(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_activate(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_deactivate(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_redraw_label(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_resize(
        arg1: *mut Fl_Color_Chooser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_tooltip(arg1: *mut Fl_Color_Chooser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_tooltip(
        arg1: *mut Fl_Color_Chooser,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_get_type(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_type(arg1: *mut Fl_Color_Chooser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_color(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_color(arg1: *mut Fl_Color_Chooser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Color_Chooser_label_color(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_label_color(
        arg1: *mut Fl_Color_Chooser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_label_font(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_label_font(
        arg1: *mut Fl_Color_Chooser,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_label_size(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_label_size(arg1: *mut Fl_Color_Chooser, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_label_type(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_label_type(arg1: *mut Fl_Color_Chooser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_box(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_box(arg1: *mut Fl_Color_Chooser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_changed(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_changed(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_clear_changed(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_align(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_align(arg1: *mut Fl_Color_Chooser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_delete(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_set_image(
        arg1: *mut Fl_Color_Chooser,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_set_image_with_size(
        arg1: *mut Fl_Color_Chooser,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_set_handler(
        self_: *mut Fl_Color_Chooser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_set_draw(
        self_: *mut Fl_Color_Chooser,
        cb: custom_draw_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_set_trigger(arg1: *mut Fl_Color_Chooser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_image(arg1: *const Fl_Color_Chooser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Color_Chooser_parent(self_: *const Fl_Color_Chooser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Color_Chooser_selection_color(arg1: *mut Fl_Color_Chooser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Color_Chooser_set_selection_color(
        arg1: *mut Fl_Color_Chooser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_do_callback(arg1: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_inside(
        self_: *const Fl_Color_Chooser,
        arg1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_window(arg1: *const Fl_Color_Chooser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Color_Chooser_top_window(
        arg1: *const Fl_Color_Chooser,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Color_Chooser_takes_events(arg1: *const Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_r(self_: *mut Fl_Color_Chooser) -> f64;
}
extern "C" {
    pub fn Fl_Color_Chooser_g(self_: *mut Fl_Color_Chooser) -> f64;
}
extern "C" {
    pub fn Fl_Color_Chooser_b(self_: *mut Fl_Color_Chooser) -> f64;
}
extern "C" {
    pub fn Fl_Color_Chooser_begin(self_: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_end(self_: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_find(
        self_: *mut Fl_Color_Chooser,
        arg1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_add(self_: *mut Fl_Color_Chooser, arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Color_Chooser_insert(
        self_: *mut Fl_Color_Chooser,
        arg1: *mut ::std::os::raw::c_void,
        pos: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Color_Chooser_remove(self_: *mut Fl_Color_Chooser, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Color_Chooser_clear(self_: *mut Fl_Color_Chooser);
}
extern "C" {
    pub fn Fl_Color_Chooser_children(self_: *mut Fl_Color_Chooser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Color_Chooser_child(
        arg1: *mut Fl_Color_Chooser,
        index: ::std::os::raw::c_int,
    ) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Color_Chooser_resizable(
        self_: *mut Fl_Color_Chooser,
        arg1: *mut ::std::os::raw::c_void,
    );
}
