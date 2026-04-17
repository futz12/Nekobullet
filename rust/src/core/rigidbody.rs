use std::cell::Cell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use super::collision::ShapeHandle;
use super::types::{ActivationState, MotionType, Real, Transform, Vec3};
use crate::ffi::{self, nkTransform};

pub struct RigidBody {
    handle: NonNull<c_void>,
    world: Cell<Option<NonNull<c_void>>>,
    shape: Option<ShapeHandle>,
    _marker: PhantomData<*mut ()>,
}

impl RigidBody {
    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn get_transform(&self) -> Transform {
        let mut ffi_transform: nkTransform = nkTransform::default();
        unsafe {
            ffi::nk_rigidbody_get_transform(self.handle.as_ptr(), &mut ffi_transform);
        }
        ffi_transform.to_core_transform()
    }

    pub fn set_transform(&self, transform: &Transform) {
        let ffi_transform = nkTransform::from_core_transform(transform);
        unsafe {
            ffi::nk_rigidbody_set_transform(self.handle.as_ptr(), &ffi_transform);
        }
    }

    pub fn get_position(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_rigidbody_get_position(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_position(&self, position: Vec3) {
        unsafe {
            ffi::nk_rigidbody_set_position(
                self.handle.as_ptr(),
                position.x,
                position.y,
                position.z,
            );
        }
    }

    pub fn get_rotation(&self) -> glam::Quat {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        let mut w: Real = 1.0;
        unsafe {
            ffi::nk_rigidbody_get_rotation(self.handle.as_ptr(), &mut x, &mut y, &mut z, &mut w);
        }
        glam::Quat::from_xyzw(x, y, z, w)
    }

    pub fn set_rotation(&self, rotation: glam::Quat) {
        unsafe {
            ffi::nk_rigidbody_set_rotation(
                self.handle.as_ptr(),
                rotation.x,
                rotation.y,
                rotation.z,
                rotation.w,
            );
        }
    }

    pub fn get_linear_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_rigidbody_get_linear_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_linear_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_rigidbody_set_linear_velocity(
                self.handle.as_ptr(),
                velocity.x,
                velocity.y,
                velocity.z,
            );
        }
    }

