use std::ffi::c_void;
use std::ptr::NonNull;

use super::types::{Real, Vec3};
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum JointType {
    Fixed = 0,
    Revolute = 1,
    Prismatic = 2,
    Floating = 3,
    Spherical = 4,
}

pub struct MultiBody {
    handle: NonNull<c_void>,
    finalized: bool,
    owned: bool,
}

impl MultiBody {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_multibody_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create multibody"),
            finalized: false,
            owned: true,
        }
    }

    pub(crate) fn from_handle(handle: NonNull<c_void>) -> Self {
        Self {
            handle,
            finalized: true,
            owned: true,
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_body(
        &mut self,
        body_index: i32,
        parent_index: i32,
        joint_type: JointType,
        parent_r: Vec3,
        body_t_parent: [[Real; 3]; 3],
        axis: Vec3,
        mass: Real,
        com: Vec3,
        inertia: [[Real; 3]; 3],
    ) -> i32 {
        unsafe {
            ffi::nk_multibody_add_body(
                self.handle.as_ptr(),
                body_index,
                parent_index,
                joint_type as i32,
                parent_r.x, parent_r.y, parent_r.z,
                body_t_parent[0][0], body_t_parent[0][1], body_t_parent[0][2],
                body_t_parent[1][0], body_t_parent[1][1], body_t_parent[1][2],
                body_t_parent[2][0], body_t_parent[2][1], body_t_parent[2][2],
                axis.x, axis.y, axis.z,
                mass,
                com.x, com.y, com.z,
                inertia[0][0], inertia[0][1], inertia[0][2],
                inertia[1][1], inertia[1][2], inertia[2][2],
            )
        }
    }

    pub fn finalize(&mut self) -> i32 {
        let result = unsafe { ffi::nk_multibody_finalize(self.handle.as_ptr()) };
        if result == 0 {
            self.finalized = true;
        }
        result
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized
    }

    pub fn calculate_inverse_dynamics(
        &self,
        q: &[Real],
        u: &[Real],
        dot_u: &[Real],
        joint_forces: &mut [Real],
    ) -> i32 {
        let num_dofs = joint_forces.len() as i32;
        unsafe {
            ffi::nk_multibody_calculate_inverse_dynamics(
                self.handle.as_ptr(),
                q.as_ptr(),
                u.as_ptr(),
                dot_u.as_ptr(),
                joint_forces.as_mut_ptr(),
                num_dofs,
            )
        }
    }

    pub fn calculate_mass_matrix(
        &self,
        q: &[Real],
        mass_matrix: &mut [Real],
        initialize: bool,
        set_lower_triangular: bool,
    ) -> i32 {
        let num_q = q.len() as i32;
        unsafe {
            ffi::nk_multibody_calculate_mass_matrix(
                self.handle.as_ptr(),
                q.as_ptr(),
                num_q,
                mass_matrix.as_mut_ptr(),
                if initialize { 1 } else { 0 },
                if set_lower_triangular { 1 } else { 0 },
            )
        }
    }

    pub fn num_bodies(&self) -> i32 {
        unsafe { ffi::nk_multibody_get_num_bodies(self.handle.as_ptr()) }
    }

    pub fn num_dofs(&self) -> i32 {
        unsafe { ffi::nk_multibody_get_num_dofs(self.handle.as_ptr()) }
    }

    pub fn set_gravity(&self, gravity: Vec3) {
        unsafe {
            ffi::nk_multibody_set_gravity(self.handle.as_ptr(), gravity.x, gravity.y, gravity.z);
        }
    }

    pub fn set_accept_invalid_mass(&self, accept: bool) {
        unsafe {
            ffi::nk_multibody_set_accept_invalid_mass(self.handle.as_ptr(), if accept { 1 } else { 0 });
        }
    }

    pub fn print_tree(&self) {
        unsafe {
            ffi::nk_multibody_print_tree(self.handle.as_ptr());
        }
    }
}

impl Default for MultiBody {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MultiBody {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::nk_multibody_destroy(self.handle.as_ptr());
            }
        }
    }
}

unsafe impl Send for MultiBody {}
unsafe impl Sync for MultiBody {}
