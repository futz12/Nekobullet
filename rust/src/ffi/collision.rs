use super::types::{nkReal, nkShapeHandle, nkTransform};

extern "C" {
    pub fn nk_shape_create_box(half_extent_x: nkReal, half_extent_y: nkReal, half_extent_z: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_sphere(radius: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_capsule(radius: nkReal, height: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_capsule_z(radius: nkReal, height: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_cylinder(radius: nkReal, height: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_cone(radius: nkReal, height: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_plane(normal_x: nkReal, normal_y: nkReal, normal_z: nkReal, constant: nkReal) -> nkShapeHandle;
    pub fn nk_shape_create_convex_hull(points: *const nkReal, num_points: i32) -> nkShapeHandle;
    pub fn nk_shape_create_compound() -> nkShapeHandle;
    pub fn nk_compound_add_child(compound: nkShapeHandle, child_shape: nkShapeHandle, local_transform: *const nkTransform);
    pub fn nk_shape_create_triangle_mesh(
        vertices: *const nkReal,
        num_vertices: i32,
        indices: *const i32,
        num_indices: i32,
    ) -> nkShapeHandle;
    pub fn nk_shape_create_heightfield(
        width: i32,
        length: i32,
        height_data: *const nkReal,
        min_height: nkReal,
        max_height: nkReal,
        up_axis: i32,
    ) -> nkShapeHandle;
    pub fn nk_shape_destroy(shape: nkShapeHandle);
    pub fn nk_shape_get_type(shape: nkShapeHandle) -> i32;
    pub fn nk_shape_set_local_scaling(shape: nkShapeHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_shape_get_local_scaling(shape: nkShapeHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_shape_calculate_local_inertia(shape: nkShapeHandle, mass: nkReal, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_shape_set_margin(shape: nkShapeHandle, margin: nkReal);
    pub fn nk_shape_get_margin(shape: nkShapeHandle) -> nkReal;
    pub fn nk_shape_get_angular_motion_disc(shape: nkShapeHandle) -> nkReal;
    pub fn nk_shape_get_contact_breaking_threshold(shape: nkShapeHandle, default_contact_threshold: nkReal) -> nkReal;
    pub fn nk_box_get_half_extents(shape: nkShapeHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_sphere_get_radius(shape: nkShapeHandle) -> nkReal;
    pub fn nk_capsule_get_radius(shape: nkShapeHandle) -> nkReal;
    pub fn nk_capsule_get_half_height(shape: nkShapeHandle) -> nkReal;
    pub fn nk_cylinder_get_radius(shape: nkShapeHandle) -> nkReal;
    pub fn nk_cylinder_get_half_height(shape: nkShapeHandle) -> nkReal;
    pub fn nk_cone_get_radius(shape: nkShapeHandle) -> nkReal;
    pub fn nk_cone_get_height(shape: nkShapeHandle) -> nkReal;
}

pub const SHAPE_TYPE_BOX: i32 = 0;
pub const SHAPE_TYPE_SPHERE: i32 = 1;
pub const SHAPE_TYPE_CAPSULE: i32 = 2;
pub const SHAPE_TYPE_CYLINDER: i32 = 3;
pub const SHAPE_TYPE_CONE: i32 = 4;
pub const SHAPE_TYPE_COMPOUND: i32 = 5;
pub const SHAPE_TYPE_CONVEX_HULL: i32 = 6;
pub const SHAPE_TYPE_TRIANGLE_MESH: i32 = 7;
pub const SHAPE_TYPE_HEIGHTFIELD: i32 = 8;
pub const SHAPE_TYPE_STATIC_PLANE: i32 = 9;

pub unsafe fn shape_create_box(half_extent_x: nkReal, half_extent_y: nkReal, half_extent_z: nkReal) -> nkShapeHandle {
    nk_shape_create_box(half_extent_x, half_extent_y, half_extent_z)
}

pub unsafe fn shape_create_sphere(radius: nkReal) -> nkShapeHandle {
    nk_shape_create_sphere(radius)
}

pub unsafe fn shape_create_capsule(radius: nkReal, height: nkReal) -> nkShapeHandle {
    nk_shape_create_capsule(radius, height)
}

pub unsafe fn shape_create_capsule_z(radius: nkReal, height: nkReal) -> nkShapeHandle {
    nk_shape_create_capsule_z(radius, height)
}

pub unsafe fn shape_create_cylinder(radius: nkReal, height: nkReal) -> nkShapeHandle {
    nk_shape_create_cylinder(radius, height)
}

pub unsafe fn shape_create_cone(radius: nkReal, height: nkReal) -> nkShapeHandle {
    nk_shape_create_cone(radius, height)
}

pub unsafe fn shape_destroy(shape: nkShapeHandle) {
    nk_shape_destroy(shape);
}

pub unsafe fn shape_get_type(shape: nkShapeHandle) -> i32 {
    nk_shape_get_type(shape)
}

pub unsafe fn shape_set_local_scaling(shape: nkShapeHandle, x: nkReal, y: nkReal, z: nkReal) {
    nk_shape_set_local_scaling(shape, x, y, z);
}

pub unsafe fn shape_get_local_scaling(shape: nkShapeHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal) {
    nk_shape_get_local_scaling(shape, out_x, out_y, out_z);
}

pub unsafe fn shape_calculate_local_inertia(shape: nkShapeHandle, mass: nkReal, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal) {
    nk_shape_calculate_local_inertia(shape, mass, out_x, out_y, out_z);
}

pub unsafe fn shape_set_margin(shape: nkShapeHandle, margin: nkReal) {
    nk_shape_set_margin(shape, margin);
}

pub unsafe fn shape_get_margin(shape: nkShapeHandle) -> nkReal {
    nk_shape_get_margin(shape)
}

pub unsafe fn shape_get_angular_motion_disc(shape: nkShapeHandle) -> nkReal {
    nk_shape_get_angular_motion_disc(shape)
}

pub unsafe fn shape_get_contact_breaking_threshold(shape: nkShapeHandle, default_contact_threshold: nkReal) -> nkReal {
    nk_shape_get_contact_breaking_threshold(shape, default_contact_threshold)
}

pub unsafe fn box_get_half_extents(shape: nkShapeHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal) {
    nk_box_get_half_extents(shape, out_x, out_y, out_z);
}

pub unsafe fn sphere_get_radius(shape: nkShapeHandle) -> nkReal {
    nk_sphere_get_radius(shape)
}

pub unsafe fn capsule_get_radius(shape: nkShapeHandle) -> nkReal {
    nk_capsule_get_radius(shape)
}

pub unsafe fn capsule_get_half_height(shape: nkShapeHandle) -> nkReal {
    nk_capsule_get_half_height(shape)
}

pub unsafe fn cylinder_get_radius(shape: nkShapeHandle) -> nkReal {
    nk_cylinder_get_radius(shape)
}

pub unsafe fn cylinder_get_half_height(shape: nkShapeHandle) -> nkReal {
    nk_cylinder_get_half_height(shape)
}

pub unsafe fn cone_get_radius(shape: nkShapeHandle) -> nkReal {
    nk_cone_get_radius(shape)
}

pub unsafe fn cone_get_height(shape: nkShapeHandle) -> nkReal {
    nk_cone_get_height(shape)
}
