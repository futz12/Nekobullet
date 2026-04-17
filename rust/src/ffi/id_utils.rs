use super::types::{nkMultiBodyHandle, nkReal};

pub type nkMultiBodyNameMapHandle = *mut std::ffi::c_void;
pub type nkMultiBodyTreeCreatorHandle = *mut std::ffi::c_void;
pub type nkUser2InternalIndexHandle = *mut std::ffi::c_void;

extern "C" {
    pub fn nk_multibody_name_map_create() -> nkMultiBodyNameMapHandle;
    pub fn nk_multibody_name_map_destroy(map: nkMultiBodyNameMapHandle);

    pub fn nk_multibody_name_map_add_body(
        map: nkMultiBodyNameMapHandle,
        index: i32,
        name: *const std::ffi::c_char,
    ) -> i32;
    pub fn nk_multibody_name_map_add_joint(
        map: nkMultiBodyNameMapHandle,
        index: i32,
        name: *const std::ffi::c_char,
    ) -> i32;

    pub fn nk_multibody_name_map_get_body_name(
        map: nkMultiBodyNameMapHandle,
        index: i32,
        out_name: *mut std::ffi::c_char,
        max_len: i32,
    ) -> i32;
    pub fn nk_multibody_name_map_get_joint_name(
        map: nkMultiBodyNameMapHandle,
        index: i32,
        out_name: *mut std::ffi::c_char,
        max_len: i32,
    ) -> i32;

    pub fn nk_multibody_name_map_get_body_index(
        map: nkMultiBodyNameMapHandle,
        name: *const std::ffi::c_char,
        out_index: *mut i32,
    ) -> i32;
    pub fn nk_multibody_name_map_get_joint_index(
        map: nkMultiBodyNameMapHandle,
        name: *const std::ffi::c_char,
        out_index: *mut i32,
    ) -> i32;

    pub fn nk_multibody_tree_creator_from_bt_multibody(
        btmb: nkMultiBodyHandle,
        verbose: i32,
    ) -> nkMultiBodyTreeCreatorHandle;
    pub fn nk_multibody_tree_creator_destroy(creator: nkMultiBodyTreeCreatorHandle);

    pub fn nk_multibody_create_from_creator(creator: nkMultiBodyTreeCreatorHandle) -> nkMultiBodyHandle;

    pub fn nk_clone_tree_creator_create(reference: nkMultiBodyHandle) -> nkMultiBodyTreeCreatorHandle;
    pub fn nk_clone_tree_creator_destroy(creator: nkMultiBodyTreeCreatorHandle);

    pub fn nk_multibody_clone(reference: nkMultiBodyHandle) -> nkMultiBodyHandle;

    pub fn nk_simple_tree_creator_create(num_bodies: i32) -> nkMultiBodyTreeCreatorHandle;
    pub fn nk_simple_tree_creator_destroy(creator: nkMultiBodyTreeCreatorHandle);

    pub fn nk_user2internal_index_create() -> nkUser2InternalIndexHandle;
    pub fn nk_user2internal_index_destroy(handle: nkUser2InternalIndexHandle);
    pub fn nk_user2internal_index_add_body(handle: nkUser2InternalIndexHandle, body: i32, parent: i32);
    pub fn nk_user2internal_index_build_mapping(handle: nkUser2InternalIndexHandle) -> i32;
    pub fn nk_user2internal_index_user2internal(handle: nkUser2InternalIndexHandle, user: i32, out_internal: *mut i32) -> i32;
    pub fn nk_user2internal_index_internal2user(handle: nkUser2InternalIndexHandle, internal: i32, out_user: *mut i32) -> i32;

    pub fn nk_id_random_init();
    pub fn nk_id_random_init_with_seed(seed: std::ffi::c_uint);
    pub fn nk_id_random_int(low: i32, high: i32) -> i32;
    pub fn nk_id_random_float(low: nkReal, high: nkReal) -> nkReal;
    pub fn nk_id_random_mass() -> nkReal;
    pub fn nk_id_random_inertia_principal(out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_id_random_axis(out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
}
