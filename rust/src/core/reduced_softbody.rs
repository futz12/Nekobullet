use std::ffi::{c_void, CString};
use std::ptr::NonNull;

use super::softbody::SoftBodyWorldInfo;
use super::types::{Real, Transform, Vec3};
use crate::ffi;

pub struct ReducedDeformableBody {
    handle: NonNull<c_void>,
}

impl ReducedDeformableBody {
    pub fn new(
        world_info: &SoftBodyWorldInfo,
        positions: &[[Real; 3]],
        masses: &[Real],
    ) -> Option<Self> {
        if positions.len() != masses.len() || positions.is_empty() {
            return None;
        }

        let handle = unsafe {
            ffi::nk_reduced_softbody_create(
                world_info.handle(),
                positions.len() as i32,
                positions.as_ptr() as *const Real,
                masses.as_ptr(),
            )
        };

        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn from_vtk(world_info: &SoftBodyWorldInfo, vtk_path: &str) -> Option<Self> {
        let c_path = CString::new(vtk_path).ok()?;
        let handle = unsafe {
            ffi::nk_reduced_softbody_create_from_vtk(world_info.handle(), c_path.as_ptr())
        };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn set_reduced_modes(&self, num_modes: i32, full_size: i32) {
        unsafe {
            ffi::nk_reduced_softbody_set_reduced_modes(self.handle.as_ptr(), num_modes, full_size);
        }
    }

    pub fn num_reduced_modes(&self) -> i32 {
        unsafe { ffi::nk_reduced_softbody_get_num_reduced_modes(self.handle.as_ptr()) }
    }

    pub fn num_full_dofs(&self) -> i32 {
        unsafe { ffi::nk_reduced_softbody_get_num_full_dofs(self.handle.as_ptr()) }
    }

    pub fn set_stiffness_scale(&self, ks: Real) {
        unsafe {
            ffi::nk_reduced_softbody_set_stiffness_scale(self.handle.as_ptr(), ks);
        }
    }

    pub fn set_mass_scale(&self, rho: Real) {
        unsafe {
            ffi::nk_reduced_softbody_set_mass_scale(self.handle.as_ptr(), rho);
        }
    }

    pub fn set_damping(&self, alpha: Real, beta: Real) {
        unsafe {
            ffi::nk_reduced_softbody_set_damping(self.handle.as_ptr(), alpha, beta);
        }
    }

    pub fn set_fixed_node(&self, node_index: i32) {
        unsafe {
            ffi::nk_reduced_softbody_set_fixed_node(self.handle.as_ptr(), node_index);
        }
    }

    pub fn disable_reduced_modes(&self, rigid_only: bool) {
        unsafe {
            ffi::nk_reduced_softbody_disable_reduced_modes(
                self.handle.as_ptr(),
                if rigid_only { 1 } else { 0 },
            );
        }
    }

    pub fn set_rigid_velocity(&self, v: Vec3) {
        unsafe {
            ffi::nk_reduced_softbody_set_rigid_velocity(self.handle.as_ptr(), v.x, v.y, v.z);
        }
    }

    pub fn set_rigid_angular_velocity(&self, w: Vec3) {
        unsafe {
            ffi::nk_reduced_softbody_set_rigid_angular_velocity(self.handle.as_ptr(), w.x, w.y, w.z);
        }
    }

    pub fn rigid_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_reduced_softbody_get_rigid_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn rigid_angular_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_reduced_softbody_get_rigid_angular_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn total_mass(&self) -> Real {
        unsafe { ffi::nk_reduced_softbody_get_total_mass(self.handle.as_ptr()) }
    }

    pub fn set_total_mass(&self, mass: Real) {
        unsafe {
            ffi::nk_reduced_softbody_set_total_mass(self.handle.as_ptr(), mass);
        }
    }

    pub fn rigid_transform(&self) -> Transform {
        let mut transform: ffi::nkTransform = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::nk_reduced_softbody_get_rigid_transform(self.handle.as_ptr(), &mut transform);
        }
        Transform {
            position: Vec3::new(transform.origin.data[0], transform.origin.data[1], transform.origin.data[2]),
            rotation: glam::Quat::from_xyzw(
                transform.rotation.data[0],
                transform.rotation.data[1],
                transform.rotation.data[2],
                transform.rotation.data[3],
            ),
        }
    }

    pub fn set_rigid_transform(&self, transform: &Transform) {
        let nk_transform = ffi::nkTransform {
            origin: ffi::nkVector3 {
                data: [transform.position.x, transform.position.y, transform.position.z],
            },
            rotation: ffi::nkQuaternion {
                data: [transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w],
            },
        };
        unsafe {
            ffi::nk_reduced_softbody_set_rigid_transform(self.handle.as_ptr(), &nk_transform as *const _ as *mut _);
        }
    }

    pub fn apply_central_impulse(&self, impulse: Vec3) {
        unsafe {
            ffi::nk_reduced_softbody_apply_central_impulse(self.handle.as_ptr(), impulse.x, impulse.y, impulse.z);
        }
    }

    pub fn apply_torque_impulse(&self, torque: Vec3) {
        unsafe {
            ffi::nk_reduced_softbody_apply_torque_impulse(self.handle.as_ptr(), torque.x, torque.y, torque.z);
        }
    }

    pub fn num_nodes(&self) -> i32 {
        unsafe { ffi::nk_reduced_softbody_get_num_nodes(self.handle.as_ptr()) }
    }

    pub fn node_position(&self, index: i32) -> Option<Vec3> {
        if index < 0 || index >= self.num_nodes() {
            return None;
        }
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_reduced_softbody_get_node_position(self.handle.as_ptr(), index, &mut x, &mut y, &mut z);
        }
        Some(Vec3::new(x, y, z))
    }

