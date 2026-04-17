
pub type nkMLCPSolverHandle = *mut std::ffi::c_void;
pub type nkDantzigSolverHandle = *mut std::ffi::c_void;

extern "C" {
    pub fn nk_dantzig_solver_create() -> nkDantzigSolverHandle;
    pub fn nk_dantzig_solver_destroy(solver: nkDantzigSolverHandle);

    pub fn nk_mlcp_solver_create(mlcp_interface: nkDantzigSolverHandle) -> nkMLCPSolverHandle;
    pub fn nk_mlcp_solver_destroy(solver: nkMLCPSolverHandle);

    pub fn nk_mlcp_solver_get_num_fallbacks(solver: nkMLCPSolverHandle) -> i32;
    pub fn nk_mlcp_solver_set_num_fallbacks(solver: nkMLCPSolverHandle, num: i32);

    pub fn nk_mlcp_solver_create_default() -> nkMLCPSolverHandle;
}
