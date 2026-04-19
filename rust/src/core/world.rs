use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use super::constraint::{Constraint, ConstraintBuilder};
use super::ghost::GhostObject;
use super::rigidbody::{RigidBody, RigidBodyBuilder};
use super::types::{Real, Transform, Vec3};
use crate::ffi::{self, nkTransform};

pub type RigidBodyHandle = u64;
pub type ConstraintHandle = u64;
pub type GhostHandle = u64;

pub type ContactCallback = Box<dyn FnMut(&ContactPoint)>;
pub type CollisionFilterCallback = Box<dyn Fn(RigidBodyHandle, RigidBodyHandle) -> bool>;

struct CallbackStorage {
    contact_callback: RefCell<Option<ContactCallback>>,
    collision_filter: RefCell<Option<CollisionFilterCallback>>,
}

pub struct PhysicsWorld {
    handle: NonNull<c_void>,
    bodies: BTreeMap<RigidBodyHandle, RigidBody>,
    constraints: BTreeMap<ConstraintHandle, NonNull<c_void>>,
    ghosts: BTreeMap<GhostHandle, GhostObject>,
    next_body_handle: Cell<RigidBodyHandle>,
    next_constraint_handle: Cell<ConstraintHandle>,
    next_ghost_handle: Cell<GhostHandle>,
    callbacks: Box<CallbackStorage>,
    _marker: PhantomData<*mut ()>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_world_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create physics world"),
            bodies: BTreeMap::new(),
            constraints: BTreeMap::new(),
            ghosts: BTreeMap::new(),
            next_body_handle: Cell::new(1),
            next_constraint_handle: Cell::new(1),
            next_ghost_handle: Cell::new(1),
            callbacks: Box::new(CallbackStorage {
                contact_callback: RefCell::new(None),
                collision_filter: RefCell::new(None),
            }),
            _marker: PhantomData,
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn set_gravity(&self, gravity: Vec3) {
        unsafe {
            ffi::nk_world_set_gravity(self.handle.as_ptr(), gravity.x, gravity.y, gravity.z);
        }
    }

    pub fn get_gravity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_world_get_gravity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn step_simulation(&self, time_step: Real, max_sub_steps: i32, fixed_time_step: Real) {
        unsafe {
            ffi::nk_world_step_simulation(
                self.handle.as_ptr(),
                time_step,
                max_sub_steps,
                fixed_time_step,
            );
        }
    }

    pub fn step(&self, time_step: Real) {
        self.step_simulation(time_step, 1, 1.0 / 60.0);
    }

    pub fn add_rigid_body(&mut self, body: RigidBody) -> RigidBodyHandle {
        let handle = self.next_body_handle.get();
        self.next_body_handle.set(handle + 1);

        unsafe {
            ffi::nk_world_add_rigid_body(self.handle.as_ptr(), body.handle());
        }

        self.bodies.insert(handle, body);
        handle
    }

    pub fn add_rigid_body_with_filter(&mut self, body: RigidBody, group: i32, mask: i32) -> RigidBodyHandle {
        let handle = self.next_body_handle.get();
        self.next_body_handle.set(handle + 1);

        unsafe {
            ffi::nk_world_add_rigid_body_with_filter(self.handle.as_ptr(), body.handle(), group, mask);
        }

        self.bodies.insert(handle, body);
        handle
    }

