use std::ffi::c_void;
use std::ptr::NonNull;

use super::types::{Quat, Real, Transform, Vec3};
use crate::ffi::{self, nkTransform};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConstraintType {
    Point2Point = 0,
    Hinge = 1,
    Slider = 2,
    Fixed = 3,
    Generic6Dof = 4,
    ConeTwist = 5,
    Generic6DofSpring = 6,
    Universal = 7,
    Hinge2 = 8,
    Gear = 9,
    Generic6DofSpring2 = 10,
    Unknown = 255,
}

impl ConstraintType {
    fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Point2Point,
            1 => Self::Hinge,
            2 => Self::Slider,
            3 => Self::Fixed,
            4 => Self::Generic6Dof,
            5 => Self::ConeTwist,
            6 => Self::Generic6DofSpring,
            7 => Self::Universal,
            8 => Self::Hinge2,
            9 => Self::Gear,
            10 => Self::Generic6DofSpring2,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConstraintParam {
    Erp = 1,
    StopErp = 2,
    Cfm = 3,
    StopCfm = 4,
}

pub struct Constraint {
    handle: NonNull<c_void>,
    constraint_type: ConstraintType,
}

impl Constraint {
    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn constraint_type(&self) -> ConstraintType {
        let raw_type = unsafe { ffi::nk_constraint_get_constraint_type(self.handle.as_ptr()) };
        let queried = ConstraintType::from_raw(raw_type);
        if queried == ConstraintType::Unknown {
            self.constraint_type
        } else {
            queried
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::nk_constraint_set_enabled(self.handle.as_ptr(), if enabled { 1 } else { 0 });
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::nk_constraint_is_enabled(self.handle.as_ptr()) != 0 }
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        unsafe {
            ffi::nk_constraint_set_breaking_impulse_threshold(self.handle.as_ptr(), threshold);
        }
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        unsafe { ffi::nk_constraint_get_breaking_impulse_threshold(self.handle.as_ptr()) }
    }

    pub fn set_param(&self, param: ConstraintParam, value: Real, axis: i32) {
        unsafe {
            ffi::nk_constraint_set_param(self.handle.as_ptr(), param as i32, value, axis);
        }
    }

    pub fn get_param(&self, param: ConstraintParam, axis: i32) -> Real {
        unsafe { ffi::nk_constraint_get_param(self.handle.as_ptr(), param as i32, axis) }
    }
}

pub struct Generic6DofConstraint {
    inner: Constraint,
}

impl Generic6DofConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
        use_linear_reference_frame_a: bool,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_generic_6dof(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
                if use_linear_reference_frame_a { 1 } else { 0 },
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Generic6Dof,
            },
        }
    }

    pub fn set_linear_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_linear_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn get_linear_lower_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_linear_lower_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_linear_upper_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_linear_upper_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_angular_lower_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_angular_lower_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_angular_upper_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_angular_upper_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct Generic6DofSpringConstraint {
    inner: Constraint,
}

impl Generic6DofSpringConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
        use_linear_reference_frame_a: bool,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_generic_6dof_spring(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
                if use_linear_reference_frame_a { 1 } else { 0 },
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Generic6DofSpring,
            },
        }
    }

    pub fn enable_spring(&self, axis: i32, enable: bool) {
        unsafe {
            ffi::nk_constraint_enable_spring_6dof(self.inner.handle.as_ptr(), axis, if enable { 1 } else { 0 });
        }
    }

    pub fn set_stiffness(&self, axis: i32, stiffness: Real) {
        unsafe {
            ffi::nk_constraint_set_stiffness_6dof(self.inner.handle.as_ptr(), axis, stiffness);
        }
    }

    pub fn set_damping(&self, axis: i32, damping: Real) {
        unsafe {
            ffi::nk_constraint_set_damping_6dof(self.inner.handle.as_ptr(), axis, damping);
        }
    }

    pub fn set_equilibrium_point(&self, axis: i32, val: Real) {
        unsafe {
            ffi::nk_constraint_set_equilibrium_point_6dof(self.inner.handle.as_ptr(), axis, val);
        }
    }

    pub fn set_linear_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_linear_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct ConeTwistConstraint {
    inner: Constraint,
}

