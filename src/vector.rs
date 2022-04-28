pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

/// Returns the dot product of two 3d vectors
/// 
/// # Arguments
/// * `lhs` - Left hand vector
/// * `rhs` - Right hand vector
pub fn dot(lhs: Vec3D, rhs: Vec3D) -> f64 {
    // Return dot product
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

/// Returns the length of a given vector
pub fn length(vec: Vec3D) -> f64 {
    (vec.x*vec.x + vec.y * vec.y + vec.z*vec.z).sqrt()
}