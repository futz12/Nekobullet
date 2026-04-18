use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;

use super::types::{Real, Transform, Vec3};
use crate::ffi::{self, nkTransform};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CollisionShapeType {
    Box = 0,
    Sphere = 1,
    Capsule = 2,
    Cylinder = 3,
    Cone = 4,
    Compound = 5,
    ConvexHull = 6,
    TriangleMesh = 7,
    Heightfield = 8,
    StaticPlane = 9,
}

#[derive(Debug)]
pub enum CollisionShape {
    Box { half_extents: Vec3 },
    Sphere { radius: Real },
    Capsule { radius: Real, height: Real },
    CapsuleZ { radius: Real, height: Real },
    Cylinder { half_extents: Vec3 },
    Cone { radius: Real, height: Real },
    Plane { normal: Vec3, constant: Real },
    ConvexHull { points: Vec<Vec3> },
    Compound { shapes: Vec<(Transform, ShapeHandle)> },
    TriangleMesh { vertices: Vec<Real>, indices: Vec<i32> },
    Heightfield { width: i32, length: i32, heights: Vec<Real>, min_height: Real, max_height: Real, up_axis: i32 },
}

pub struct ShapeHandle {
    handle: NonNull<c_void>,
    shape_type: CollisionShapeType,
}

impl fmt::Debug for ShapeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShapeHandle")
            .field("shape_type", &self.shape_type)
            .finish()
    }
}

impl ShapeHandle {
    pub fn new_box(half_extents: Vec3) -> Self {
        let handle = unsafe {
            ffi::nk_shape_create_box(half_extents.x, half_extents.y, half_extents.z)
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create box shape"),
            shape_type: CollisionShapeType::Box,
        }
    }

    pub fn new_sphere(radius: Real) -> Self {
        let handle = unsafe { ffi::nk_shape_create_sphere(radius) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create sphere shape"),
            shape_type: CollisionShapeType::Sphere,
        }
    }

    pub fn new_capsule(radius: Real, height: Real) -> Self {
        let handle = unsafe { ffi::nk_shape_create_capsule(radius, height) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create capsule shape"),
            shape_type: CollisionShapeType::Capsule,
        }
    }

    pub fn new_capsule_z(radius: Real, height: Real) -> Self {
        let handle = unsafe { ffi::nk_shape_create_capsule_z(radius, height) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create capsule z shape"),
            shape_type: CollisionShapeType::Capsule,
        }
    }

    pub fn new_cylinder(half_extents: Vec3) -> Self {
        let handle = unsafe {
            ffi::nk_shape_create_cylinder(half_extents.x, half_extents.y)
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create cylinder shape"),
            shape_type: CollisionShapeType::Cylinder,
        }
    }

    pub fn new_cone(radius: Real, height: Real) -> Self {
        let handle = unsafe { ffi::nk_shape_create_cone(radius, height) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create cone shape"),
            shape_type: CollisionShapeType::Cone,
        }
    }

    pub fn new_plane(normal: Vec3, constant: Real) -> Self {
        let handle = unsafe { ffi::nk_shape_create_plane(normal.x, normal.y, normal.z, constant) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create plane shape"),
            shape_type: CollisionShapeType::StaticPlane,
        }
    }

    pub fn new_convex_hull(points: &[Vec3]) -> Self {
        let flat_points: Vec<Real> = points.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        let handle = unsafe {
            ffi::nk_shape_create_convex_hull(flat_points.as_ptr(), points.len() as i32)
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create convex hull shape"),
            shape_type: CollisionShapeType::ConvexHull,
        }
    }

    pub fn new_compound() -> Self {
        let handle = unsafe { ffi::nk_shape_create_compound() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create compound shape"),
            shape_type: CollisionShapeType::Compound,
        }
    }

