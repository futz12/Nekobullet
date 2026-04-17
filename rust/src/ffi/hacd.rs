#![allow(non_snake_case)]

use std::ffi::c_void;

pub type nkHACDHandle = *mut c_void;

#[repr(C)]
pub struct nkHACDParams {
    pub maxHullCount: i32,
    pub maxVerticesPerHull: i32,
    pub concavity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub ccConnectDist: f64,
    pub addFacesPoints: i32,
    pub addExtraDistPoints: i32,
    pub addNeighboursDistPoints: i32,
}

extern "C" {
    pub fn nk_hacd_create() -> nkHACDHandle;
    pub fn nk_hacd_destroy(hacd: nkHACDHandle);

    pub fn nk_hacd_set_params(hacd: nkHACDHandle, params: *const nkHACDParams);
    pub fn nk_hacd_get_params(hacd: nkHACDHandle, outParams: *mut nkHACDParams);

    pub fn nk_hacd_set_points(hacd: nkHACDHandle, points: *const f64, numPoints: i32) -> i32;
    pub fn nk_hacd_set_triangles(hacd: nkHACDHandle, triangles: *const i32, numTriangles: i32) -> i32;

    pub fn nk_hacd_compute(hacd: nkHACDHandle) -> i32;

    pub fn nk_hacd_get_num_hulls(hacd: nkHACDHandle) -> i32;
    pub fn nk_hacd_get_hull_points_count(hacd: nkHACDHandle, hullIndex: i32) -> i32;
    pub fn nk_hacd_get_hull_triangles_count(hacd: nkHACDHandle, hullIndex: i32) -> i32;
    pub fn nk_hacd_get_hull(
        hacd: nkHACDHandle,
        hullIndex: i32,
        outPoints: *mut f64,
        outNumPoints: *mut i32,
        outTriangles: *mut i32,
        outNumTriangles: *mut i32,
    ) -> i32;
}
