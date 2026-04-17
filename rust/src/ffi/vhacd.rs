#![allow(non_snake_case)]

pub type nkVHACDHandle = *mut std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct nkVHACDParams {
    pub resolution: u32,
    pub depth: i32,
    pub concavity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub maxVerticesPerHull: u32,
    pub minVolumePerCH: f64,
    pub planeDownsampling: i32,
    pub convexhullDownsampling: i32,
    pub pca: i32,
    pub mode: i32,
    pub convexhullApproximation: i32,
}

impl Default for nkVHACDParams {
    fn default() -> Self {
        Self {
            resolution: 100000,
            depth: 20,
            concavity: 0.0025,
            alpha: 0.05,
            beta: 0.05,
            gamma: 0.00125,
            maxVerticesPerHull: 64,
            minVolumePerCH: 0.0001,
            planeDownsampling: 4,
            convexhullDownsampling: 4,
            pca: 0,
            mode: 0,
            convexhullApproximation: 1,
        }
    }
}

extern "C" {
    pub fn nk_vhacd_create() -> nkVHACDHandle;
    pub fn nk_vhacd_destroy(vhacd: nkVHACDHandle);
    
    pub fn nk_vhacd_compute(
        vhacd: nkVHACDHandle,
        points: *const f64,
        numPoints: i32,
        triangles: *const i32,
        numTriangles: i32,
        params: *const nkVHACDParams,
    ) -> i32;
    
    pub fn nk_vhacd_cancel(vhacd: nkVHACDHandle);
    
    pub fn nk_vhacd_get_num_hulls(vhacd: nkVHACDHandle) -> i32;
    
    pub fn nk_vhacd_get_hull(
        vhacd: nkVHACDHandle,
        hullIndex: i32,
        outPoints: *mut f64,
        outNumPoints: *mut i32,
        outTriangles: *mut i32,
        outNumTriangles: *mut i32,
    ) -> i32;
    
    pub fn nk_vhacd_clean(vhacd: nkVHACDHandle);
}
