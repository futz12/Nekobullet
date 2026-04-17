#ifndef NEKOBULLET_CHARACTER_HPP
#define NEKOBULLET_CHARACTER_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkCharacterHandle nk_character_create(nkGhostObjectHandle ghost, nkShapeHandle convex_shape, 
    nkReal step_height, nkReal up_x, nkReal up_y, nkReal up_z);
void nk_character_destroy(nkCharacterHandle character);

void nk_character_set_walk_direction(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_set_velocity_for_time(nkCharacterHandle character, nkReal vx, nkReal vy, nkReal vz, nkReal time_interval);
void nk_character_set_linear_velocity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_get_linear_velocity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_character_set_angular_velocity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_get_angular_velocity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_character_jump(nkCharacterHandle character, nkReal vx, nkReal vy, nkReal vz);
int nk_character_can_jump(nkCharacterHandle character);
int nk_character_on_ground(nkCharacterHandle character);

void nk_character_set_jump_speed(nkCharacterHandle character, nkReal speed);
nkReal nk_character_get_jump_speed(nkCharacterHandle character);
void nk_character_set_fall_speed(nkCharacterHandle character, nkReal speed);
nkReal nk_character_get_fall_speed(nkCharacterHandle character);
void nk_character_set_max_jump_height(nkCharacterHandle character, nkReal height);
void nk_character_set_max_slope(nkCharacterHandle character, nkReal slope_radians);
nkReal nk_character_get_max_slope(nkCharacterHandle character);
void nk_character_set_step_height(nkCharacterHandle character, nkReal height);
nkReal nk_character_get_step_height(nkCharacterHandle character);
void nk_character_set_gravity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_get_gravity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_character_set_up(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_get_up(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_character_warp(nkCharacterHandle character, nkReal x, nkReal y, nkReal z);
void nk_character_reset(nkCharacterHandle character);
void nk_character_set_linear_damping(nkCharacterHandle character, nkReal damping);
nkReal nk_character_get_linear_damping(nkCharacterHandle character);
void nk_character_set_angular_damping(nkCharacterHandle character, nkReal damping);
nkReal nk_character_get_angular_damping(nkCharacterHandle character);
void nk_character_set_max_penetration_depth(nkCharacterHandle character, nkReal depth);
nkReal nk_character_get_max_penetration_depth(nkCharacterHandle character);
nkGhostObjectHandle nk_character_get_ghost_object(nkCharacterHandle character);

#ifdef __cplusplus
}
#endif

#endif
