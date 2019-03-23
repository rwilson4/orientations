#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! A library for Rotations and Orientations.

pub use vector3d::Vector3d;
pub use rotation::Rotation;
pub use orientation::Orientation;
pub use quaternion::Quaternion;
pub use rotation_matrix::RotationMatrix;

// Modules
mod constants;
mod vector3d;
mod rotation;
mod orientation;
mod quaternion;
mod rotation_matrix;