    pub fn get_angular_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_rigidbody_get_angular_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_angular_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_rigidbody_set_angular_velocity(
                self.handle.as_ptr(),
                velocity.x,
                velocity.y,
                velocity.z,
            );
        }
    }

    pub fn apply_force(&self, force: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_force(self.handle.as_ptr(), force.x, force.y, force.z);
        }
    }

    pub fn apply_central_force(&self, force: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_central_force(self.handle.as_ptr(), force.x, force.y, force.z);
        }
    }

    pub fn apply_impulse(&self, impulse: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_impulse(self.handle.as_ptr(), impulse.x, impulse.y, impulse.z);
        }
    }

    pub fn apply_central_impulse(&self, impulse: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_central_impulse(self.handle.as_ptr(), impulse.x, impulse.y, impulse.z);
        }
    }

    pub fn apply_torque(&self, torque: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_torque(self.handle.as_ptr(), torque.x, torque.y, torque.z);
        }
    }

    pub fn apply_torque_impulse(&self, torque: Vec3) {
        unsafe {
            ffi::nk_rigidbody_apply_torque_impulse(self.handle.as_ptr(), torque.x, torque.y, torque.z);
        }
    }

    pub fn clear_forces(&self) {
        unsafe {
            ffi::nk_rigidbody_clear_forces(self.handle.as_ptr());
        }
    }

    pub fn get_mass(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_mass(self.handle.as_ptr()) }
    }

    pub fn set_mass(&self, mass: Real) {
        unsafe {
            ffi::nk_rigidbody_set_mass(self.handle.as_ptr(), mass);
        }
    }

    pub fn get_inverse_mass(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_inverse_mass(self.handle.as_ptr()) }
    }

    pub fn get_friction(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_friction(self.handle.as_ptr()) }
    }

    pub fn set_friction(&self, friction: Real) {
        unsafe {
            ffi::nk_rigidbody_set_friction(self.handle.as_ptr(), friction);
        }
    }

    pub fn get_rolling_friction(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_rolling_friction(self.handle.as_ptr()) }
    }

    pub fn set_rolling_friction(&self, friction: Real) {
        unsafe {
            ffi::nk_rigidbody_set_rolling_friction(self.handle.as_ptr(), friction);
        }
    }

    pub fn get_spinning_friction(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_spinning_friction(self.handle.as_ptr()) }
    }

    pub fn set_spinning_friction(&self, friction: Real) {
        unsafe {
            ffi::nk_rigidbody_set_spinning_friction(self.handle.as_ptr(), friction);
        }
    }

    pub fn get_restitution(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_restitution(self.handle.as_ptr()) }
    }

    pub fn set_restitution(&self, restitution: Real) {
        unsafe {
            ffi::nk_rigidbody_set_restitution(self.handle.as_ptr(), restitution);
        }
    }

    pub fn get_linear_damping(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_linear_damping(self.handle.as_ptr()) }
    }

    pub fn get_angular_damping(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_angular_damping(self.handle.as_ptr()) }
    }

    pub fn set_damping(&self, linear: Real, angular: Real) {
        unsafe {
            ffi::nk_rigidbody_set_damping(self.handle.as_ptr(), linear, angular);
        }
    }

    pub fn set_sleeping_thresholds(&self, linear: Real, angular: Real) {
        unsafe {
            ffi::nk_rigidbody_set_sleeping_thresholds(self.handle.as_ptr(), linear, angular);
        }
    }

    pub fn get_linear_sleeping_threshold(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_linear_sleeping_threshold(self.handle.as_ptr()) }
    }

    pub fn get_angular_sleeping_threshold(&self) -> Real {
        unsafe { ffi::nk_rigidbody_get_angular_sleeping_threshold(self.handle.as_ptr()) }
    }

    pub fn get_activation_state(&self) -> ActivationState {
        let state = unsafe { ffi::nk_rigidbody_get_activation_state(self.handle.as_ptr()) };
        match state {
            0 => ActivationState::Inactive,
            1 => ActivationState::Active,
            2 => ActivationState::DisableDeactivation,
            3 => ActivationState::DisableSimulation,
            _ => ActivationState::Active,
        }
    }

    pub fn set_activation_state(&self, state: ActivationState) {
        unsafe {
            ffi::nk_rigidbody_set_activation_state(self.handle.as_ptr(), state as i32);
        }
    }

    pub fn activate(&self) {
        unsafe {
            ffi::nk_rigidbody_activate(self.handle.as_ptr());
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { ffi::nk_rigidbody_is_active(self.handle.as_ptr()) != 0 }
    }

    pub fn is_static(&self) -> bool {
        unsafe { ffi::nk_rigidbody_is_static(self.handle.as_ptr()) != 0 }
    }

    pub fn set_static(&self, is_static: bool) {
        unsafe {
            ffi::nk_rigidbody_set_static(self.handle.as_ptr(), if is_static { 1 } else { 0 });
        }
    }

    pub fn is_kinematic(&self) -> bool {
        unsafe { ffi::nk_rigidbody_is_kinematic(self.handle.as_ptr()) != 0 }
    }

    pub fn is_dynamic(&self) -> bool {
        unsafe { ffi::nk_rigidbody_is_dynamic(self.handle.as_ptr()) != 0 }
    }

    pub fn set_kinematic(&self, kinematic: bool) {
        unsafe {
            ffi::nk_rigidbody_set_kinematic(self.handle.as_ptr(), if kinematic { 1 } else { 0 });
        }
    }

    pub fn set_collision_shape(&mut self, shape: ShapeHandle) {
        unsafe {
            ffi::nk_rigidbody_set_collision_shape(self.handle.as_ptr(), shape.handle());
        }
        self.shape = Some(shape);
    }

    pub fn get_collision_shape(&self) -> Option<&ShapeHandle> {
        self.shape.as_ref()
    }

    pub fn get_aabb(&self) -> super::types::Aabb {
        let mut min_x: Real = 0.0;
        let mut min_y: Real = 0.0;
        let mut min_z: Real = 0.0;
        let mut max_x: Real = 0.0;
        let mut max_y: Real = 0.0;
        let mut max_z: Real = 0.0;
        unsafe {
            ffi::nk_rigidbody_get_aabb(
                self.handle.as_ptr(),
                &mut min_x,
                &mut min_y,
                &mut min_z,
                &mut max_x,
                &mut max_y,
                &mut max_z,
            );
        }
        super::types::Aabb::new(Vec3::new(min_x, min_y, min_z), Vec3::new(max_x, max_y, max_z))
    }

    pub fn set_collision_group(&self, group: i32) {
        unsafe {
            ffi::nk_rigidbody_set_collision_group(self.handle.as_ptr(), group);
        }
    }

    pub fn get_collision_group(&self) -> i32 {
        unsafe { ffi::nk_rigidbody_get_collision_group(self.handle.as_ptr()) }
    }

    pub fn set_collision_mask(&self, mask: i32) {
        unsafe {
            ffi::nk_rigidbody_set_collision_mask(self.handle.as_ptr(), mask);
        }
    }

    pub fn get_collision_mask(&self) -> i32 {
        unsafe { ffi::nk_rigidbody_get_collision_mask(self.handle.as_ptr()) }
    }
}

