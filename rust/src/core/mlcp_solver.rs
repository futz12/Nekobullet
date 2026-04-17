use std::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;

pub struct DantzigSolver {
    handle: NonNull<c_void>,
}

impl DantzigSolver {
    pub fn new() -> Self {
        let handle = unsafe { ffi::nk_dantzig_solver_create() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create DantzigSolver"),
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Default for DantzigSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DantzigSolver {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_dantzig_solver_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for DantzigSolver {}
unsafe impl Sync for DantzigSolver {}

pub struct MLCPSolver {
    handle: NonNull<c_void>,
    owns_interface: bool,
    interface: Option<NonNull<c_void>>,
}

impl MLCPSolver {
    pub fn new(interface: &DantzigSolver) -> Self {
        let handle = unsafe { ffi::nk_mlcp_solver_create(interface.handle()) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create MLCPSolver"),
            owns_interface: false,
            interface: None,
        }
    }

    pub fn new_default() -> Self {
        let handle = unsafe { ffi::nk_mlcp_solver_create_default() };
        Self {
            handle: NonNull::new(handle).expect("Failed to create MLCPSolver"),
            owns_interface: true,
            interface: None,
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn num_fallbacks(&self) -> i32 {
        unsafe { ffi::nk_mlcp_solver_get_num_fallbacks(self.handle.as_ptr()) }
    }

    pub fn set_num_fallbacks(&self, num: i32) {
        unsafe {
            ffi::nk_mlcp_solver_set_num_fallbacks(self.handle.as_ptr(), num);
        }
    }
}

impl Default for MLCPSolver {
    fn default() -> Self {
        Self::new_default()
    }
}

impl Drop for MLCPSolver {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_mlcp_solver_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for MLCPSolver {}
unsafe impl Sync for MLCPSolver {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dantzig_solver_create() {
        let solver = DantzigSolver::new();
        assert!(!solver.handle().is_null());
    }

    #[test]
    fn test_mlcp_solver_create_default() {
        let solver = MLCPSolver::new_default();
        assert!(!solver.handle().is_null());
    }

    #[test]
    fn test_mlcp_solver_with_dantzig() {
        let dantzig = DantzigSolver::new();
        let solver = MLCPSolver::new(&dantzig);
        assert!(!solver.handle().is_null());
    }

    #[test]
    fn test_mlcp_solver_fallbacks() {
        let solver = MLCPSolver::new_default();
        solver.set_num_fallbacks(5);
        assert_eq!(solver.num_fallbacks(), 5);
    }
}
