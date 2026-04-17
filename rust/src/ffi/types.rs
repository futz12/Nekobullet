#[cfg(feature = "double-precision")]
pub type nkReal = f64;

#[cfg(not(feature = "double-precision"))]
pub type nkReal = f32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: nkReal,
    pub y: nkReal,
    pub z: nkReal,
}

impl Vector3 {
    pub fn new(x: nkReal, y: nkReal, z: nkReal) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<(nkReal, nkReal, nkReal)> for Vector3 {
    fn from((x, y, z): (nkReal, nkReal, nkReal)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vector3> for (nkReal, nkReal, nkReal) {
    fn from(v: Vector3) -> Self {
        (v.x, v.y, v.z)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Quaternion {
    pub x: nkReal,
    pub y: nkReal,
    pub z: nkReal,
    pub w: nkReal,
}

impl Quaternion {
    pub fn new(x: nkReal, y: nkReal, z: nkReal, w: nkReal) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl From<(nkReal, nkReal, nkReal, nkReal)> for Quaternion {
    fn from((x, y, z, w): (nkReal, nkReal, nkReal, nkReal)) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<Quaternion> for (nkReal, nkReal, nkReal, nkReal) {
    fn from(q: Quaternion) -> Self {
        (q.x, q.y, q.z, q.w)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub origin: Vector3,
    pub rotation: Quaternion,
}

impl Transform {
    pub fn new(origin: Vector3, rotation: Quaternion) -> Self {
        Self { origin, rotation }
    }

    pub fn identity() -> Self {
        Self {
            origin: Vector3::zero(),
            rotation: Quaternion::identity(),
        }
    }

    pub fn from_translation(x: nkReal, y: nkReal, z: nkReal) -> Self {
        Self {
            origin: Vector3::new(x, y, z),
            rotation: Quaternion::identity(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nkVector3 {
    pub data: [nkReal; 3],
}

impl nkVector3 {
    pub fn new(x: nkReal, y: nkReal, z: nkReal) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn from_vector3(v: &Vector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.data[0], self.data[1], self.data[2])
    }
}

impl Default for nkVector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nkQuaternion {
    pub data: [nkReal; 4],
}

impl nkQuaternion {
    pub fn new(x: nkReal, y: nkReal, z: nkReal, w: nkReal) -> Self {
        Self { data: [x, y, z, w] }
    }

    pub fn from_quaternion(q: &Quaternion) -> Self {
        Self::new(q.x, q.y, q.z, q.w)
    }

    pub fn to_quaternion(&self) -> Quaternion {
        Quaternion::new(self.data[0], self.data[1], self.data[2], self.data[3])
    }
}

impl Default for nkQuaternion {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub struct nkTransform {
    pub origin: nkVector3,
    pub rotation: nkQuaternion,
}

impl nkTransform {
    pub fn new(origin: nkVector3, rotation: nkQuaternion) -> Self {
        Self { origin, rotation }
    }

    pub fn from_transform(t: &Transform) -> Self {
        Self {
            origin: nkVector3::from_vector3(&t.origin),
            rotation: nkQuaternion::from_quaternion(&t.rotation),
        }
    }

    pub fn to_transform(&self) -> Transform {
        Transform {
            origin: self.origin.to_vector3(),
            rotation: self.rotation.to_quaternion(),
        }
    }

    pub fn from_core_transform(t: &super::super::core::types::Transform) -> Self {
        Self {
            origin: nkVector3::new(t.position.x, t.position.y, t.position.z),
            rotation: nkQuaternion::new(t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w),
        }
    }

    pub fn to_core_transform(&self) -> super::super::core::types::Transform {
        super::super::core::types::Transform {
            position: super::super::core::types::Vec3::new(self.origin.data[0], self.origin.data[1], self.origin.data[2]),
            rotation: super::super::core::types::Quat::from_xyzw(
                self.rotation.data[0],
                self.rotation.data[1],
                self.rotation.data[2],
                self.rotation.data[3],
            ),
        }
    }
}


pub type nkWorldHandle = *mut std::ffi::c_void;
pub type nkRigidBodyHandle = *mut std::ffi::c_void;
pub type nkShapeHandle = *mut std::ffi::c_void;
pub type nkConstraintHandle = *mut std::ffi::c_void;
pub type nkVehicleHandle = *mut std::ffi::c_void;
pub type nkVehicleRaycasterHandle = *mut std::ffi::c_void;
pub type nkMultiBodyHandle = *mut std::ffi::c_void;
pub type nkGhostObjectHandle = *mut std::ffi::c_void;
pub type nkSoftBodyHandle = *mut std::ffi::c_void;
pub type nkSoftBodyWorldInfoHandle = *mut std::ffi::c_void;
pub type nkCharacterHandle = *mut std::ffi::c_void;
pub type nkHACDHandle = *mut std::ffi::c_void;
pub type nkVHACDHandle = *mut std::ffi::c_void;
pub type nkWorldImporterHandle = *mut std::ffi::c_void;
pub type nkGImpactShapeHandle = *mut std::ffi::c_void;

pub const JOINT_TYPE_FIXED: i32 = 0;
pub const JOINT_TYPE_REVOLUTE: i32 = 1;
pub const JOINT_TYPE_PRISMATIC: i32 = 2;
pub const JOINT_TYPE_FLOATING: i32 = 3;
pub const JOINT_TYPE_SPHERICAL: i32 = 4;