    pub fn add_child_shape(&mut self, child: ShapeHandle, local_transform: Transform) {
        if self.shape_type != CollisionShapeType::Compound {
            panic!("Cannot add child shape to non-compound shape");
        }
        let nk_transform = nkTransform::from_core_transform(&local_transform);
        unsafe {
            ffi::nk_compound_add_child(self.handle.as_ptr(), child.handle(), &nk_transform);
        }
        std::mem::forget(child);
    }

    pub fn new_triangle_mesh(vertices: &[Vec3], indices: &[i32]) -> Self {
        let flat_vertices: Vec<Real> = vertices.iter().flat_map(|v| [v.x, v.y, v.z]).collect();
        let handle = unsafe {
            ffi::nk_shape_create_triangle_mesh(
                flat_vertices.as_ptr(),
                vertices.len() as i32,
                indices.as_ptr(),
                indices.len() as i32,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create triangle mesh shape"),
            shape_type: CollisionShapeType::TriangleMesh,
        }
    }

    pub fn new_heightfield(
        width: i32,
        length: i32,
        heights: &[Real],
        min_height: Real,
        max_height: Real,
        up_axis: i32,
    ) -> Self {
        let handle = unsafe {
            ffi::nk_shape_create_heightfield(
                width,
                length,
                heights.as_ptr(),
                min_height,
                max_height,
                up_axis,
            )
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create heightfield shape"),
            shape_type: CollisionShapeType::Heightfield,
        }
    }

    pub fn shape_type(&self) -> CollisionShapeType {
        self.shape_type
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn calculate_local_inertia(&self, mass: Real) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_shape_calculate_local_inertia(
                self.handle.as_ptr(),
                mass,
                &mut x,
                &mut y,
                &mut z,
            );
        }
        Vec3::new(x, y, z)
    }

    pub fn set_local_scaling(&mut self, scaling: Vec3) {
        unsafe {
            ffi::nk_shape_set_local_scaling(
                self.handle.as_ptr(),
                scaling.x,
                scaling.y,
                scaling.z,
            );
        }
    }

