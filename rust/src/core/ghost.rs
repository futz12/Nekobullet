use std::ffi::c_void;
use std::ptr::NonNull;

use super::collision::ShapeHandle;
use super::types::{Quat, Transform, Vec3};
use crate::ffi::{self, nkTransform};

pub struct GhostObject {
    handle: NonNull<c_void>,
    shape: Option<ShapeHandle>,
}

impl GhostObject {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_ghost_create() };
        Self {
            handle: unsafe { NonNull::new_unchecked(handle) },
            shape: None,
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn set_shape(&mut self, shape: ShapeHandle) {
        unsafe {
            ffi::nk_ghost_set_shape(self.handle.as_ptr(), shape.handle());
        }
        self.shape = Some(shape);
    }

    pub fn get_shape(&self) -> Option<&ShapeHandle> {
        self.shape.as_ref()
    }

    pub fn set_transform(&self, transform: &Transform) {
        let nk_transform = nkTransform::from_core_transform(transform);
        unsafe {
            ffi::nk_ghost_set_transform(self.handle.as_ptr(), &nk_transform);
        }
    }

    pub fn get_transform(&self) -> Transform {
        let mut nk_transform: nkTransform = nkTransform::default();
        unsafe {
            ffi::nk_ghost_get_transform(self.handle.as_ptr(), &mut nk_transform);
        }
        nk_transform.to_core_transform()
    }

    pub fn set_position(&self, position: Vec3) {
        let mut transform = self.get_transform();
        transform.position = position;
        self.set_transform(&transform);
    }

    pub fn get_position(&self) -> Vec3 {
        self.get_transform().position
    }

    pub fn set_rotation(&self, rotation: Quat) {
        let mut transform = self.get_transform();
        transform.rotation = rotation;
        self.set_transform(&transform);
    }

    pub fn get_rotation(&self) -> Quat {
        self.get_transform().rotation
    }

    pub fn get_num_overlapping_objects(&self) -> i32 {
        unsafe { ffi::nk_ghost_get_num_overlapping_objects(self.handle.as_ptr()) }
    }

    pub fn get_overlapping_object(&self, index: i32) -> Option<*mut c_void> {
        if index < 0 || index >= self.get_num_overlapping_objects() {
            return None;
        }
        let obj = unsafe { ffi::nk_ghost_get_overlapping_object(self.handle.as_ptr(), index) };
        if obj.is_null() {
            None
        } else {
            Some(obj)
        }
    }

    pub fn overlapping_objects(&self) -> OverlappingObjectsIter<'_> {
        OverlappingObjectsIter {
            ghost: self,
            index: 0,
            count: self.get_num_overlapping_objects(),
        }
    }
}

impl Default for GhostObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GhostObject {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_ghost_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for GhostObject {}
unsafe impl Sync for GhostObject {}

pub struct OverlappingObjectsIter<'a> {
    ghost: &'a GhostObject,
    index: i32,
    count: i32,
}

impl<'a> Iterator for OverlappingObjectsIter<'a> {
    type Item = *mut c_void;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let obj = self.ghost.get_overlapping_object(self.index);
        self.index += 1;
        obj
    }
}

pub struct GhostObjectBuilder {
    shape: Option<ShapeHandle>,
    position: Vec3,
    rotation: Quat,
}

impl GhostObjectBuilder {
    pub fn new() -> Self {
        Self {
            shape: None,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn shape(mut self, shape: ShapeHandle) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn build(self) -> Result<GhostObject, &'static str> {
        let mut ghost = GhostObject::new();
        
        if let Some(shape) = self.shape {
            ghost.set_shape(shape);
        }
        
        ghost.set_transform(&Transform::new(self.position, self.rotation));
        
        Ok(ghost)
    }
}

impl Default for GhostObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
