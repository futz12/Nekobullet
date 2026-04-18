use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let is_wasm = target.contains("wasm32");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let nekobullet_root = manifest_dir.parent().unwrap();
    let src_dir = nekobullet_root.join("src");
    let cpp_bind_dir = nekobullet_root.join("cpp_bind");
    let wasm_std_dir = nekobullet_root.join("wasm_std");

    if is_wasm {
        build_wasm(&src_dir, &cpp_bind_dir, &wasm_std_dir, &out_dir);
    } else {
        build_native(&src_dir, &cpp_bind_dir, &out_dir, &target);
    }

    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("types.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("world.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("world.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("shape.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("shape.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("rigidbody.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("rigidbody.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("raytest.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("raytest.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("constraint.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("constraint.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("ghost.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("ghost.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("softbody.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("softbody.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("vehicle.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("vehicle.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("character.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("character.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("inverse_dynamics.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("inverse_dynamics.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("hacd.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("hacd.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("vhacd.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("vhacd.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("gimpact.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("gimpact.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("gimpact_decomp.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("gimpact_decomp.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("id_utils.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("id_utils.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("mlcp_solver.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("mlcp_solver.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("multibody_constraint.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("multibody_constraint.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("reduced_softbody.h").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("reduced_softbody.cpp").display());
    println!("cargo:rerun-if-changed={}", cpp_bind_dir.join("nekobullet.h").display());

    walk_dir_and_rerun(&src_dir);
    walk_dir_and_rerun(&wasm_std_dir);
}

fn walk_dir_and_rerun(dir: &Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir_and_rerun(&path);
            } else if let Some(ext) = path.extension() {
                if ext == "cpp" || ext == "h" || ext == "hpp" {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }
}

fn build_native(src_dir: &Path, cpp_bind_dir: &Path, out_dir: &Path, target: &str) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let nekobullet_root = manifest_dir.parent().unwrap();
    let extras_dir = nekobullet_root.join("Extras");

    let mut build = cc::Build::new();

    build
        .cpp(true)
        .include(src_dir)
        .include(cpp_bind_dir)
        .include(&extras_dir);

    let is_windows_msvc = target.contains("msvc");
    let is_windows = target.contains("windows");

    if is_windows_msvc {
        build.flag("/std:c++17");
        build.flag("/EHsc");
    } else {
        build.flag("-std=c++17");
        build.flag("-fPIC");
        if !is_windows {
            build.flag("-fvisibility=hidden");
        }
    }

    compile_bullet_core(&mut build, src_dir, is_windows_msvc, &extras_dir);
    compile_cpp_bind(&mut build, cpp_bind_dir);

    build.compile("nekobullet_native");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=nekobullet_native");

    if is_windows_msvc {
        println!("cargo:rustc-link-lib=dylib=user32");
    }
}

fn compile_bullet_core(build: &mut cc::Build, src_dir: &Path, is_msvc: bool, extras_dir: &Path) {
    let collision = env::var("CARGO_FEATURE_COLLISION").is_ok();
    let dynamics = env::var("CARGO_FEATURE_DYNAMICS").is_ok();
    let softbody = env::var("CARGO_FEATURE_SOFTBODY").is_ok();
    let vehicle = env::var("CARGO_FEATURE_VEHICLE").is_ok();
    let character = env::var("CARGO_FEATURE_CHARACTER").is_ok();
    let multibody = env::var("CARGO_FEATURE_MULTIBODY").is_ok();
    let inverse_dynamics = env::var("CARGO_FEATURE_INVERSE_DYNAMICS").is_ok();
    let parallel = env::var("CARGO_FEATURE_PARALLEL").is_ok();
    let hacd = env::var("CARGO_FEATURE_HACD").is_ok();
    let vhacd = env::var("CARGO_FEATURE_VHACD").is_ok();
    let gimpact = env::var("CARGO_FEATURE_GIMPACT").is_ok();

    compile_linearmath(build, src_dir);

    if collision || dynamics {
        compile_bullet_collision(build, src_dir);
    }

    if dynamics {
        compile_bullet_dynamics(build, src_dir, vehicle, character, multibody, parallel, inverse_dynamics);
    }

    if softbody {
        compile_bullet_softbody(build, src_dir);
    }

    if inverse_dynamics {
        compile_bullet_inverse_dynamics(build, src_dir);
    }

    if hacd {
        compile_hacd(build, extras_dir);
    }

    if vhacd {
        compile_vhacd(build, extras_dir);
    }

    if gimpact {
        compile_gimpact_utils(build, extras_dir);
    }

    if parallel && is_msvc {
        build.flag("/openmp");
    } else if parallel {
        build.flag("-fopenmp");
    }
}

fn compile_linearmath(build: &mut cc::Build, src_dir: &Path) {
    let linear_math = src_dir.join("LinearMath");
    build.include(&linear_math);

    let files = [
        "btAlignedAllocator.cpp",
        "btConvexHull.cpp",
        "btConvexHullComputer.cpp",
        "btGeometryUtil.cpp",
        "btPolarDecomposition.cpp",
        "btQuickprof.cpp",
        "btSerializer.cpp",
        "btSerializer64.cpp",
        "btThreads.cpp",
        "btVector3.cpp",
        "btReducedVector.cpp",
    ];

    for file in &files {
        build.file(linear_math.join(file));
    }

    let task_scheduler = linear_math.join("TaskScheduler");
    build.file(task_scheduler.join("btTaskScheduler.cpp"));

    if cfg!(windows) {
        build.file(task_scheduler.join("btThreadSupportWin32.cpp"));
    } else {
        build.file(task_scheduler.join("btThreadSupportPosix.cpp"));
    }
}

fn compile_bullet_collision(build: &mut cc::Build, src_dir: &Path) {
    let collision_dir = src_dir.join("BulletCollision");
    build.include(&collision_dir);

    let broadphase_files = [
        "btAxisSweep3.cpp",
        "btBroadphaseProxy.cpp",
        "btCollisionAlgorithm.cpp",
        "btDbvt.cpp",
        "btDbvtBroadphase.cpp",
        "btDispatcher.cpp",
        "btOverlappingPairCache.cpp",
        "btQuantizedBvh.cpp",
        "btSimpleBroadphase.cpp",
    ];

    let broadphase_dir = collision_dir.join("BroadphaseCollision");
    for file in &broadphase_files {
        build.file(broadphase_dir.join(file));
    }

    let dispatch_files = [
        "SphereTriangleDetector.cpp",
        "btActivatingCollisionAlgorithm.cpp",
        "btBox2dBox2dCollisionAlgorithm.cpp",
        "btBoxBoxCollisionAlgorithm.cpp",
        "btBoxBoxDetector.cpp",
        "btCollisionDispatcher.cpp",
        "btCollisionDispatcherMt.cpp",
        "btCollisionObject.cpp",
        "btCollisionWorld.cpp",
        "btCollisionWorldImporter.cpp",
        "btCompoundCollisionAlgorithm.cpp",
        "btCompoundCompoundCollisionAlgorithm.cpp",
        "btConvex2dConvex2dAlgorithm.cpp",
        "btConvexConcaveCollisionAlgorithm.cpp",
        "btConvexConvexAlgorithm.cpp",
        "btConvexPlaneCollisionAlgorithm.cpp",
        "btDefaultCollisionConfiguration.cpp",
        "btEmptyCollisionAlgorithm.cpp",
        "btGhostObject.cpp",
        "btHashedSimplePairCache.cpp",
        "btInternalEdgeUtility.cpp",
        "btManifoldResult.cpp",
        "btSimulationIslandManager.cpp",
        "btSphereBoxCollisionAlgorithm.cpp",
        "btSphereSphereCollisionAlgorithm.cpp",
        "btSphereTriangleCollisionAlgorithm.cpp",
        "btUnionFind.cpp",
    ];

    let dispatch_dir = collision_dir.join("CollisionDispatch");
    for file in &dispatch_files {
        build.file(dispatch_dir.join(file));
    }

    let shapes_files = [
        "btBox2dShape.cpp",
        "btBoxShape.cpp",
        "btBvhTriangleMeshShape.cpp",
        "btCapsuleShape.cpp",
        "btCollisionShape.cpp",
        "btCompoundShape.cpp",
        "btConcaveShape.cpp",
        "btConeShape.cpp",
        "btConvex2dShape.cpp",
        "btConvexHullShape.cpp",
        "btConvexInternalShape.cpp",
        "btConvexPointCloudShape.cpp",
        "btConvexPolyhedron.cpp",
        "btConvexShape.cpp",
        "btConvexTriangleMeshShape.cpp",
        "btCylinderShape.cpp",
        "btEmptyShape.cpp",
        "btHeightfieldTerrainShape.cpp",
        "btMiniSDF.cpp",
        "btMinkowskiSumShape.cpp",
        "btMultiSphereShape.cpp",
        "btMultimaterialTriangleMeshShape.cpp",
        "btOptimizedBvh.cpp",
        "btPolyhedralConvexShape.cpp",
        "btScaledBvhTriangleMeshShape.cpp",
        "btSdfCollisionShape.cpp",
        "btShapeHull.cpp",
        "btSphereShape.cpp",
        "btStaticPlaneShape.cpp",
        "btStridingMeshInterface.cpp",
        "btTetrahedronShape.cpp",
        "btTriangleBuffer.cpp",
        "btTriangleCallback.cpp",
        "btTriangleIndexVertexArray.cpp",
        "btTriangleIndexVertexMaterialArray.cpp",
        "btTriangleMesh.cpp",
        "btTriangleMeshShape.cpp",
        "btUniformScalingShape.cpp",
    ];

    let shapes_dir = collision_dir.join("CollisionShapes");
    for file in &shapes_files {
        build.file(shapes_dir.join(file));
    }

    let narrowphase_files = [
        "btContinuousConvexCollision.cpp",
        "btConvexCast.cpp",
        "btGjkConvexCast.cpp",
        "btGjkEpa2.cpp",
        "btGjkEpaPenetrationDepthSolver.cpp",
        "btGjkPairDetector.cpp",
        "btMinkowskiPenetrationDepthSolver.cpp",
        "btPersistentManifold.cpp",
        "btPolyhedralContactClipping.cpp",
        "btRaycastCallback.cpp",
        "btSubSimplexConvexCast.cpp",
        "btVoronoiSimplexSolver.cpp",
    ];

    let narrowphase_dir = collision_dir.join("NarrowPhaseCollision");
    for file in &narrowphase_files {
        build.file(narrowphase_dir.join(file));
    }

    let gimpact_files = [
        "btContactProcessing.cpp",
        "btGImpactBvh.cpp",
        "btGImpactCollisionAlgorithm.cpp",
        "btGImpactQuantizedBvh.cpp",
        "btGImpactShape.cpp",
        "btGenericPoolAllocator.cpp",
        "btTriangleShapeEx.cpp",
        "gim_box_set.cpp",
        "gim_contact.cpp",
        "gim_memory.cpp",
        "gim_tri_collision.cpp",
    ];

    let gimpact_dir = collision_dir.join("Gimpact");
    for file in &gimpact_files {
        build.file(gimpact_dir.join(file));
    }
}

fn compile_bullet_dynamics(
    build: &mut cc::Build,
    src_dir: &Path,
    vehicle: bool,
    character: bool,
    multibody: bool,
    parallel: bool,
    inverse_dynamics: bool,
) {
    let dynamics_dir = src_dir.join("BulletDynamics");
    build.include(&dynamics_dir);

    let constraint_files = [
        "btBatchedConstraints.cpp",
        "btConeTwistConstraint.cpp",
        "btContactConstraint.cpp",
        "btFixedConstraint.cpp",
        "btGearConstraint.cpp",
        "btGeneric6DofConstraint.cpp",
        "btGeneric6DofSpring2Constraint.cpp",
        "btGeneric6DofSpringConstraint.cpp",
        "btHinge2Constraint.cpp",
        "btHingeConstraint.cpp",
        "btNNCGConstraintSolver.cpp",
        "btPoint2PointConstraint.cpp",
        "btSequentialImpulseConstraintSolver.cpp",
        "btSliderConstraint.cpp",
        "btSolve2LinearConstraint.cpp",
        "btTypedConstraint.cpp",
        "btUniversalConstraint.cpp",
    ];

    let constraint_dir = dynamics_dir.join("ConstraintSolver");
    for file in &constraint_files {
        build.file(constraint_dir.join(file));
    }

    if parallel {
        build.file(constraint_dir.join("btSequentialImpulseConstraintSolverMt.cpp"));
    }

    let dynamics_files = [
        "btDiscreteDynamicsWorld.cpp",
        "btRigidBody.cpp",
        "btSimpleDynamicsWorld.cpp",
    ];

    let dynamics_subdir = dynamics_dir.join("Dynamics");
    for file in &dynamics_files {
        build.file(dynamics_subdir.join(file));
    }

    if parallel {
        build.file(dynamics_subdir.join("btDiscreteDynamicsWorldMt.cpp"));
        build.file(dynamics_subdir.join("btSimulationIslandManagerMt.cpp"));
    }

    if vehicle {
        let vehicle_files = [
            "btRaycastVehicle.cpp",
            "btWheelInfo.cpp",
        ];

        let vehicle_dir = dynamics_dir.join("Vehicle");
        for file in &vehicle_files {
            build.file(vehicle_dir.join(file));
        }
    }

    if character {
        let character_dir = dynamics_dir.join("Character");
        build.file(character_dir.join("btKinematicCharacterController.cpp"));
    }

    if multibody {
        let multibody_files = [
            "btMultiBody.cpp",
            "btMultiBodyConstraint.cpp",
            "btMultiBodyConstraintSolver.cpp",
            "btMultiBodyDynamicsWorld.cpp",
            "btMultiBodyFixedConstraint.cpp",
            "btMultiBodyGearConstraint.cpp",
            "btMultiBodyJointLimitConstraint.cpp",
            "btMultiBodyJointMotor.cpp",
            "btMultiBodyMLCPConstraintSolver.cpp",
            "btMultiBodyPoint2Point.cpp",
            "btMultiBodySliderConstraint.cpp",
            "btMultiBodySphericalJointLimit.cpp",
            "btMultiBodySphericalJointMotor.cpp",
        ];

        let multibody_dir = dynamics_dir.join("Featherstone");
        for file in &multibody_files {
            build.file(multibody_dir.join(file));
        }
    }

}

fn compile_bullet_softbody(build: &mut cc::Build, src_dir: &Path) {
    let softbody_dir = src_dir.join("BulletSoftBody");
    build.include(&softbody_dir);

    let files = [
        "btDefaultSoftBodySolver.cpp",
        "btDeformableBackwardEulerObjective.cpp",
        "btDeformableBodySolver.cpp",
        "btDeformableContactConstraint.cpp",
        "btDeformableContactProjection.cpp",
        "btDeformableMultiBodyConstraintSolver.cpp",
        "btDeformableMultiBodyDynamicsWorld.cpp",
        "btSoftBody.cpp",
        "btSoftBodyConcaveCollisionAlgorithm.cpp",
        "btSoftBodyHelpers.cpp",
        "btSoftBodyRigidBodyCollisionConfiguration.cpp",
        "btSoftMultiBodyDynamicsWorld.cpp",
        "btSoftRigidCollisionAlgorithm.cpp",
        "btSoftRigidDynamicsWorld.cpp",
        "btSoftSoftCollisionAlgorithm.cpp",
        "poly34.cpp",
    ];

    for file in &files {
        build.file(softbody_dir.join(file));
    }

    let reduced_dir = softbody_dir.join("BulletReducedDeformableBody");
    let reduced_files = [
        "btReducedDeformableBody.cpp",
        "btReducedDeformableBodyHelpers.cpp",
        "btReducedDeformableBodySolver.cpp",
        "btReducedDeformableContactConstraint.cpp",
    ];

    for file in &reduced_files {
        build.file(reduced_dir.join(file));
    }
}

fn compile_bullet_inverse_dynamics(build: &mut cc::Build, src_dir: &Path) {
    let inv_dynamics_dir = src_dir.join("BulletInverseDynamics");
    build.include(&inv_dynamics_dir);
    
    build.define("BT_USE_INVERSE_DYNAMICS_WITH_BULLET2", None);

    let files = [
        "IDMath.cpp",
        "MultiBodyTree.cpp",
    ];

    for file in &files {
        build.file(inv_dynamics_dir.join(file));
    }

    let details_dir = inv_dynamics_dir.join("details");
    let details_files = [
        "MultiBodyTreeImpl.cpp",
        "MultiBodyTreeInitCache.cpp",
    ];

    for file in &details_files {
        build.file(details_dir.join(file));
    }
}

fn compile_cpp_bind(build: &mut cc::Build, cpp_bind_dir: &Path) {
    let collision = env::var("CARGO_FEATURE_COLLISION").is_ok();
    let dynamics = env::var("CARGO_FEATURE_DYNAMICS").is_ok();
    let softbody = env::var("CARGO_FEATURE_SOFTBODY").is_ok();
    let vehicle = env::var("CARGO_FEATURE_VEHICLE").is_ok();
    let character = env::var("CARGO_FEATURE_CHARACTER").is_ok();
    let multibody = env::var("CARGO_FEATURE_MULTIBODY").is_ok();
    let inverse_dynamics = env::var("CARGO_FEATURE_INVERSE_DYNAMICS").is_ok();
    let hacd = env::var("CARGO_FEATURE_HACD").is_ok();
    let vhacd = env::var("CARGO_FEATURE_VHACD").is_ok();
    let gimpact = env::var("CARGO_FEATURE_GIMPACT").is_ok();

    let mut files = vec![];

    if collision || dynamics {
        files.extend_from_slice(&["world.cpp", "shape.cpp", "raytest.cpp", "ghost.cpp"]);
    }

    if dynamics {
        files.push("rigidbody.cpp");
        files.push("constraint.cpp");
    }

    if softbody {
        files.push("softbody.cpp");
        files.push("reduced_softbody.cpp");
    }

    if vehicle {
        files.push("vehicle.cpp");
    }

    if character {
        files.push("character.cpp");
    }

    if multibody || inverse_dynamics {
        files.push("multibody_constraint.cpp");
        files.push("mlcp_solver.cpp");
    }

    if inverse_dynamics {
        files.push("inverse_dynamics.cpp");
        files.push("id_utils.cpp");
    }
    if hacd {
        files.push("hacd.cpp");
    }
    if vhacd {
        files.push("vhacd.cpp");
    }
    if gimpact {
        files.push("gimpact.cpp");
        files.push("gimpact_decomp.cpp");
    }

    for file in &files {
        build.file(cpp_bind_dir.join(file));
    }
}

fn compile_hacd(build: &mut cc::Build, extras_dir: &Path) {
    let hacd_dir = extras_dir.join("HACD");
    build.include(&hacd_dir);

    let files = [
        "hacdHACD.cpp",
        "hacdICHull.cpp",
        "hacdManifoldMesh.cpp",
        "hacdGraph.cpp",
    ];

    for file in &files {
        build.file(hacd_dir.join(file));
    }
}

fn compile_vhacd(build: &mut cc::Build, extras_dir: &Path) {
    let vhacd_dir = extras_dir.join("VHACD");
    build.include(vhacd_dir.join("inc"));
    build.include(vhacd_dir.join("public"));

    let src_dir = vhacd_dir.join("src");
    let files = [
        "VHACD.cpp",
        "vhacdICHull.cpp",
        "vhacdManifoldMesh.cpp",
        "vhacdMesh.cpp",
        "vhacdVolume.cpp",
    ];

    for file in &files {
        build.file(src_dir.join(file));
    }
}

fn compile_gimpact_utils(_build: &mut cc::Build, _extras_dir: &Path) {
}

fn build_wasm(src_dir: &Path, cpp_bind_dir: &Path, wasm_std_dir: &Path, out_dir: &Path) {
    let mut build = cc::Build::new();

    build
        .warnings(false)
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .compiler("clang++")
        .include(src_dir)
        .include(cpp_bind_dir)
        .include(wasm_std_dir)
        .flag("-xc++")
        .flag("-matomics")
        .flag("-mbulk-memory")
        .flag("-Wno-c++11-narrowing")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-#pragma-messages")
        .flag("-fno-exceptions")
        .target("wasm32-unknown-unknown")
        .define("_WIN32", None)
        .define("_MSC_VER", "1401")
        .define("__i386__", None)
        .define("BT_NO_SIMD_OPERATOR_OVERLOADS", None)
        .define("__wasm32__", None);

    let wasm_parallel = env::var("CARGO_FEATURE_WASM_PARALLEL").is_ok();
    if wasm_parallel {
        build
            .flag("-mthread-model=posix")
            .define("BT_USE_OPENMP", None);
    }

    compile_bullet_core_wasm(&mut build, src_dir);
    compile_cpp_bind(&mut build, cpp_bind_dir);
    compile_wasm_std(&mut build, wasm_std_dir);

    build.opt_level_str("fast").compile("nekobullet_wasm");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=nekobullet_wasm");
}

fn compile_bullet_core_wasm(build: &mut cc::Build, src_dir: &Path) {
    let collision = env::var("CARGO_FEATURE_COLLISION").is_ok();
    let dynamics = env::var("CARGO_FEATURE_DYNAMICS").is_ok();
    let softbody = env::var("CARGO_FEATURE_SOFTBODY").is_ok();
    let vehicle = env::var("CARGO_FEATURE_VEHICLE").is_ok();
    let character = env::var("CARGO_FEATURE_CHARACTER").is_ok();
    let multibody = env::var("CARGO_FEATURE_MULTIBODY").is_ok();
    let inverse_dynamics = env::var("CARGO_FEATURE_INVERSE_DYNAMICS").is_ok();

    compile_linearmath(build, src_dir);

    if collision || dynamics {
        compile_bullet_collision(build, src_dir);
    }

    if dynamics {
        compile_bullet_dynamics(build, src_dir, vehicle, character, multibody, false, inverse_dynamics);
    }

    if softbody {
        compile_bullet_softbody(build, src_dir);
    }

    if inverse_dynamics {
        compile_bullet_inverse_dynamics(build, src_dir);
    }
}

fn compile_wasm_std(build: &mut cc::Build, wasm_std_dir: &Path) {
    let files = [
        "cxa_guard.cpp",
        "stdio.cpp",
        "string_c.cpp",
        "string.cpp",
        "windows.cpp",
        "math.cpp",
    ];

    for file in &files {
        build.file(wasm_std_dir.join(file));
    }
}
