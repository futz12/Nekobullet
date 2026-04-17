use std::ffi::{c_void, CString, CStr};
use std::ptr::NonNull;

use super::inverse_dynamics::MultiBody;
use crate::ffi;

pub struct MultiBodyNameMap {
    handle: NonNull<c_void>,
}

impl MultiBodyNameMap {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_multibody_name_map_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create name map"),
        }
    }

    pub fn add_body(&self, index: i32, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe {
            ffi::nk_multibody_name_map_add_body(self.handle.as_ptr(), index, c_name.as_ptr())
        }
    }

    pub fn add_joint(&self, index: i32, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe {
            ffi::nk_multibody_name_map_add_joint(self.handle.as_ptr(), index, c_name.as_ptr())
        }
    }

    pub fn get_body_name(&self, index: i32) -> Option<String> {
        let mut buffer = [0i8; 256];
        let result = unsafe {
            ffi::nk_multibody_name_map_get_body_name(
                self.handle.as_ptr(),
                index,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };
        
        if result == 0 {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .ok()
                    .map(|s| s.to_string())
            }
        } else {
            None
        }
    }

    pub fn get_joint_name(&self, index: i32) -> Option<String> {
        let mut buffer = [0i8; 256];
        let result = unsafe {
            ffi::nk_multibody_name_map_get_joint_name(
                self.handle.as_ptr(),
                index,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };
        
        if result == 0 {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_str()
                    .ok()
                    .map(|s| s.to_string())
            }
        } else {
            None
        }
    }

    pub fn get_body_index(&self, name: &str) -> Option<i32> {
        let c_name = CString::new(name).ok()?;
        let mut index: i32 = -1;
        let result = unsafe {
            ffi::nk_multibody_name_map_get_body_index(
                self.handle.as_ptr(),
                c_name.as_ptr(),
                &mut index,
            )
        };
        
        if result == 0 { Some(index) } else { None }
    }

    pub fn get_joint_index(&self, name: &str) -> Option<i32> {
        let c_name = CString::new(name).ok()?;
        let mut index: i32 = -1;
        let result = unsafe {
            ffi::nk_multibody_name_map_get_joint_index(
                self.handle.as_ptr(),
                c_name.as_ptr(),
                &mut index,
            )
        };
        
        if result == 0 { Some(index) } else { None }
    }
}

impl Default for MultiBodyNameMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MultiBodyNameMap {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_multibody_name_map_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for MultiBodyNameMap {}
unsafe impl Sync for MultiBodyNameMap {}

pub struct MultiBodyTreeCreator {
    handle: NonNull<c_void>,
}

impl MultiBodyTreeCreator {
    pub fn from_bt_multibody(btmb: &MultiBody, verbose: bool) -> Option<Self> {
        let handle = unsafe {
            ffi::nk_multibody_tree_creator_from_bt_multibody(
                btmb.handle(),
                if verbose { 1 } else { 0 },
            )
        };
        
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn create_tree(&self) -> Option<MultiBody> {
        let handle = unsafe { ffi::nk_multibody_create_from_creator(self.handle.as_ptr()) };
        NonNull::new(handle).map(MultiBody::from_handle)
    }
}

impl Drop for MultiBodyTreeCreator {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_multibody_tree_creator_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for MultiBodyTreeCreator {}
unsafe impl Sync for MultiBodyTreeCreator {}

pub struct CloneTreeCreator {
    handle: NonNull<c_void>,
}

impl CloneTreeCreator {
    pub fn new(reference: &MultiBody) -> Option<Self> {
        let handle = unsafe { ffi::nk_clone_tree_creator_create(reference.handle()) };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn create_tree(&self) -> Option<MultiBody> {
        let handle = unsafe { ffi::nk_multibody_create_from_creator(self.handle.as_ptr()) };
        NonNull::new(handle).map(MultiBody::from_handle)
    }
}

impl Drop for CloneTreeCreator {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_clone_tree_creator_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for CloneTreeCreator {}
unsafe impl Sync for CloneTreeCreator {}

pub fn clone_multibody(reference: &MultiBody) -> Option<MultiBody> {
    let handle = unsafe { ffi::nk_multibody_clone(reference.handle()) };
    NonNull::new(handle).map(MultiBody::from_handle)
}

pub struct SimpleTreeCreator {
    handle: NonNull<c_void>,
}

impl SimpleTreeCreator {
    pub fn new(num_bodies: i32) -> Option<Self> {
        if num_bodies <= 0 {
            return None;
        }
        let handle = unsafe { ffi::nk_simple_tree_creator_create(num_bodies) };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn create_tree(&self) -> Option<MultiBody> {
        let handle = unsafe { ffi::nk_multibody_create_from_creator(self.handle.as_ptr()) };
        NonNull::new(handle).map(MultiBody::from_handle)
    }
}

impl Drop for SimpleTreeCreator {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_simple_tree_creator_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for SimpleTreeCreator {}
unsafe impl Sync for SimpleTreeCreator {}

pub struct User2InternalIndex {
    handle: NonNull<c_void>,
}

impl User2InternalIndex {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_user2internal_index_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create User2InternalIndex"),
        }
    }

    pub fn add_body(&self, body: i32, parent: i32) {
        unsafe {
            ffi::nk_user2internal_index_add_body(self.handle.as_ptr(), body, parent);
        }
    }

    pub fn build_mapping(&self) -> i32 {
        unsafe { ffi::nk_user2internal_index_build_mapping(self.handle.as_ptr()) }
    }

    pub fn user_to_internal(&self, user: i32) -> Option<i32> {
        let mut internal: i32 = -1;
        let result = unsafe {
            ffi::nk_user2internal_index_user2internal(self.handle.as_ptr(), user, &mut internal)
        };
        if result == 0 { Some(internal) } else { None }
    }

    pub fn internal_to_user(&self, internal: i32) -> Option<i32> {
        let mut user: i32 = -1;
        let result = unsafe {
            ffi::nk_user2internal_index_internal2user(self.handle.as_ptr(), internal, &mut user)
        };
        if result == 0 { Some(user) } else { None }
    }
}

impl Default for User2InternalIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for User2InternalIndex {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_user2internal_index_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for User2InternalIndex {}
unsafe impl Sync for User2InternalIndex {}

pub mod random {
    use super::ffi;
    use crate::core::types::Real;

    pub fn init() {
        unsafe { ffi::nk_id_random_init() }
    }

    pub fn init_with_seed(seed: u32) {
        unsafe { ffi::nk_id_random_init_with_seed(seed) }
    }

    pub fn random_int(low: i32, high: i32) -> i32 {
        unsafe { ffi::nk_id_random_int(low, high) }
    }

    pub fn random_float(low: Real, high: Real) -> Real {
        unsafe { ffi::nk_id_random_float(low, high) }
    }

    pub fn random_mass() -> Real {
        unsafe { ffi::nk_id_random_mass() }
    }

    pub fn random_inertia_principal() -> (Real, Real, Real) {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_id_random_inertia_principal(&mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    pub fn random_axis() -> (Real, Real, Real) {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_id_random_axis(&mut x, &mut y, &mut z);
        }
        (x, y, z)
    }
}
