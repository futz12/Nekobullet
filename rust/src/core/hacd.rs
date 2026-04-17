use std::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;

pub struct HACDParams {
    pub max_hull_count: i32,
    pub max_vertices_per_hull: i32,
    pub concavity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub cc_connect_dist: f64,
    pub add_faces_points: bool,
    pub add_extra_dist_points: bool,
    pub add_neighbours_dist_points: bool,
}

impl Default for HACDParams {
    fn default() -> Self {
        Self {
            max_hull_count: 32,
            max_vertices_per_hull: 64,
            concavity: 0.0025,
            alpha: 0.05,
            beta: 0.05,
            cc_connect_dist: 30.0,
            add_faces_points: true,
            add_extra_dist_points: false,
            add_neighbours_dist_points: false,
        }
    }
}

impl HACDParams {
    fn to_ffi(&self) -> ffi::nkHACDParams {
        ffi::nkHACDParams {
            maxHullCount: self.max_hull_count,
            maxVerticesPerHull: self.max_vertices_per_hull,
            concavity: self.concavity,
            alpha: self.alpha,
            beta: self.beta,
            ccConnectDist: self.cc_connect_dist,
            addFacesPoints: if self.add_faces_points { 1 } else { 0 },
            addExtraDistPoints: if self.add_extra_dist_points { 1 } else { 0 },
            addNeighboursDistPoints: if self.add_neighbours_dist_points { 1 } else { 0 },
        }
    }
}

pub struct ConvexHull {
    pub points: Vec<f64>,
    pub triangles: Vec<i32>,
}

impl ConvexHull {
    pub fn num_points(&self) -> i32 {
        (self.points.len() / 3) as i32
    }

    pub fn num_triangles(&self) -> i32 {
        (self.triangles.len() / 3) as i32
    }

    pub fn get_point(&self, index: i32) -> [f64; 3] {
        let base = (index * 3) as usize;
        [self.points[base], self.points[base + 1], self.points[base + 2]]
    }

    pub fn get_triangle(&self, index: i32) -> [i32; 3] {
        let base = (index * 3) as usize;
        [self.triangles[base], self.triangles[base + 1], self.triangles[base + 2]]
    }
}

pub struct HACD {
    handle: NonNull<c_void>,
}

impl HACD {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_hacd_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create HACD instance"),
        }
    }

    pub fn set_params(&self, params: &HACDParams) {
        let ffi_params = params.to_ffi();
        unsafe {
            ffi::nk_hacd_set_params(self.handle.as_ptr(), &ffi_params);
        }
    }

    pub fn get_params(&self) -> HACDParams {
        let mut ffi_params: ffi::nkHACDParams = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::nk_hacd_get_params(self.handle.as_ptr(), &mut ffi_params);
        }
        HACDParams {
            max_hull_count: ffi_params.maxHullCount,
            max_vertices_per_hull: ffi_params.maxVerticesPerHull,
            concavity: ffi_params.concavity,
            alpha: ffi_params.alpha,
            beta: ffi_params.beta,
            cc_connect_dist: ffi_params.ccConnectDist,
            add_faces_points: ffi_params.addFacesPoints != 0,
            add_extra_dist_points: ffi_params.addExtraDistPoints != 0,
            add_neighbours_dist_points: ffi_params.addNeighboursDistPoints != 0,
        }
    }

    pub fn set_mesh(&self, points: &[f64], triangles: &[i32]) -> Result<(), String> {
        let num_points = (points.len() / 3) as i32;
        let num_triangles = (triangles.len() / 3) as i32;

        let points_result = unsafe {
            ffi::nk_hacd_set_points(self.handle.as_ptr(), points.as_ptr(), num_points)
        };

        if points_result != 0 {
            return Err("Failed to set points".to_string());
        }

        let triangles_result = unsafe {
            ffi::nk_hacd_set_triangles(self.handle.as_ptr(), triangles.as_ptr(), num_triangles)
        };

        if triangles_result != 0 {
            return Err("Failed to set triangles".to_string());
        }

        Ok(())
    }

    pub fn compute(&self) -> Result<(), String> {
        let result = unsafe { ffi::nk_hacd_compute(self.handle.as_ptr()) };
        if result != 0 {
            Err("HACD computation failed".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_num_hulls(&self) -> i32 {
        unsafe { ffi::nk_hacd_get_num_hulls(self.handle.as_ptr()) }
    }

    pub fn get_hull(&self, hull_index: i32) -> Option<ConvexHull> {
        let num_points = unsafe {
            ffi::nk_hacd_get_hull_points_count(self.handle.as_ptr(), hull_index)
        };
        let num_triangles = unsafe {
            ffi::nk_hacd_get_hull_triangles_count(self.handle.as_ptr(), hull_index)
        };

        if num_points <= 0 || num_triangles <= 0 {
            return None;
        }

        let mut points = vec![0.0f64; (num_points * 3) as usize];
        let mut triangles = vec![0i32; (num_triangles * 3) as usize];
        let mut out_num_points: i32 = 0;
        let mut out_num_triangles: i32 = 0;

        let result = unsafe {
            ffi::nk_hacd_get_hull(
                self.handle.as_ptr(),
                hull_index,
                points.as_mut_ptr(),
                &mut out_num_points,
                triangles.as_mut_ptr(),
                &mut out_num_triangles,
            )
        };

        if result != 0 {
            return None;
        }

        points.truncate((out_num_points * 3) as usize);
        triangles.truncate((out_num_triangles * 3) as usize);

        Some(ConvexHull { points, triangles })
    }

    pub fn get_hulls(&self) -> Vec<ConvexHull> {
        let num_hulls = self.get_num_hulls();
        let mut hulls = Vec::with_capacity(num_hulls as usize);

        for i in 0..num_hulls {
            if let Some(hull) = self.get_hull(i) {
                hulls.push(hull);
            }
        }

        hulls
    }
}

impl Default for HACD {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HACD {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_hacd_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for HACD {}
unsafe impl Sync for HACD {}