    pub fn create_rigid_body(&mut self, builder: RigidBodyBuilder) -> Result<RigidBodyHandle, &'static str> {
        let body = builder.build()?;
        Ok(self.add_rigid_body(body))
    }

    pub fn remove_rigid_body(&mut self, handle: RigidBodyHandle) -> Option<RigidBody> {
        if let Some(body) = self.bodies.remove(&handle) {
            unsafe {
                ffi::nk_world_remove_rigid_body(self.handle.as_ptr(), body.handle());
            }
            Some(body)
        } else {
            None
        }
    }

    pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.bodies.get(&handle)
    }

    pub fn get_rigid_body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.bodies.get_mut(&handle)
    }

    pub fn bodies(&self) -> impl Iterator<Item = (&RigidBodyHandle, &RigidBody)> {
        self.bodies.iter()
    }

    pub fn bodies_mut(&mut self) -> impl Iterator<Item = (&RigidBodyHandle, &mut RigidBody)> {
        self.bodies.iter_mut()
    }

    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn clear_forces(&self) {
        unsafe {
            ffi::nk_world_clear_forces(self.handle.as_ptr());
        }
    }

    pub fn get_num_collision_objects(&self) -> i32 {
        unsafe { ffi::nk_world_get_num_collision_objects(self.handle.as_ptr()) }
    }

    pub fn get_num_rigid_bodies(&self) -> i32 {
        unsafe { ffi::nk_world_get_num_rigid_bodies(self.handle.as_ptr()) }
    }

    pub fn get_num_constraints(&self) -> i32 {
        unsafe { ffi::nk_world_get_num_constraints(self.handle.as_ptr()) }
    }

    pub fn set_time_step(&self, time_step: Real) {
        unsafe {
            ffi::nk_world_set_time_step(self.handle.as_ptr(), time_step);
        }
    }

    pub fn get_time_step(&self) -> Real {
        unsafe { ffi::nk_world_get_time_step(self.handle.as_ptr()) }
    }

    pub fn set_max_sub_steps(&self, max_sub_steps: i32) {
        unsafe {
            ffi::nk_world_set_max_sub_steps(self.handle.as_ptr(), max_sub_steps);
        }
    }

    pub fn get_max_sub_steps(&self) -> i32 {
        unsafe { ffi::nk_world_get_max_sub_steps(self.handle.as_ptr()) }
    }

    pub fn set_contact_breaking_threshold(&self, threshold: Real) {
        unsafe {
            ffi::nk_world_set_contact_breaking_threshold(self.handle.as_ptr(), threshold);
        }
    }

    pub fn get_contact_breaking_threshold(&self) -> Real {
        unsafe { ffi::nk_world_get_contact_breaking_threshold(self.handle.as_ptr()) }
    }

    pub fn ray_test_closest(&self, from: Vec3, to: Vec3) -> Option<RayTestResult> {
        let mut result: ffi::nkRayTestResult = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::nk_world_ray_test_closest(
                self.handle.as_ptr(),
                from.x, from.y, from.z,
                to.x, to.y, to.z,
                &mut result,
            );
        }
        
        if result.hit != 0 {
            Some(RayTestResult {
                hit_point: Vec3::new(result.hit_point[0], result.hit_point[1], result.hit_point[2]),
                hit_normal: Vec3::new(result.hit_normal[0], result.hit_normal[1], result.hit_normal[2]),
                hit_fraction: result.hit_fraction,
                body_ptr: result.body,
            })
        } else {
            None
        }
    }

    pub fn ray_test_all(&self, from: Vec3, to: Vec3, max_results: usize) -> Vec<RayTestResult> {
        let mut results: Vec<ffi::nkRayTestResult> = Vec::with_capacity(max_results);
        let mut num_results: i32 = 0;
        
        unsafe {
            ffi::nk_world_ray_test_all(
                self.handle.as_ptr(),
                from.x, from.y, from.z,
                to.x, to.y, to.z,
                results.as_mut_ptr(),
                max_results as i32,
                &mut num_results,
            );
            results.set_len(num_results as usize);
        }
        
        results.into_iter().filter_map(|r| {
            if r.hit != 0 {
                Some(RayTestResult {
                    hit_point: Vec3::new(r.hit_point[0], r.hit_point[1], r.hit_point[2]),
                    hit_normal: Vec3::new(r.hit_normal[0], r.hit_normal[1], r.hit_normal[2]),
                    hit_fraction: r.hit_fraction,
                    body_ptr: r.body,
                })
            } else {
                None
            }
        }).collect()
    }

    pub fn create_point2point_constraint(
        &mut self,
        body_a_handle: RigidBodyHandle,
        body_b_handle: RigidBodyHandle,
        pivot_a: Vec3,
        pivot_b: Vec3,
    ) -> Option<ConstraintHandle> {
        let body_a = self.bodies.get(&body_a_handle)?;
        let body_b = self.bodies.get(&body_b_handle)?;
        
        let constraint = unsafe {
            ffi::nk_constraint_create_point2point(
                body_a.handle(),
                body_b.handle(),
                pivot_a.x, pivot_a.y, pivot_a.z,
                pivot_b.x, pivot_b.y, pivot_b.z,
            )
        };
        
        if constraint.is_null() {
            return None;
        }
        
        let handle = self.next_constraint_handle.get();
        self.next_constraint_handle.set(handle + 1);
        
        unsafe {
            ffi::nk_world_add_constraint(self.handle.as_ptr(), constraint, 1);
        }
        
        self.constraints.insert(handle, unsafe { NonNull::new_unchecked(constraint) });
        Some(handle)
    }

    pub fn create_fixed_constraint(
        &mut self,
        body_a_handle: RigidBodyHandle,
        body_b_handle: RigidBodyHandle,
    ) -> Option<ConstraintHandle> {
        let body_a = self.bodies.get(&body_a_handle)?;
        let body_b = self.bodies.get(&body_b_handle)?;

        let constraint = unsafe {
            ffi::nk_constraint_create_fixed(
                body_a.handle(),
                body_b.handle(),
            )
        };

        if constraint.is_null() {
            return None;
        }

        let handle = self.next_constraint_handle.get();
        self.next_constraint_handle.set(handle + 1);

        unsafe {
            ffi::nk_world_add_constraint(self.handle.as_ptr(), constraint, 1);
        }

        self.constraints.insert(handle, unsafe { NonNull::new_unchecked(constraint) });
        Some(handle)
    }

    pub fn create_generic_6dof_constraint(
        &mut self,
        body_a_handle: RigidBodyHandle,
        body_b_handle: RigidBodyHandle,
        frame_a: Transform,
        frame_b: Transform,
        use_linear_reference_frame_a: bool,
    ) -> Option<ConstraintHandle> {
        let body_a = self.bodies.get(&body_a_handle)?;
        let body_b = self.bodies.get(&body_b_handle)?;
        
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let constraint = unsafe {
            ffi::nk_constraint_create_generic_6dof(
                body_a.handle(),
                body_b.handle(),
                &nk_frame_a,
                &nk_frame_b,
                if use_linear_reference_frame_a { 1 } else { 0 },
            )
        };
        
        if constraint.is_null() {
            return None;
        }
        
        let handle = self.next_constraint_handle.get();
        self.next_constraint_handle.set(handle + 1);
        
        unsafe {
            ffi::nk_world_add_constraint(self.handle.as_ptr(), constraint, 1);
        }
        
        self.constraints.insert(handle, unsafe { NonNull::new_unchecked(constraint) });
        Some(handle)
    }

    pub fn create_cone_twist_constraint(
        &mut self,
        body_a_handle: RigidBodyHandle,
        body_b_handle: RigidBodyHandle,
        frame_a: Transform,
        frame_b: Transform,
    ) -> Option<ConstraintHandle> {
        let body_a = self.bodies.get(&body_a_handle)?;
        let body_b = self.bodies.get(&body_b_handle)?;
        
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let constraint = unsafe {
            ffi::nk_constraint_create_cone_twist(
                body_a.handle(),
                body_b.handle(),
                &nk_frame_a,
                &nk_frame_b,
            )
        };
        
        if constraint.is_null() {
            return None;
        }
        
        let handle = self.next_constraint_handle.get();
        self.next_constraint_handle.set(handle + 1);
        
        unsafe {
            ffi::nk_world_add_constraint(self.handle.as_ptr(), constraint, 1);
        }
        
        self.constraints.insert(handle, unsafe { NonNull::new_unchecked(constraint) });
        Some(handle)
    }

    pub fn create_constraint(&mut self, builder: ConstraintBuilder) -> Option<ConstraintHandle> {
        let constraint = builder.build().ok()?;

        let handle = self.next_constraint_handle.get();
        self.next_constraint_handle.set(handle + 1);

        unsafe {
            ffi::nk_world_add_constraint(self.handle.as_ptr(), constraint, 1);
        }

        self.constraints.insert(handle, unsafe { NonNull::new_unchecked(constraint) });
        Some(handle)
    }

    pub fn get_constraint(&self, handle: ConstraintHandle) -> Option<Constraint> {
        self.constraints.get(&handle).map(|ptr| unsafe {
            Constraint::from_raw(ptr.as_ptr(), crate::core::constraint::ConstraintType::Generic6DofSpring)
        })
    }

    pub fn add_ghost(&mut self, ghost: GhostObject) -> GhostHandle {
        let handle = self.next_ghost_handle.get();
        self.next_ghost_handle.set(handle + 1);
        
        unsafe {
            ffi::nk_world_add_ghost(self.handle.as_ptr(), ghost.handle());
        }
        
        self.ghosts.insert(handle, ghost);
        handle
    }

    pub fn remove_ghost(&mut self, handle: GhostHandle) -> Option<GhostObject> {
        if let Some(ghost) = self.ghosts.remove(&handle) {
            unsafe {
                ffi::nk_world_remove_ghost(self.handle.as_ptr(), ghost.handle());
            }
            Some(ghost)
        } else {
            None
        }
    }

    pub fn get_ghost(&self, handle: GhostHandle) -> Option<&GhostObject> {
        self.ghosts.get(&handle)
    }

    pub fn get_ghost_mut(&mut self, handle: GhostHandle) -> Option<&mut GhostObject> {
        self.ghosts.get_mut(&handle)
    }

    pub fn remove_constraint(&mut self, handle: ConstraintHandle) -> bool {
        if let Some(constraint) = self.constraints.remove(&handle) {
            unsafe {
                ffi::nk_world_remove_constraint(self.handle.as_ptr(), constraint.as_ptr());
                ffi::nk_constraint_destroy(self.handle.as_ptr(), constraint.as_ptr());
            }
            true
        } else {
            false
        }
    }

    pub fn set_contact_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&ContactPoint) + 'static,
    {
        *self.callbacks.contact_callback.borrow_mut() = Some(Box::new(callback));
        
        extern "C" fn contact_trampoline(contact: *mut ffi::nkContactPoint, user_data: *mut c_void) {
            unsafe {
                let world = &*(user_data as *const PhysicsWorld);
                if let Some(ref mut callback) = *world.callbacks.contact_callback.borrow_mut() {
                    let contact_point = ContactPoint {
                        position: Vec3::new((*contact).position[0], (*contact).position[1], (*contact).position[2]),
                        normal: Vec3::new((*contact).normal[0], (*contact).normal[1], (*contact).normal[2]),
                        distance: (*contact).distance,
                        body_a_ptr: (*contact).body_a,
                        body_b_ptr: (*contact).body_b,
                    };
                    callback(&contact_point);
                }
            }
        }
        
        unsafe {
            ffi::nk_world_set_contact_callback(
                self.handle.as_ptr(),
                Some(contact_trampoline),
                self as *const Self as *mut c_void,
            );
        }
    }

    pub fn clear_contact_callback(&mut self) {
        *self.callbacks.contact_callback.borrow_mut() = None;
        unsafe {
            ffi::nk_world_set_contact_callback(
                self.handle.as_ptr(),
                None,
                std::ptr::null_mut(),
            );
        }
    }

    pub fn set_collision_filter<F>(&mut self, filter: F)
    where
        F: Fn(RigidBodyHandle, RigidBodyHandle) -> bool + 'static,
    {
        *self.callbacks.collision_filter.borrow_mut() = Some(Box::new(filter));
        
        extern "C" fn filter_trampoline(
            body_a: ffi::nkRigidBodyHandle,
            body_b: ffi::nkRigidBodyHandle,
            user_data: *mut c_void,
        ) -> i32 {
            unsafe {
                let world = &*(user_data as *const PhysicsWorld);
                if let Some(ref filter) = *world.callbacks.collision_filter.borrow() {
                    let handle_a = world.find_body_handle(body_a);
                    let handle_b = world.find_body_handle(body_b);
                    if let (Some(a), Some(b)) = (handle_a, handle_b) {
                        return if filter(a, b) { 1 } else { 0 };
                    }
                }
                1
            }
        }
        
        unsafe {
            ffi::nk_world_set_collision_filter(
                self.handle.as_ptr(),
                Some(filter_trampoline),
                self as *const Self as *mut c_void,
            );
        }
    }

    pub fn clear_collision_filter(&mut self) {
        *self.callbacks.collision_filter.borrow_mut() = None;
        unsafe {
            ffi::nk_world_set_collision_filter(
                self.handle.as_ptr(),
                None,
                std::ptr::null_mut(),
            );
        }
    }

    pub fn set_solver_iterations(&self, iterations: i32) {
        unsafe {
            ffi::nk_world_set_solver_iterations(self.handle.as_ptr(), iterations);
        }
    }

    pub fn get_solver_iterations(&self) -> i32 {
        unsafe { ffi::nk_world_get_solver_iterations(self.handle.as_ptr()) }
    }

    pub fn set_erp(&self, erp: Real) {
        unsafe {
            ffi::nk_world_set_erp(self.handle.as_ptr(), erp);
        }
    }

    pub fn get_erp(&self) -> Real {
        unsafe { ffi::nk_world_get_erp(self.handle.as_ptr()) }
    }

    pub fn set_erp2(&self, erp2: Real) {
        unsafe {
            ffi::nk_world_set_erp2(self.handle.as_ptr(), erp2);
        }
    }

    pub fn get_erp2(&self) -> Real {
        unsafe { ffi::nk_world_get_erp2(self.handle.as_ptr()) }
    }

    fn find_body_handle(&self, body_ptr: *mut c_void) -> Option<RigidBodyHandle> {
        for (handle, body) in self.bodies.iter() {
            if body.handle() == body_ptr {
                return Some(*handle);
            }
        }
        None
    }

    #[cfg(feature = "softbody")]
    pub fn add_softbody(&self, softbody: &super::softbody::SoftBody) {
        unsafe {
            ffi::nk_world_add_softbody(self.handle.as_ptr(), softbody.handle());
        }
    }

    #[cfg(feature = "softbody")]
    pub fn remove_softbody(&self, softbody: &super::softbody::SoftBody) {
        unsafe {
            ffi::nk_world_remove_softbody(self.handle.as_ptr(), softbody.handle());
        }
    }

    #[cfg(feature = "softbody")]
    pub fn num_softbodies(&self) -> i32 {
        unsafe { ffi::nk_world_get_num_softbodies(self.handle.as_ptr()) }
    }

    #[cfg(feature = "softbody")]
    pub fn softbody_world_info(&self) -> super::softbody::SoftBodyWorldInfo {
        let info = super::softbody::SoftBodyWorldInfo::new();
        info.set_gravity(self.get_gravity());
        info
    }

    #[cfg(feature = "vehicle")]
    pub fn add_vehicle(&self, vehicle: &super::vehicle::Vehicle) {
        unsafe {
            ffi::nk_world_add_vehicle(self.handle.as_ptr(), vehicle.handle());
        }
    }

    #[cfg(feature = "vehicle")]
    pub fn remove_vehicle(&self, vehicle: &super::vehicle::Vehicle) {
        unsafe {
            ffi::nk_world_remove_vehicle(self.handle.as_ptr(), vehicle.handle());
        }
    }

    #[cfg(feature = "character")]
    pub fn add_character(&self, character: &super::character::CharacterController) {
        unsafe {
            ffi::nk_world_add_character(self.handle.as_ptr(), character.handle());
        }
    }

    #[cfg(feature = "character")]
    pub fn remove_character(&self, character: &super::character::CharacterController) {
        unsafe {
            ffi::nk_world_remove_character(self.handle.as_ptr(), character.handle());
        }
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PhysicsWorld {
    fn drop(&mut self) {
        self.constraints.clear();
        self.ghosts.clear();
        self.bodies.clear();
        unsafe {
            ffi::nk_world_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for PhysicsWorld {}
unsafe impl Sync for PhysicsWorld {}

#[derive(Debug, Clone)]
pub struct RayTestResult {
    pub hit_point: Vec3,
    pub hit_normal: Vec3,
    pub hit_fraction: Real,
    body_ptr: *mut c_void,
}

impl RayTestResult {
    pub fn get_rigid_body_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle> {
        for (handle, body) in world.bodies.iter() {
            if body.handle() == self.body_ptr {
                return Some(*handle);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: Real,
    pub body_a_ptr: *mut c_void,
    pub body_b_ptr: *mut c_void,
}

impl ContactPoint {
    pub fn get_body_a_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle> {
        for (handle, body) in world.bodies.iter() {
            if body.handle() == self.body_a_ptr {
                return Some(*handle);
            }
        }
        None
    }

    pub fn get_body_b_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle> {
        for (handle, body) in world.bodies.iter() {
            if body.handle() == self.body_b_ptr {
                return Some(*handle);
            }
        }
        None
    }
}

pub struct PhysicsWorldBuilder {
    gravity: Vec3,
    broadphase_type: BroadphaseType,
    solver_iterations: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BroadphaseType {
    DynamicAabbTree,
    AxisSweep3,
}

impl Default for PhysicsWorldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorldBuilder {
    pub fn new() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            broadphase_type: BroadphaseType::DynamicAabbTree,
            solver_iterations: 10,
        }
    }

    pub fn gravity(mut self, gravity: Vec3) -> Self {
        self.gravity = gravity;
        self
    }

    pub fn broadphase(mut self, broadphase_type: BroadphaseType) -> Self {
        self.broadphase_type = broadphase_type;
        self
    }

    pub fn solver_iterations(mut self, iterations: i32) -> Self {
        self.solver_iterations = iterations;
        self
    }

    pub fn build(self) -> PhysicsWorld {
        let world = PhysicsWorld::new();
        world.set_gravity(self.gravity);
        world
    }
}
