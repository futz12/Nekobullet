use std::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VHACDParams {
    pub resolution: u32,
    pub depth: i32,
    pub concavity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub max_vertices_per_hull: u32,
    pub min_volume_per_ch: f64,
    pub plane_downsampling: i32,
    pub convexhull_downsampling: i32,
    pub pca: bool,
    pub mode: i32,
    pub convexhull_approximation: bool,
}

impl Default for VHACDParams {
    fn default() -> Self {
        Self {
            resolution: 100000,
            depth: 20,
            concavity: 0.0025,
            alpha: 0.05,
            beta: 0.05,
            gamma: 0.00125,
            max_vertices_per_hull: 64,
            min_volume_per_ch: 0.0001,
            plane_downsampling: 4,
            convexhull_downsampling: 4,
            pca: false,
            mode: 0,
            convexhull_approximation: true,
        }
    }
}

impl From<VHACDParams> for ffi::nkVHACDParams {
    fn from(params: VHACDParams) -> Self {
        Self {
            resolution: params.resolution,
            depth: params.depth,
            concavity: params.concavity,
            alpha: params.alpha,
            beta: params.beta,
            gamma: params.gamma,
            maxVerticesPerHull: params.max_vertices_per_hull,
            minVolumePerCH: params.min_volume_per_ch,
            planeDownsampling: params.plane_downsampling,
            convexhullDownsampling: params.convexhull_downsampling,
            pca: if params.pca { 1 } else { 0 },
            mode: params.mode,
            convexhullApproximation: if params.convexhull_approximation { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn get_point(&self, index: i32) -> Option<[f64; 3]> {
        if index < 0 || index >= self.num_points() {
            return None;
        }
        let i = (index * 3) as usize;
        Some([self.points[i], self.points[i + 1], self.points[i + 2]])
    }

    pub fn get_triangle(&self, index: i32) -> Option<[i32; 3]> {
        if index < 0 || index >= self.num_triangles() {
            return None;
        }
        let i = (index * 3) as usize;
        Some([self.triangles[i], self.triangles[i + 1], self.triangles[i + 2]])
    }
}

pub struct VHACD {
    handle: NonNull<c_void>,
}

impl VHACD {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_vhacd_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create VHACD instance"),
        }
    }

    pub fn compute(&self, points: &[f64], triangles: &[i32], params: &VHACDParams) -> bool {
        let num_points = (points.len() / 3) as i32;
        let num_triangles = (triangles.len() / 3) as i32;
        let ffi_params = ffi::nkVHACDParams::from(*params);

        let result = unsafe {
            ffi::nk_vhacd_compute(
                self.handle.as_ptr(),
                points.as_ptr(),
                num_points,
                triangles.as_ptr(),
                num_triangles,
                &ffi_params,
            )
        };

        result != 0
    }

    pub fn cancel(&self) {
        unsafe {
            ffi::nk_vhacd_cancel(self.handle.as_ptr());
        }
    }

    pub fn get_num_hulls(&self) -> i32 {
        unsafe { ffi::nk_vhacd_get_num_hulls(self.handle.as_ptr()) }
    }

    pub fn get_hull(&self, index: i32) -> Option<ConvexHull> {
        if index < 0 || index >= self.get_num_hulls() {
            return None;
        }

        let mut num_points: i32 = 0;
        let mut num_triangles: i32 = 0;

        let has_data = unsafe {
            ffi::nk_vhacd_get_hull(
                self.handle.as_ptr(),
                index,
                std::ptr::null_mut(),
                &mut num_points,
                std::ptr::null_mut(),
                &mut num_triangles,
            )
        };

        if has_data == 0 || num_points == 0 {
            return None;
        }

        let mut points = vec![0.0f64; (num_points * 3) as usize];
        let mut triangles = vec![0i32; (num_triangles * 3) as usize];

        let success = unsafe {
            ffi::nk_vhacd_get_hull(
                self.handle.as_ptr(),
                index,
                points.as_mut_ptr(),
                &mut num_points,
                triangles.as_mut_ptr(),
                &mut num_triangles,
            )
        };

        if success != 0 {
            Some(ConvexHull { points, triangles })
        } else {
            None
        }
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

    pub fn clean(&self) {
        unsafe {
            ffi::nk_vhacd_clean(self.handle.as_ptr());
        }
    }
}

impl Default for VHACD {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VHACD {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_vhacd_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for VHACD {}
unsafe impl Sync for VHACD {}
