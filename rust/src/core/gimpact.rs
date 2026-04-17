use std::ffi::c_void;
use std::ptr::NonNull;

use super::collision::ShapeHandle;
use super::types::{Real, Vec3};
use crate::ffi;

pub struct GImpactShape {
    handle: NonNull<c_void>,
}

impl GImpactShape {
    pub unsafe fn from_trimesh(trimesh_handle: *mut c_void, scale: Vec3) -> Self {
        let handle = ffi::nk_gimpact_shape_create(
            trimesh_handle,
            scale.x,
            scale.y,
            scale.z,
        );
        Self {
            handle: NonNull::new(handle).expect("Failed to create GImpact shape"),
        }
    }

    pub unsafe fn from_shape_handle(trimesh: &ShapeHandle, scale: Vec3) -> Self {
        Self::from_trimesh(trimesh.handle(), scale)
    }

    pub fn update_bound(&mut self) {
        unsafe {
            ffi::nk_gimpact_shape_update_bound(self.handle.as_ptr());
        }
    }

    pub fn num_child_shapes(&self) -> i32 {
        unsafe { ffi::nk_gimpact_shape_get_num_child_shapes(self.handle.as_ptr()) }
    }

    pub fn set_margin(&mut self, margin: Real) {
        unsafe {
            ffi::nk_gimpact_shape_set_margin(self.handle.as_ptr(), margin);
        }
    }

    pub fn get_margin(&self) -> Real {
        unsafe { ffi::nk_gimpact_shape_get_margin(self.handle.as_ptr()) }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for GImpactShape {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_gimpact_shape_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for GImpactShape {}
unsafe impl Sync for GImpactShape {}
