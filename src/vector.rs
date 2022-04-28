use std::ops;

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

/// Returns the length squared of a given vector
pub fn lengthSquared(vec: Vec3D) -> f64 {
    vec.x*vec.x + vec.y * vec.y + vec.z*vec.z
}

/// Returns the distance between two vectors
pub fn distance(lhs: Vec3D, rhs: Vec3D) -> f64 {
    let x = lhs.x-rhs.x;
    let y = lhs.y-rhs.y;
    let z= lhs.z-rhs.z;
    (x*x+y*y+z*z).sqrt()
}

/// Performs component wise multiplication of two vectors
pub fn component_mult(lhs: Vec3D, rhs: Vec3D) -> Vec3D {
    Vec3D {
        x: lhs.x*rhs.x, 
        y: lhs.y*rhs.y, 
        z: lhs.z*rhs.z, 
    }
}

/// Divides a Vec3D by a scalar
impl ops::Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero");
        }

        Vec3D {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

/// Adds a Vec3D to a Vec3D
impl ops::Add<Vec3D> for Vec3D {
    type Output = Self;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            x: self.x+rhs.x, 
            y: self.y+rhs.y, 
            z: self.z+rhs.z, 
        }
    }
}

/// Subtracts rhs from a Vec3D
impl ops::Sub<Vec3D> for Vec3D {
    type Output = Self;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            x: self.x-rhs.x, 
            y: self.y-rhs.y, 
            z: self.z-rhs.z, 
        }
    }
}

/// Cross multiplication
impl ops::Mul<Vec3D> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

/// Cross multiplication
impl ops::Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Cross multiplication
impl ops::Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}