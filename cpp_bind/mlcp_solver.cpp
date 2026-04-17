#include "mlcp_solver.h"
#include "BulletDynamics/MLCPSolvers/btMLCPSolver.h"
#include "BulletDynamics/MLCPSolvers/btDantzigSolver.h"

nkDantzigSolverHandle nk_dantzig_solver_create()
{
    btDantzigSolver* solver = new btDantzigSolver();
    return static_cast<nkDantzigSolverHandle>(solver);
}

void nk_dantzig_solver_destroy(nkDantzigSolverHandle solver)
{
    if (!solver) return;
    btDantzigSolver* dantzigSolver = static_cast<btDantzigSolver*>(solver);
    delete dantzigSolver;
}

nkMLCPSolverHandle nk_mlcp_solver_create(nkDantzigSolverHandle mlcp_interface)
{
    if (!mlcp_interface) return nullptr;
    btMLCPSolverInterface* interface = static_cast<btMLCPSolverInterface*>(mlcp_interface);
    btMLCPSolver* solver = new btMLCPSolver(interface);
    return static_cast<nkMLCPSolverHandle>(solver);
}

void nk_mlcp_solver_destroy(nkMLCPSolverHandle solver)
{
    if (!solver) return;
    btMLCPSolver* mlcpSolver = static_cast<btMLCPSolver*>(solver);
    delete mlcpSolver;
}

int nk_mlcp_solver_get_num_fallbacks(nkMLCPSolverHandle solver)
{
    if (!solver) return 0;
    btMLCPSolver* mlcpSolver = static_cast<btMLCPSolver*>(solver);
    return mlcpSolver->getNumFallbacks();
}

void nk_mlcp_solver_set_num_fallbacks(nkMLCPSolverHandle solver, int num)
{
    if (!solver) return;
    btMLCPSolver* mlcpSolver = static_cast<btMLCPSolver*>(solver);
    mlcpSolver->setNumFallbacks(num);
}

nkMLCPSolverHandle nk_mlcp_solver_create_default()
{
    btDantzigSolver* dantzig = new btDantzigSolver();
    btMLCPSolver* solver = new btMLCPSolver(dantzig);
    return static_cast<nkMLCPSolverHandle>(solver);
}
