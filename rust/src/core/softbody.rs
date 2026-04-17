use std::ffi::c_void;
use std::ptr::NonNull;

use super::rigidbody::RigidBody;
use super::types::{Real, Vec3};
use crate::ffi;

pub struct SoftBodyWorldInfo {
    handle: NonNull<c_void>,
}

impl SoftBodyWorldInfo {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_softbody_world_info_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create soft body world info"),
        }
    }

    pub fn set_gravity(&self, gravity: Vec3) {
        unsafe {
            ffi::nk_softbody_world_info_set_gravity(self.handle.as_ptr(), gravity.x, gravity.y, gravity.z);
        }
    }

    pub fn set_air_density(&self, density: Real) {
        unsafe {
            ffi::nk_softbody_world_info_set_air_density(self.handle.as_ptr(), density);
        }
    }

    pub fn set_water_density(&self, density: Real) {
        unsafe {
            ffi::nk_softbody_world_info_set_water_density(self.handle.as_ptr(), density);
        }
    }

    pub fn set_water_offset(&self, offset: Real) {
        unsafe {
            ffi::nk_softbody_world_info_set_water_offset(self.handle.as_ptr(), offset);
        }
    }

    pub fn set_water_normal(&self, normal: Vec3) {
        unsafe {
            ffi::nk_softbody_world_info_set_water_normal(self.handle.as_ptr(), normal.x, normal.y, normal.z);
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Default for SoftBodyWorldInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SoftBodyWorldInfo {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_softbody_world_info_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for SoftBodyWorldInfo {}
unsafe impl Sync for SoftBodyWorldInfo {}

pub struct SoftBody {
    handle: NonNull<c_void>,
}

impl SoftBody {
    pub fn create_rope(world_info: &SoftBodyWorldInfo, from: Vec3, to: Vec3, resolution: i32, fixeds: i32) -> Self {
        let handle = unsafe {
            ffi::nk_softbody_create_rope(
                world_info.handle(),
                from.x, from.y, from.z,
                to.x, to.y, to.z,
                resolution, fixeds,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create rope soft body"),
        }
    }

    pub fn create_patch(world_info: &SoftBodyWorldInfo, corners: [Vec3; 4], resx: i32, resy: i32, fixeds: i32, gendiags: bool) -> Self {
        let handle = unsafe {
            ffi::nk_softbody_create_patch(
                world_info.handle(),
                corners[0].x, corners[0].y, corners[0].z,
                corners[1].x, corners[1].y, corners[1].z,
                corners[2].x, corners[2].y, corners[2].z,
                corners[3].x, corners[3].y, corners[3].z,
                resx, resy, fixeds, if gendiags { 1 } else { 0 },
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create patch soft body"),
        }
    }

    pub fn create_ellipsoid(world_info: &SoftBodyWorldInfo, center: Vec3, radius: Vec3, resolution: i32) -> Self {
        let handle = unsafe {
            ffi::nk_softbody_create_ellipsoid(
                world_info.handle(),
                center.x, center.y, center.z,
                radius.x, radius.y, radius.z,
                resolution,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create ellipsoid soft body"),
        }
    }

    pub fn create_from_trimesh(world_info: &SoftBodyWorldInfo, vertices: &[Real], triangles: &[i32]) -> Self {
        let handle = unsafe {
            ffi::nk_softbody_create_from_trimesh(
                world_info.handle(),
                vertices.as_ptr(),
                triangles.as_ptr(),
                (triangles.len() / 3) as i32,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create trimesh soft body"),
        }
    }

    pub fn create_from_convex_hull(world_info: &SoftBodyWorldInfo, vertices: &[Real]) -> Self {
        let handle = unsafe {
            ffi::nk_softbody_create_from_convex_hull(
                world_info.handle(),
                vertices.as_ptr(),
                (vertices.len() / 3) as i32,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create convex hull soft body"),
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn num_nodes(&self) -> i32 {
        unsafe { ffi::nk_softbody_get_num_nodes(self.handle.as_ptr()) }
    }

    pub fn get_node_position(&self, index: i32) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_softbody_get_node_position(self.handle.as_ptr(), index, &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_node_position(&self, index: i32, position: Vec3) {
        unsafe {
            ffi::nk_softbody_set_node_position(self.handle.as_ptr(), index, position.x, position.y, position.z);
        }
    }

    pub fn get_node_velocity(&self, index: i32) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_softbody_get_node_velocity(self.handle.as_ptr(), index, &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_node_velocity(&self, index: i32, velocity: Vec3) {
        unsafe {
            ffi::nk_softbody_set_node_velocity(self.handle.as_ptr(), index, velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn get_node_mass(&self, index: i32) -> Real {
        unsafe { ffi::nk_softbody_get_node_mass(self.handle.as_ptr(), index) }
    }

    pub fn set_node_mass(&self, index: i32, mass: Real) {
        unsafe {
            ffi::nk_softbody_set_node_mass(self.handle.as_ptr(), index, mass);
        }
    }

    pub fn total_mass(&self) -> Real {
        unsafe { ffi::nk_softbody_get_total_mass(self.handle.as_ptr()) }
    }

    pub fn set_total_mass(&self, mass: Real) {
        unsafe {
            ffi::nk_softbody_set_total_mass(self.handle.as_ptr(), mass);
        }
    }

    pub fn set_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_softbody_set_velocity(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn add_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_softbody_add_velocity(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn apply_force(&self, force: Vec3) {
        unsafe {
            ffi::nk_softbody_apply_force(self.handle.as_ptr(), force.x, force.y, force.z);
        }
    }

    pub fn apply_impulse(&self, impulse: Vec3) {
        unsafe {
            ffi::nk_softbody_apply_impulse(self.handle.as_ptr(), impulse.x, impulse.y, impulse.z);
        }
    }

    pub fn clear_forces(&self) {
        unsafe {
            ffi::nk_softbody_clear_forces(self.handle.as_ptr());
        }
    }

    pub fn append_anchor(&self, node_index: i32, body: &RigidBody, local: Vec3, disable_collision: bool) {
        unsafe {
            ffi::nk_softbody_append_anchor(
                self.handle.as_ptr(),
                node_index,
                body.handle(),
                local.x, local.y, local.z,
                if disable_collision { 1 } else { 0 },
            );
        }
    }

    pub fn remove_anchor(&self, node_index: i32) {
        unsafe {
            ffi::nk_softbody_remove_anchor(self.handle.as_ptr(), node_index);
        }
    }

    pub fn set_material_stiffness(&self, k_lst: Real, k_ast: Real, k_vst: Real) {
        unsafe {
            ffi::nk_softbody_set_material_stiffness(self.handle.as_ptr(), k_lst, k_ast, k_vst);
        }
    }

    pub fn set_wind_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_softbody_set_wind_velocity(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn wind_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_softbody_get_wind_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_config_damping(&self, damping: Real) {
        unsafe {
            ffi::nk_softbody_set_config_damping(self.handle.as_ptr(), damping);
        }
    }

    pub fn set_config_drag(&self, drag: Real) {
        unsafe {
            ffi::nk_softbody_set_config_drag(self.handle.as_ptr(), drag);
        }
    }

    pub fn set_config_lift(&self, lift: Real) {
        unsafe {
            ffi::nk_softbody_set_config_lift(self.handle.as_ptr(), lift);
        }
    }

    pub fn set_config_pressure(&self, pressure: Real) {
        unsafe {
            ffi::nk_softbody_set_config_pressure(self.handle.as_ptr(), pressure);
        }
    }

    pub fn set_config_volume_conversation(&self, volume: Real) {
        unsafe {
            ffi::nk_softbody_set_config_volume_conversation(self.handle.as_ptr(), volume);
        }
    }

    pub fn set_config_time_scale(&self, scale: Real) {
        unsafe {
            ffi::nk_softbody_set_config_time_scale(self.handle.as_ptr(), scale);
        }
    }
}

impl Drop for SoftBody {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_softbody_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for SoftBody {}
unsafe impl Sync for SoftBody {}
