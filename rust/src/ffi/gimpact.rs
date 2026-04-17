use super::types::{nkReal, nkShapeHandle};

pub type nkGImpactShapeHandle = *mut std::ffi::c_void;

extern "C" {
    pub fn nk_gimpact_shape_create(
        trimesh: nkShapeHandle,
        scale_x: nkReal,
        scale_y: nkReal,
        scale_z: nkReal,
    ) -> nkGImpactShapeHandle;
    pub fn nk_gimpact_shape_destroy(shape: nkGImpactShapeHandle);
    pub fn nk_gimpact_shape_update_bound(shape: nkGImpactShapeHandle);
    pub fn nk_gimpact_shape_get_num_child_shapes(shape: nkGImpactShapeHandle) -> i32;
    pub fn nk_gimpact_shape_set_margin(shape: nkGImpactShapeHandle, margin: nkReal);
    pub fn nk_gimpact_shape_get_margin(shape: nkGImpactShapeHandle) -> nkReal;
}

pub unsafe fn gimpact_shape_create(
    trimesh: nkShapeHandle,
    scale_x: nkReal,
    scale_y: nkReal,
    scale_z: nkReal,
) -> nkGImpactShapeHandle {
    nk_gimpact_shape_create(trimesh, scale_x, scale_y, scale_z)
}

pub unsafe fn gimpact_shape_destroy(shape: nkGImpactShapeHandle) {
    nk_gimpact_shape_destroy(shape);
}

pub unsafe fn gimpact_shape_update_bound(shape: nkGImpactShapeHandle) {
    nk_gimpact_shape_update_bound(shape);
}

pub unsafe fn gimpact_shape_get_num_child_shapes(shape: nkGImpactShapeHandle) -> i32 {
    nk_gimpact_shape_get_num_child_shapes(shape)
}

pub unsafe fn gimpact_shape_set_margin(shape: nkGImpactShapeHandle, margin: nkReal) {
    nk_gimpact_shape_set_margin(shape, margin);
}

pub unsafe fn gimpact_shape_get_margin(shape: nkGImpactShapeHandle) -> nkReal {
    nk_gimpact_shape_get_margin(shape)
}