    pub fn get_local_scaling(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_shape_get_local_scaling(
                self.handle.as_ptr(),
                &mut x,
                &mut y,
                &mut z,
            );
        }
        Vec3::new(x, y, z)
    }

    pub fn set_margin(&mut self, margin: Real) {
        unsafe {
            ffi::nk_shape_set_margin(self.handle.as_ptr(), margin);
        }
    }

    pub fn get_margin(&self) -> Real {
        unsafe { ffi::nk_shape_get_margin(self.handle.as_ptr()) }
    }

    pub fn get_angular_motion_disc(&self) -> Real {
        unsafe { ffi::nk_shape_get_angular_motion_disc(self.handle.as_ptr()) }
    }

    pub fn get_contact_breaking_threshold(&self, default_threshold: Real) -> Real {
        unsafe { ffi::nk_shape_get_contact_breaking_threshold(self.handle.as_ptr(), default_threshold) }
    }

    pub fn get_box_half_extents(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_box_get_half_extents(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn get_sphere_radius(&self) -> Real {
        unsafe { ffi::nk_sphere_get_radius(self.handle.as_ptr()) }
    }

    pub fn get_capsule_radius(&self) -> Real {
        unsafe { ffi::nk_capsule_get_radius(self.handle.as_ptr()) }
    }

    pub fn get_capsule_half_height(&self) -> Real {
        unsafe { ffi::nk_capsule_get_half_height(self.handle.as_ptr()) }
    }

    pub fn get_cylinder_radius(&self) -> Real {
        unsafe { ffi::nk_cylinder_get_radius(self.handle.as_ptr()) }
    }

    pub fn get_cylinder_half_height(&self) -> Real {
        unsafe { ffi::nk_cylinder_get_half_height(self.handle.as_ptr()) }
    }

    pub fn get_cone_radius(&self) -> Real {
        unsafe { ffi::nk_cone_get_radius(self.handle.as_ptr()) }
    }

    pub fn get_cone_height(&self) -> Real {
        unsafe { ffi::nk_cone_get_height(self.handle.as_ptr()) }
    }
}

impl Drop for ShapeHandle {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_shape_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for ShapeHandle {}
unsafe impl Sync for ShapeHandle {}

pub struct CollisionShapeBuilder {
    shape: Option<CollisionShape>,
}

impl CollisionShapeBuilder {
    pub fn new() -> Self {
        Self { shape: None }
    }

    pub fn box_shape(mut self, half_extents: Vec3) -> Self {
        self.shape = Some(CollisionShape::Box { half_extents });
        self
    }

    pub fn sphere(mut self, radius: Real) -> Self {
        self.shape = Some(CollisionShape::Sphere { radius });
        self
    }

    pub fn capsule(mut self, radius: Real, height: Real) -> Self {
        self.shape = Some(CollisionShape::Capsule { radius, height });
        self
    }

    pub fn capsule_z(mut self, radius: Real, height: Real) -> Self {
        self.shape = Some(CollisionShape::CapsuleZ { radius, height });
        self
    }

    pub fn cylinder(mut self, half_extents: Vec3) -> Self {
        self.shape = Some(CollisionShape::Cylinder { half_extents });
        self
    }

    pub fn cone(mut self, radius: Real, height: Real) -> Self {
        self.shape = Some(CollisionShape::Cone { radius, height });
        self
    }

    pub fn plane(mut self, normal: Vec3, constant: Real) -> Self {
        self.shape = Some(CollisionShape::Plane { normal, constant });
        self
    }

    pub fn convex_hull(mut self, points: Vec<Vec3>) -> Self {
        self.shape = Some(CollisionShape::ConvexHull { points });
        self
    }

    pub fn triangle_mesh(mut self, vertices: Vec<Vec3>, indices: Vec<i32>) -> Self {
        let flat_vertices: Vec<Real> = vertices.iter().flat_map(|v| [v.x, v.y, v.z]).collect();
        self.shape = Some(CollisionShape::TriangleMesh { 
            vertices: flat_vertices, 
            indices 
        });
        self
    }

    pub fn heightfield(
        mut self, 
        width: i32, 
        length: i32, 
        heights: Vec<Real>,
        min_height: Real,
        max_height: Real,
        up_axis: i32,
    ) -> Self {
        self.shape = Some(CollisionShape::Heightfield {
            width,
            length,
            heights,
            min_height,
            max_height,
            up_axis,
        });
        self
    }

    pub fn build(self) -> Option<ShapeHandle> {
        match self.shape? {
            CollisionShape::Box { half_extents } => Some(ShapeHandle::new_box(half_extents)),
            CollisionShape::Sphere { radius } => Some(ShapeHandle::new_sphere(radius)),
            CollisionShape::Capsule { radius, height } => Some(ShapeHandle::new_capsule(radius, height)),
            CollisionShape::CapsuleZ { radius, height } => Some(ShapeHandle::new_capsule_z(radius, height)),
            CollisionShape::Cylinder { half_extents } => Some(ShapeHandle::new_cylinder(half_extents)),
            CollisionShape::Cone { radius, height } => Some(ShapeHandle::new_cone(radius, height)),
            CollisionShape::Plane { normal, constant } => Some(ShapeHandle::new_plane(normal, constant)),
            CollisionShape::ConvexHull { points } => Some(ShapeHandle::new_convex_hull(&points)),
            CollisionShape::Compound { .. } => Some(ShapeHandle::new_compound()),
            CollisionShape::TriangleMesh { vertices, indices } => {
                let vertices_vec: Vec<Vec3> = vertices.chunks(3)
                    .map(|chunk| Vec3::new(chunk[0], chunk[1], chunk[2]))
                    .collect();
                Some(ShapeHandle::new_triangle_mesh(&vertices_vec, &indices))
            }
            CollisionShape::Heightfield { width, length, heights, min_height, max_height, up_axis } => {
                Some(ShapeHandle::new_heightfield(width, length, &heights, min_height, max_height, up_axis))
            }
        }
    }
}

impl Default for CollisionShapeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
