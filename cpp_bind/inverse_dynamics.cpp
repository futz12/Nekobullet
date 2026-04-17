#include "inverse_dynamics.h"
#include "BulletInverseDynamics/MultiBodyTree.hpp"
#include "BulletInverseDynamics/IDMath.hpp"

nkMultiBodyHandle nk_multibody_create()
{
    btInverseDynamics::MultiBodyTree* tree = new btInverseDynamics::MultiBodyTree();
    return static_cast<nkMultiBodyHandle>(tree);
}

void nk_multibody_destroy(nkMultiBodyHandle multibody)
{
    if (!multibody) return;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    delete tree;
}

int nk_multibody_add_body(nkMultiBodyHandle multibody,
    int body_index, int parent_index, int joint_type,
    nkReal parent_r_x, nkReal parent_r_y, nkReal parent_r_z,
    nkReal body_T_parent_00, nkReal body_T_parent_01, nkReal body_T_parent_02,
    nkReal body_T_parent_10, nkReal body_T_parent_11, nkReal body_T_parent_12,
    nkReal body_T_parent_20, nkReal body_T_parent_21, nkReal body_T_parent_22,
    nkReal axis_x, nkReal axis_y, nkReal axis_z,
    nkReal mass,
    nkReal com_x, nkReal com_y, nkReal com_z,
    nkReal inertia_xx, nkReal inertia_xy, nkReal inertia_xz,
    nkReal inertia_yy, nkReal inertia_yz, nkReal inertia_zz)
{
    if (!multibody) return -1;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);

    btInverseDynamics::JointType jt;
    switch (joint_type)
    {
        case NK_JOINT_TYPE_FIXED:
            jt = btInverseDynamics::FIXED;
            break;
        case NK_JOINT_TYPE_REVOLUTE:
            jt = btInverseDynamics::REVOLUTE;
            break;
        case NK_JOINT_TYPE_PRISMATIC:
            jt = btInverseDynamics::PRISMATIC;
            break;
        case NK_JOINT_TYPE_FLOATING:
            jt = btInverseDynamics::FLOATING;
            break;
        case NK_JOINT_TYPE_SPHERICAL:
            jt = btInverseDynamics::SPHERICAL;
            break;
        default:
            return -1;
    }

    btInverseDynamics::vec3 parent_r_parent_body_ref;
    parent_r_parent_body_ref = btVector3(parent_r_x, parent_r_y, parent_r_z);

    btInverseDynamics::mat33 body_T_parent_ref;
    body_T_parent_ref[0][0] = body_T_parent_00;
    body_T_parent_ref[0][1] = body_T_parent_01;
    body_T_parent_ref[0][2] = body_T_parent_02;
    body_T_parent_ref[1][0] = body_T_parent_10;
    body_T_parent_ref[1][1] = body_T_parent_11;
    body_T_parent_ref[1][2] = body_T_parent_12;
    body_T_parent_ref[2][0] = body_T_parent_20;
    body_T_parent_ref[2][1] = body_T_parent_21;
    body_T_parent_ref[2][2] = body_T_parent_22;

    btInverseDynamics::vec3 body_axis_of_motion;
    body_axis_of_motion = btVector3(axis_x, axis_y, axis_z);

    btInverseDynamics::vec3 body_r_body_com;
    body_r_body_com = btVector3(com_x, com_y, com_z);

    btInverseDynamics::mat33 body_I_body;
    body_I_body[0][0] = inertia_xx;
    body_I_body[0][1] = inertia_xy;
    body_I_body[0][2] = inertia_xz;
    body_I_body[1][0] = inertia_xy;
    body_I_body[1][1] = inertia_yy;
    body_I_body[1][2] = inertia_yz;
    body_I_body[2][0] = inertia_xz;
    body_I_body[2][1] = inertia_yz;
    body_I_body[2][2] = inertia_zz;

    return tree->addBody(body_index, parent_index, jt,
        parent_r_parent_body_ref, body_T_parent_ref,
        body_axis_of_motion, mass, body_r_body_com,
        body_I_body, 0, nullptr);
}

int nk_multibody_finalize(nkMultiBodyHandle multibody)
{
    if (!multibody) return -1;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    return tree->finalize();
}

int nk_multibody_calculate_inverse_dynamics(nkMultiBodyHandle multibody,
    const nkReal* q, const nkReal* u, const nkReal* dot_u,
    nkReal* joint_forces, int num_dofs)
{
    if (!multibody || !q || !u || !dot_u || !joint_forces) return -1;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);

    btInverseDynamics::vecx q_vec(num_dofs);
    btInverseDynamics::vecx u_vec(num_dofs);
    btInverseDynamics::vecx dot_u_vec(num_dofs);
    btInverseDynamics::vecx joint_forces_vec(num_dofs);

    for (int i = 0; i < num_dofs; i++)
    {
        q_vec(i) = q[i];
        u_vec(i) = u[i];
        dot_u_vec(i) = dot_u[i];
    }

    int result = tree->calculateInverseDynamics(q_vec, u_vec, dot_u_vec, &joint_forces_vec);

    for (int i = 0; i < num_dofs; i++)
    {
        joint_forces[i] = joint_forces_vec(i);
    }

    return result;
}

int nk_multibody_calculate_mass_matrix(nkMultiBodyHandle multibody,
    const nkReal* q, int num_q,
    nkReal* mass_matrix, int initialize_matrix, int set_lower_triangular)
{
    if (!multibody || !q || !mass_matrix) return -1;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);

    btInverseDynamics::vecx q_vec(num_q);
    for (int i = 0; i < num_q; i++)
    {
        q_vec(i) = q[i];
    }

    btInverseDynamics::matxx mass_mat(num_q, num_q);

    int result = tree->calculateMassMatrix(q_vec, true,
        initialize_matrix != 0, set_lower_triangular != 0, &mass_mat);

    for (int i = 0; i < num_q; i++)
    {
        for (int j = 0; j < num_q; j++)
        {
            mass_matrix[i * num_q + j] = mass_mat(i, j);
        }
    }

    return result;
}

int nk_multibody_get_num_bodies(nkMultiBodyHandle multibody)
{
    if (!multibody) return 0;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    return tree->numBodies();
}

int nk_multibody_get_num_dofs(nkMultiBodyHandle multibody)
{
    if (!multibody) return 0;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    return tree->numDoFs();
}

void nk_multibody_set_gravity(nkMultiBodyHandle multibody, nkReal x, nkReal y, nkReal z)
{
    if (!multibody) return;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    btInverseDynamics::vec3 gravity;
    gravity = btVector3(x, y, z);
    tree->setGravityInWorldFrame(gravity);
}

void nk_multibody_set_accept_invalid_mass(nkMultiBodyHandle multibody, int accept)
{
    if (!multibody) return;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    tree->setAcceptInvalidMassParameters(accept != 0);
}

void nk_multibody_print_tree(nkMultiBodyHandle multibody)
{
    if (!multibody) return;
    btInverseDynamics::MultiBodyTree* tree = static_cast<btInverseDynamics::MultiBodyTree*>(multibody);
    tree->printTree();
}