impl Drop for RigidBody {
    fn drop(&mut self) {
        if let Some(world) = self.world.get() {
            unsafe {
                ffi::nk_rigidbody_destroy(world.as_ptr(), self.handle.as_ptr());
            }
        }
    }
}

unsafe impl Send for RigidBody {}
unsafe impl Sync for RigidBody {}

pub struct RigidBodyBuilder {
    shape: Option<ShapeHandle>,
    mass: Real,
    position: Vec3,
    rotation: glam::Quat,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
    friction: Real,
    restitution: Real,
    linear_damping: Real,
    angular_damping: Real,
    motion_type: MotionType,
    activation_state: ActivationState,
    collision_group: u16,
    collision_mask: u16,
    linear_sleeping_threshold: Real,
    angular_sleeping_threshold: Real,
    disable_deactivation: bool,
}

impl RigidBodyBuilder {
    pub fn new() -> Self {
        Self {
            shape: None,
            mass: 1.0,
            position: Vec3::ZERO,
            rotation: glam::Quat::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            friction: 0.5,
            restitution: 0.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            motion_type: MotionType::Dynamic,
            activation_state: ActivationState::Active,
            collision_group: 1,
            collision_mask: u16::MAX,
            linear_sleeping_threshold: 0.8,
            angular_sleeping_threshold: 1.0,
            disable_deactivation: false,
        }
    }

    pub fn shape(mut self, shape: ShapeHandle) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn mass(mut self, mass: Real) -> Self {
        self.mass = mass;
        if mass > 0.0 {
            self.motion_type = MotionType::Dynamic;
        } else {
            self.motion_type = MotionType::Static;
        }
        self
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: glam::Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn transform(self, transform: Transform) -> Self {
        self.position(transform.position).rotation(transform.rotation)
    }

    pub fn linear_velocity(mut self, velocity: Vec3) -> Self {
        self.linear_velocity = velocity;
        self
    }

    pub fn angular_velocity(mut self, velocity: Vec3) -> Self {
        self.angular_velocity = velocity;
        self
    }

    pub fn friction(mut self, friction: Real) -> Self {
        self.friction = friction;
        self
    }

    pub fn restitution(mut self, restitution: Real) -> Self {
        self.restitution = restitution;
        self
    }

    pub fn damping(mut self, linear: Real, angular: Real) -> Self {
        self.linear_damping = linear;
        self.angular_damping = angular;
        self
    }

    pub fn motion_type(mut self, motion_type: MotionType) -> Self {
        self.motion_type = motion_type;
        self
    }

    pub fn static_body(mut self) -> Self {
        self.motion_type = MotionType::Static;
        self.mass = 0.0;
        self
    }

    pub fn kinematic(mut self) -> Self {
        self.motion_type = MotionType::Kinematic;
        self
    }

    pub fn dynamic(mut self) -> Self {
        self.motion_type = MotionType::Dynamic;
        self
    }

    pub fn collision_filter(mut self, group: u16, mask: u16) -> Self {
        self.collision_group = group;
        self.collision_mask = mask;
        self
    }

    pub fn sleeping_threshold(mut self, linear: Real, angular: Real) -> Self {
        self.linear_sleeping_threshold = linear;
        self.angular_sleeping_threshold = angular;
        self
    }

    pub fn disable_deactivation(mut self, disable: bool) -> Self {
        self.disable_deactivation = disable;
        self
    }

    pub fn build(self) -> Result<RigidBody, &'static str> {
        let shape = self.shape.ok_or("Shape is required")?;
        
        let ffi_transform = nkTransform::from_core_transform(&Transform {
            position: self.position,
            rotation: self.rotation,
        });

        let handle = unsafe {
            ffi::nk_rigidbody_create(
                std::ptr::null_mut(),
                shape.handle(),
                self.mass,
                &ffi_transform,
            )
        };

        let handle = NonNull::new(handle).ok_or("Failed to create rigid body")?;

        let body = RigidBody {
            handle,
            world: Cell::new(None),
            shape: Some(shape),
            _marker: PhantomData,
        };

        body.set_linear_velocity(self.linear_velocity);
        body.set_angular_velocity(self.angular_velocity);
        body.set_friction(self.friction);
        body.set_restitution(self.restitution);
        body.set_damping(self.linear_damping, self.angular_damping);
        
        match self.motion_type {
            MotionType::Kinematic => body.set_kinematic(true),
            MotionType::Static => {
                body.set_kinematic(false);
            }
            MotionType::Dynamic => {}
        }
        
        body.set_activation_state(self.activation_state);

        if self.disable_deactivation {
            body.set_activation_state(ActivationState::DisableDeactivation);
        }

        Ok(body)
    }
}

impl Default for RigidBodyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
