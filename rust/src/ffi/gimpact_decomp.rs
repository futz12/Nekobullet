use super::types::{nkReal, nkShapeHandle};

pub type nkGImpactDecompShapeHandle = *mut std::ffi::c_void;

extern "C" {
    pub fn nk_gimpact_decomp_shape_create(
        trimesh: nkShapeHandle,
        scale_x: nkReal,
        scale_y: nkReal,
        scale_z: nkReal,
        margin: nkReal,
        transform_subshapes: i32,
    ) -> nkGImpactDecompShapeHandle;
    pub fn nk_gimpact_decomp_shape_destroy(shape: nkGImpactDecompShapeHandle);
    pub fn nk_gimpact_decomp_shape_get_num_child_shapes(shape: nkGImpactDecompShapeHandle) -> i32;
    pub fn nk_gimpact_decomp_shape_update_bound(shape: nkGImpactDecompShapeHandle);
    pub fn nk_gimpact_decomp_shape_set_margin(shape: nkGImpactDecompShapeHandle, margin: nkReal);
    pub fn nk_gimpact_decomp_shape_get_margin(shape: nkGImpactDecompShapeHandle) -> nkReal;
    pub fn nk_gimpact_decomp_shape_get_child_shape(
        shape: nkGImpactDecompShapeHandle,
        index: i32,
    ) -> nkShapeHandle;
    pub fn nk_gimpact_decomp_shape_get_child_transform(
        shape: nkGImpactDecompShapeHandle,
        index: i32,
        out_pos_x: *mut nkReal,
        out_pos_y: *mut nkReal,
        out_pos_z: *mut nkReal,
        out_rot_x: *mut nkReal,
        out_rot_y: *mut nkReal,
        out_rot_z: *mut nkReal,
        out_rot_w: *mut nkReal,
    );
}
