use thiserror::Error;

#[derive(Debug, Error)]
pub enum PhysicsError {
    #[error("Failed to create rigid body: {reason}")]
    RigidBodyCreation { reason: String },
    
    #[error("Failed to create constraint: {reason}")]
    ConstraintCreation { reason: String },
    
    #[error("Failed to create collision shape: {reason}")]
    ShapeCreation { reason: String },
    
    #[error("Invalid parameter: {name} = {value}")]
    InvalidParameter { name: String, value: String },
    
    #[error("Null pointer encountered in {context}")]
    NullPointer { context: String },
    
    #[error("Physics world error: {0}")]
    WorldError(String),
}
