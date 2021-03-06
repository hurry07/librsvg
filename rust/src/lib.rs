#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate downcast_rs;

pub use aspect_ratio::{
    rsvg_aspect_ratio_parse,
    rsvg_aspect_ratio_compute
};

pub use bbox::{
    RsvgBbox,
    rsvg_bbox_init,
    rsvg_bbox_insert,
    rsvg_bbox_clip
};

pub use cnode::{
    rsvg_rust_cnode_new,
    rsvg_rust_cnode_get_impl
};

pub use gradient::{
    gradient_linear_new,
    gradient_radial_new,
    gradient_destroy,
    gradient_add_color_stop,
    gradient_resolve_fallbacks_and_set_pattern
};

pub use length::{
    LengthUnit,
    LengthDir,
    RsvgLength,
    rsvg_length_parse,
    rsvg_length_normalize,
    rsvg_length_hand_normalize,
};

pub use marker::{
    rsvg_node_marker_new,
};

pub use node::{
    rsvg_node_get_type,
    rsvg_node_get_parent,
    rsvg_node_ref,
    rsvg_node_unref,
    rsvg_node_is_same,
    rsvg_node_get_state,
    rsvg_node_add_child,
    rsvg_node_set_atts,
    rsvg_node_draw,
    rsvg_node_set_attribute_parse_error,
    rsvg_node_foreach_child,
    rsvg_node_draw_children,
};

pub use path_builder::{
    rsvg_path_builder_add_to_cairo_context
};

pub use pattern::{
    pattern_new,
    pattern_destroy,
    pattern_resolve_fallbacks_and_set_pattern,
};

pub use shapes::{
    rsvg_node_circle_new,
    rsvg_node_ellipse_new,
    rsvg_node_line_new,
    rsvg_node_path_new,
    rsvg_node_polygon_new,
    rsvg_node_polyline_new,
    rsvg_node_rect_new,
};

pub use structure::{
    rsvg_node_group_new,
    rsvg_node_defs_new,
    rsvg_node_switch_new,
    rsvg_node_svg_new,
    rsvg_node_svg_get_size,
    rsvg_node_svg_get_view_box,
    rsvg_node_svg_apply_atts,
};

pub use transform::{
    rsvg_parse_transform,
};

pub use viewbox::{
    RsvgViewBox
};


mod aspect_ratio;
mod bbox;
mod cnode;
mod drawing_ctx;
mod error;
mod handle;
mod gradient;
mod length;
mod marker;
mod node;
mod paint_server;
mod pt;
mod parsers;
mod parse_transform;
mod path_builder;
mod path_parser;
mod pattern;
mod property_bag;
mod state;
mod shapes;
mod structure;
mod transform;
mod util;
mod viewbox;