impl ConeTwistConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_cone_twist(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::ConeTwist,
            },
        }
    }

    pub fn set_limit(
        &self,
        swing_span1: Real,
        swing_span2: Real,
        twist_span: Real,
        softness: Real,
        bias_factor: Real,
        relaxation_factor: Real,
    ) {
        unsafe {
            ffi::nk_constraint_set_limit_cone_twist(
                self.inner.handle.as_ptr(),
                swing_span1,
                swing_span2,
                twist_span,
                softness,
                bias_factor,
                relaxation_factor,
            );
        }
    }

    pub fn set_motor_target(&self, target: Quat) {
        unsafe {
            ffi::nk_constraint_set_motor_target_cone_twist(
                self.inner.handle.as_ptr(),
                target.x,
                target.y,
                target.z,
                target.w,
            );
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct UniversalConstraint {
    inner: Constraint,
}

impl UniversalConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        anchor: Vec3,
        axis1: Vec3,
        axis2: Vec3,
    ) -> Self {
        let handle = unsafe {
            ffi::nk_constraint_create_universal(
                body_a_handle,
                body_b_handle,
                anchor.x, anchor.y, anchor.z,
                axis1.x, axis1.y, axis1.z,
                axis2.x, axis2.y, axis2.z,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Universal,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct Hinge2Constraint {
    inner: Constraint,
}

impl Hinge2Constraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        anchor: Vec3,
        axis1: Vec3,
        axis2: Vec3,
    ) -> Self {
        let handle = unsafe {
            ffi::nk_constraint_create_hinge2(
                body_a_handle,
                body_b_handle,
                anchor.x, anchor.y, anchor.z,
                axis1.x, axis1.y, axis1.z,
                axis2.x, axis2.y, axis2.z,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Hinge2,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct GearConstraint {
    inner: Constraint,
}

impl GearConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        axis_a: Vec3,
        axis_b: Vec3,
    ) -> Self {
        let handle = unsafe {
            ffi::nk_constraint_create_gear(
                body_a_handle,
                body_b_handle,
                axis_a.x, axis_a.y, axis_a.z,
                axis_b.x, axis_b.y, axis_b.z,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Gear,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct Generic6DofSpring2Constraint {
    inner: Constraint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RotateOrder {
    XYZ = 0,
    XZY = 1,
    YXZ = 2,
    YZX = 3,
    ZXY = 4,
    ZYX = 5,
}

impl Generic6DofSpring2Constraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
        rotate_order: RotateOrder,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_generic_6dof_spring2(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
                rotate_order as i32,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Generic6DofSpring2,
            },
        }
    }

    pub fn set_linear_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_linear_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_linear_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_lower_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_lower_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn set_angular_upper_limit(&self, limit: Vec3) {
        unsafe {
            ffi::nk_constraint_set_angular_upper_limit(self.inner.handle.as_ptr(), limit.x, limit.y, limit.z);
        }
    }

    pub fn get_linear_lower_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_linear_lower_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_linear_upper_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_linear_upper_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_angular_lower_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_angular_lower_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_angular_upper_limit(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_constraint_get_angular_upper_limit(self.inner.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct HingeConstraint {
    inner: Constraint,
}

pub struct FixedConstraint {
    inner: Constraint,
}

impl FixedConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
    ) -> Self {
        let handle = unsafe {
            ffi::nk_constraint_create_fixed(
                body_a_handle,
                body_b_handle,
            )
        };

        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Fixed,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

impl HingeConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
        low_limit: Real,
        high_limit: Real,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_hinge(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
                low_limit,
                high_limit,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Hinge,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct SliderConstraint {
    inner: Constraint,
}

impl SliderConstraint {
    pub unsafe fn new(
        body_a_handle: *mut c_void,
        body_b_handle: *mut c_void,
        frame_a: Transform,
        frame_b: Transform,
    ) -> Self {
        let nk_frame_a = nkTransform::from_core_transform(&frame_a);
        let nk_frame_b = nkTransform::from_core_transform(&frame_b);
        
        let handle = unsafe {
            ffi::nk_constraint_create_slider(
                body_a_handle,
                body_b_handle,
                &nk_frame_a,
                &nk_frame_b,
            )
        };
        
        Self {
            inner: Constraint {
                handle: unsafe { NonNull::new_unchecked(handle) },
                constraint_type: ConstraintType::Slider,
            },
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }

    pub fn set_breaking_impulse_threshold(&self, threshold: Real) {
        self.inner.set_breaking_impulse_threshold(threshold);
    }

    pub fn get_breaking_impulse_threshold(&self) -> Real {
        self.inner.get_breaking_impulse_threshold()
    }

    pub fn handle(&self) -> *mut c_void {
        self.inner.handle.as_ptr()
    }
}

pub struct ConstraintBuilder {
    constraint_type: ConstraintType,
    body_a: Option<*mut c_void>,
    body_b: Option<*mut c_void>,
    frame_a: Transform,
    frame_b: Transform,
    pivot_a: Vec3,
    pivot_b: Vec3,
    use_linear_reference_frame_a: bool,
    linear_lower_limit: Vec3,
    linear_upper_limit: Vec3,
    angular_lower_limit: Vec3,
    angular_upper_limit: Vec3,
    breaking_impulse_threshold: Real,
    enable_spring: [bool; 6],
    spring_stiffness: [Real; 6],
    spring_damping: [Real; 6],
    cone_twist_limit: Option<(Real, Real, Real, Real, Real, Real)>,
    stop_erp: Option<Real>,
}

impl ConstraintBuilder {
    pub fn new() -> Self {
        Self {
            constraint_type: ConstraintType::Point2Point,
            body_a: None,
            body_b: None,
            frame_a: Transform::identity(),
            frame_b: Transform::identity(),
            pivot_a: Vec3::ZERO,
            pivot_b: Vec3::ZERO,
            use_linear_reference_frame_a: true,
            linear_lower_limit: Vec3::new(-1.0, -1.0, -1.0),
            linear_upper_limit: Vec3::new(1.0, 1.0, 1.0),
            angular_lower_limit: Vec3::new(-std::f32::consts::PI, -std::f32::consts::PI, -std::f32::consts::PI),
            angular_upper_limit: Vec3::new(std::f32::consts::PI, std::f32::consts::PI, std::f32::consts::PI),
            breaking_impulse_threshold: f32::MAX,
            enable_spring: [false; 6],
            spring_stiffness: [0.0; 6],
            spring_damping: [0.0; 6],
            cone_twist_limit: None,
            stop_erp: None,
        }
    }

    pub fn point2point(mut self, body_a: *mut c_void, body_b: *mut c_void) -> Self {
        self.constraint_type = ConstraintType::Point2Point;
        self.body_a = Some(body_a);
        self.body_b = Some(body_b);
        self
    }

    pub fn fixed(mut self, body_a: *mut c_void, body_b: *mut c_void) -> Self {
        self.constraint_type = ConstraintType::Fixed;
        self.body_a = Some(body_a);
        self.body_b = Some(body_b);
        self
    }

    pub fn generic_6dof(mut self, body_a: *mut c_void, body_b: *mut c_void) -> Self {
        self.constraint_type = ConstraintType::Generic6Dof;
        self.body_a = Some(body_a);
        self.body_b = Some(body_b);
        self
    }

    pub fn generic_6dof_spring(mut self, body_a: *mut c_void, body_b: *mut c_void) -> Self {
        self.constraint_type = ConstraintType::Generic6DofSpring;
        self.body_a = Some(body_a);
        self.body_b = Some(body_b);
        self
    }

    pub fn cone_twist(mut self, body_a: *mut c_void, body_b: *mut c_void) -> Self {
        self.constraint_type = ConstraintType::ConeTwist;
        self.body_a = Some(body_a);
        self.body_b = Some(body_b);
        self
    }

    pub fn pivot_a(mut self, pivot: Vec3) -> Self {
        self.pivot_a = pivot;
        self
    }

    pub fn pivot_b(mut self, pivot: Vec3) -> Self {
        self.pivot_b = pivot;
        self
    }

    pub fn frame_a(mut self, frame: Transform) -> Self {
        self.frame_a = frame;
        self
    }

    pub fn frame_b(mut self, frame: Transform) -> Self {
        self.frame_b = frame;
        self
    }

    pub fn linear_limits(mut self, lower: Vec3, upper: Vec3) -> Self {
        self.linear_lower_limit = lower;
        self.linear_upper_limit = upper;
        self
    }

    pub fn angular_limits(mut self, lower: Vec3, upper: Vec3) -> Self {
        self.angular_lower_limit = lower;
        self.angular_upper_limit = upper;
        self
    }

    pub fn breaking_impulse_threshold(mut self, threshold: Real) -> Self {
        self.breaking_impulse_threshold = threshold;
        self
    }

    pub fn spring(mut self, axis: i32, stiffness: Real, damping: Real) -> Self {
        if (0..6).contains(&axis) {
            self.enable_spring[axis as usize] = true;
            self.spring_stiffness[axis as usize] = stiffness;
            self.spring_damping[axis as usize] = damping;
        }
        self
    }

    pub fn cone_twist_limit(
        mut self,
        swing_span1: Real,
        swing_span2: Real,
        twist_span: Real,
        softness: Real,
        bias_factor: Real,
        relaxation_factor: Real,
    ) -> Self {
        self.cone_twist_limit = Some((swing_span1, swing_span2, twist_span, softness, bias_factor, relaxation_factor));
        self
    }

    pub fn stop_erp(mut self, erp: Real) -> Self {
        self.stop_erp = Some(erp);
        self
    }

    pub fn build(self) -> Result<*mut c_void, &'static str> {
        let body_a = self.body_a.ok_or("Body A is required")?;
        let body_b = self.body_b.ok_or("Body B is required")?;

        let constraint_handle = match self.constraint_type {
            ConstraintType::Point2Point => {
                
                unsafe {
                    ffi::nk_constraint_create_point2point(
                        body_a,
                        body_b,
                        self.pivot_a.x, self.pivot_a.y, self.pivot_a.z,
                        self.pivot_b.x, self.pivot_b.y, self.pivot_b.z,
                    )
                }
            }
            ConstraintType::Fixed => {
                unsafe {
                    ffi::nk_constraint_create_fixed(
                        body_a,
                        body_b,
                    )
                }
            }
            ConstraintType::Generic6Dof => {
                let nk_frame_a = nkTransform::from_core_transform(&self.frame_a);
                let nk_frame_b = nkTransform::from_core_transform(&self.frame_b);
                
                let handle = unsafe {
                    ffi::nk_constraint_create_generic_6dof(
                        body_a,
                        body_b,
                        &nk_frame_a,
                        &nk_frame_b,
                        if self.use_linear_reference_frame_a { 1 } else { 0 },
                    )
                };
                
                if !handle.is_null() {
                    unsafe {
                        ffi::nk_constraint_set_linear_lower_limit(handle, self.linear_lower_limit.x, self.linear_lower_limit.y, self.linear_lower_limit.z);
                        ffi::nk_constraint_set_linear_upper_limit(handle, self.linear_upper_limit.x, self.linear_upper_limit.y, self.linear_upper_limit.z);
                        ffi::nk_constraint_set_angular_lower_limit(handle, self.angular_lower_limit.x, self.angular_lower_limit.y, self.angular_lower_limit.z);
                        ffi::nk_constraint_set_angular_upper_limit(handle, self.angular_upper_limit.x, self.angular_upper_limit.y, self.angular_upper_limit.z);
                    }
                }
                handle
            }
            ConstraintType::Generic6DofSpring => {
                let nk_frame_a = nkTransform::from_core_transform(&self.frame_a);
                let nk_frame_b = nkTransform::from_core_transform(&self.frame_b);
                
                let handle = unsafe {
                    ffi::nk_constraint_create_generic_6dof_spring(
                        body_a,
                        body_b,
                        &nk_frame_a,
                        &nk_frame_b,
                        if self.use_linear_reference_frame_a { 1 } else { 0 },
                    )
                };
                
                if !handle.is_null() {
                    unsafe {
                        ffi::nk_constraint_set_linear_lower_limit(handle, self.linear_lower_limit.x, self.linear_lower_limit.y, self.linear_lower_limit.z);
                        ffi::nk_constraint_set_linear_upper_limit(handle, self.linear_upper_limit.x, self.linear_upper_limit.y, self.linear_upper_limit.z);
                        ffi::nk_constraint_set_angular_lower_limit(handle, self.angular_lower_limit.x, self.angular_lower_limit.y, self.angular_lower_limit.z);
                        ffi::nk_constraint_set_angular_upper_limit(handle, self.angular_upper_limit.x, self.angular_upper_limit.y, self.angular_upper_limit.z);
                        
                        for i in 0..6 {
                            if self.enable_spring[i] {
                                ffi::nk_constraint_enable_spring_6dof(handle, i as i32, 1);
                                ffi::nk_constraint_set_stiffness_6dof(handle, i as i32, self.spring_stiffness[i]);
                                ffi::nk_constraint_set_damping_6dof(handle, i as i32, self.spring_damping[i]);
                            }
                        }
                    }
                }
                handle
            }
            ConstraintType::ConeTwist => {
                let nk_frame_a = nkTransform::from_core_transform(&self.frame_a);
                let nk_frame_b = nkTransform::from_core_transform(&self.frame_b);
                
                let handle = unsafe {
                    ffi::nk_constraint_create_cone_twist(
                        body_a,
                        body_b,
                        &nk_frame_a,
                        &nk_frame_b,
                    )
                };
                
                if !handle.is_null() {
                    if let Some((s1, s2, ts, soft, bias, relax)) = self.cone_twist_limit {
                        unsafe {
                            ffi::nk_constraint_set_limit_cone_twist(handle, s1, s2, ts, soft, bias, relax);
                        }
                    }
                }
                handle
            }
            _ => return Err("Unsupported constraint type"),
        };

        if constraint_handle.is_null() {
            return Err("Failed to create constraint");
        }

        if self.breaking_impulse_threshold < f32::MAX {
            unsafe {
                ffi::nk_constraint_set_breaking_impulse_threshold(constraint_handle, self.breaking_impulse_threshold);
            }
        }

        if let Some(erp) = self.stop_erp {
            unsafe {
                for axis in 0..6 {
                    ffi::nk_constraint_set_param(constraint_handle, ConstraintParam::StopErp as i32, erp, axis);
                }
            }
        }

        Ok(constraint_handle)
    }
}

impl Default for ConstraintBuilder {
    fn default() -> Self {
        Self::new()
    }
}