    pub fn node_rest_position(&self, index: i32) -> Option<Vec3> {
        if index < 0 || index >= self.num_nodes() {
            return None;
        }
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_reduced_softbody_get_node_rest_position(self.handle.as_ptr(), index, &mut x, &mut y, &mut z);
        }
        Some(Vec3::new(x, y, z))
    }

    pub fn read_reduced_info(&self, file_path: &str) {
        if let Ok(c_path) = CString::new(file_path) {
            unsafe {
                ffi::nk_reduced_softbody_read_reduced_info(self.handle.as_ptr(), c_path.as_ptr());
            }
        }
    }
}

impl Drop for ReducedDeformableBody {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_reduced_softbody_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for ReducedDeformableBody {}
unsafe impl Sync for ReducedDeformableBody {}

#[cfg(test)]
mod tests_reduced_softbody {
    use super::*;
    use crate::core::world::PhysicsWorldBuilder;

    fn create_world() -> crate::core::world::PhysicsWorld {
        PhysicsWorldBuilder::new().build()
    }

    #[test]
    fn test_reduced_softbody_create() {
        let world = create_world();
        let info = world.softbody_world_info();
        
        let positions = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let masses = [1.0, 1.0, 1.0, 1.0];
        
        let body = ReducedDeformableBody::new(&info, &positions, &masses);
        assert!(body.is_some());
        
        let body = body.unwrap();
        assert_eq!(body.num_nodes(), 4);
    }

    #[test]
    fn test_reduced_softbody_modes() {
        let world = create_world();
        let info = world.softbody_world_info();
        
        let positions = [[0.0, 0.0, 0.0]; 4];
        let masses = [1.0; 4];
        
        let body = ReducedDeformableBody::new(&info, &positions, &masses).unwrap();
        
        body.set_reduced_modes(3, 12);
        assert_eq!(body.num_reduced_modes(), 3);
        assert_eq!(body.num_full_dofs(), 12);
    }

    #[test]
    fn test_reduced_softbody_velocity() {
        let world = create_world();
        let info = world.softbody_world_info();
        
        let positions = [[0.0, 0.0, 0.0]; 4];
        let masses = [1.0; 4];
        
        let body = ReducedDeformableBody::new(&info, &positions, &masses).unwrap();
        
        body.set_rigid_velocity(Vec3::new(1.0, 2.0, 3.0));
        let v = body.rigid_velocity();
        assert!((v.x - 1.0).abs() < 0.001);
        assert!((v.y - 2.0).abs() < 0.001);
        assert!((v.z - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_reduced_softbody_mass() {
        let world = create_world();
        let info = world.softbody_world_info();
        
        let positions = [[0.0, 0.0, 0.0]; 4];
        let masses = [1.0; 4];
        
        let body = ReducedDeformableBody::new(&info, &positions, &masses).unwrap();
        
        body.set_total_mass(10.0);
        assert!((body.total_mass() - 10.0).abs() < 0.001);
    }
}
