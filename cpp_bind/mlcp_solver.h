#ifndef NEKOBULLET_MLCP_HPP
#define NEKOBULLET_MLCP_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkMLCPSolverHandle;
typedef void* nkDantzigSolverHandle;

nkDantzigSolverHandle nk_dantzig_solver_create();
void nk_dantzig_solver_destroy(nkDantzigSolverHandle solver);

nkMLCPSolverHandle nk_mlcp_solver_create(nkDantzigSolverHandle mlcp_interface);
void nk_mlcp_solver_destroy(nkMLCPSolverHandle solver);

int nk_mlcp_solver_get_num_fallbacks(nkMLCPSolverHandle solver);
void nk_mlcp_solver_set_num_fallbacks(nkMLCPSolverHandle solver, int num);

nkMLCPSolverHandle nk_mlcp_solver_create_default();

#ifdef __cplusplus
}
#endif

#endif
