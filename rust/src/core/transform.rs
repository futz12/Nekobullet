use super::types::{Mat4, Quat, Transform, Vec3};

#[repr(C)]
pub struct FfiVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for FfiVec3 {
    fn from(v: Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl From<FfiVec3> for Vec3 {
    fn from(v: FfiVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<&Vec3> for FfiVec3 {
    fn from(v: &Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

#[repr(C)]
pub struct FfiQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Quat> for FfiQuat {
    fn from(q: Quat) -> Self {
        Self { x: q.x, y: q.y, z: q.z, w: q.w }
    }
}

impl From<FfiQuat> for Quat {
    fn from(q: FfiQuat) -> Self {
        Self::from_xyzw(q.x, q.y, q.z, q.w)
    }
}

impl From<&Quat> for FfiQuat {
    fn from(q: &Quat) -> Self {
        Self { x: q.x, y: q.y, z: q.z, w: q.w }
    }
}

#[repr(C)]
pub struct FfiTransform {
    pub position: FfiVec3,
    pub rotation: FfiQuat,
}

impl From<Transform> for FfiTransform {
    fn from(t: Transform) -> Self {
        Self {
            position: t.position.into(),
            rotation: t.rotation.into(),
        }
    }
}

impl From<FfiTransform> for Transform {
    fn from(t: FfiTransform) -> Self {
        Self {
            position: t.position.into(),
            rotation: t.rotation.into(),
        }
    }
}

impl From<&Transform> for FfiTransform {
    fn from(t: &Transform) -> Self {
        Self {
            position: (&t.position).into(),
            rotation: (&t.rotation).into(),
        }
    }
}

#[repr(C, align(16))]
pub struct FfiMat4 {
    pub m: [f32; 16],
}

impl From<Mat4> for FfiMat4 {
    fn from(m: Mat4) -> Self {
        let mut result = Self { m: [0.0; 16] };
        result.m.copy_from_slice(m.to_cols_array().as_slice());
        result
    }
}

impl From<FfiMat4> for Mat4 {
    fn from(m: FfiMat4) -> Self {
        Self::from_cols_array(&m.m)
    }
}

impl From<&Mat4> for FfiMat4 {
    fn from(m: &Mat4) -> Self {
        let mut result = Self { m: [0.0; 16] };
        result.m.copy_from_slice(m.to_cols_array().as_slice());
        result
    }
}

pub fn vec3_to_ptr(v: &Vec3) -> *const f32 {
    v.as_ref().as_ptr()
}

pub fn quat_to_ptr(q: &Quat) -> *const f32 {
    q.as_ref().as_ptr()
}

pub fn mat4_to_ptr(m: &Mat4) -> *const f32 {
    m.as_ref().as_ptr()
}

pub fn mat4_to_mut_ptr(m: &mut Mat4) -> *mut f32 {
    m.as_mut().as_mut_ptr()
}

pub unsafe fn ptr_to_vec3(ptr: *const f32) -> Vec3 {
    let slice = std::slice::from_raw_parts(ptr, 3);
    Vec3::new(slice[0], slice[1], slice[2])
}

pub unsafe fn ptr_to_quat(ptr: *const f32) -> Quat {
    let slice = std::slice::from_raw_parts(ptr, 4);
    Quat::from_xyzw(slice[0], slice[1], slice[2], slice[3])
}

pub unsafe fn ptr_to_mat4(ptr: *const f32) -> Mat4 {
    let slice = std::slice::from_raw_parts(ptr, 16);
    let mut arr = [0.0f32; 16];
    arr.copy_from_slice(slice);
    Mat4::from_cols_array(&arr)
}
