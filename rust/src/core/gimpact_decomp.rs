use std::ffi::c_void;
use std::ptr::NonNull;

use super::collision::ShapeHandle;
use super::types::{Real, Transform, Vec3};
use crate::ffi;

pub struct GImpactDecompShape {
    handle: NonNull<c_void>,
}

impl GImpactDecompShape {
    pub fn new(
        trimesh: &ShapeHandle,
        scale: Vec3,
        margin: Real,
        transform_subshapes: bool,
    ) -> Option<Self> {
        let handle = unsafe {
            ffi::nk_gimpact_decomp_shape_create(
                trimesh.handle(),
                scale.x,
                scale.y,
                scale.z,
                margin,
                if transform_subshapes { 1 } else { 0 },
            )
        };
        
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn num_child_shapes(&self) -> i32 {
        unsafe { ffi::nk_gimpact_decomp_shape_get_num_child_shapes(self.handle.as_ptr()) }
    }

    pub fn update_bound(&self) {
        unsafe {
            ffi::nk_gimpact_decomp_shape_update_bound(self.handle.as_ptr());
        }
    }

    pub fn set_margin(&self, margin: Real) {
        unsafe {
            ffi::nk_gimpact_decomp_shape_set_margin(self.handle.as_ptr(), margin);
        }
    }

    pub fn get_margin(&self) -> Real {
        unsafe { ffi::nk_gimpact_decomp_shape_get_margin(self.handle.as_ptr()) }
    }

    pub fn get_child_shape_ptr(&self, index: i32) -> Option<*mut c_void> {
        if index < 0 || index >= self.num_child_shapes() {
            return None;
        }

        let shape = unsafe {
            ffi::nk_gimpact_decomp_shape_get_child_shape(
                self.handle.as_ptr(),
                index,
            )
        };

        if shape.is_null() {
            None
        } else {
            Some(shape)
        }
    }

    pub fn get_child_transform(&self, index: i32) -> Option<Transform> {
        if index < 0 || index >= self.num_child_shapes() {
            return None;
        }

        let mut pos_x: Real = 0.0;
        let mut pos_y: Real = 0.0;
        let mut pos_z: Real = 0.0;
        let mut rot_x: Real = 0.0;
        let mut rot_y: Real = 0.0;
        let mut rot_z: Real = 0.0;
        let mut rot_w: Real = 1.0;

        unsafe {
            ffi::nk_gimpact_decomp_shape_get_child_transform(
                self.handle.as_ptr(),
                index,
                &mut pos_x,
                &mut pos_y,
                &mut pos_z,
                &mut rot_x,
                &mut rot_y,
                &mut rot_z,
                &mut rot_w,
            );
        }

        Some(Transform {
            position: Vec3::new(pos_x, pos_y, pos_z),
            rotation: glam::Quat::from_xyzw(rot_x, rot_y, rot_z, rot_w),
        })
    }
}

impl Drop for GImpactDecompShape {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_gimpact_decomp_shape_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for GImpactDecompShape {}
unsafe impl Sync for GImpactDecompShape {}
