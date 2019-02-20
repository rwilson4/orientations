const DBL_EPSILON: f64 = 2.220_446_049_250_313e-16;

#[derive(Clone)]
struct Vector3d {
    data: [f64; 3]
}

impl Vector3d {
    fn new(data: [f64; 3]) -> Vector3d {
        Vector3d{ data }
    }

    fn dot(&self, other: &Vector3d) -> f64 {
        let mut dot_product: f64 = 0.0;
        for i in 0..3 {
            dot_product += self.data[i] * other.data[i];
        }
        dot_product
    }

    fn norm_squared(&self) -> f64 {
        self.dot(&self)
    }

    fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    fn scalar_multiple(&self, alpha: f64) -> Vector3d {
        Vector3d::new(
            [
                alpha * self.data[0],
                alpha * self.data[1],
                alpha * self.data[2]
            ]
        )
    }

    fn negate(&self) -> Vector3d {
        Vector3d::new(
            [
                -self.data[0],
                -self.data[1],
                -self.data[2]
            ]
        )
    }

    fn zero() -> Vector3d {
        Vector3d::new( [0.0, 0.0, 0.0] )
    }
}

#[derive(Clone)]
struct Quaternion {
    real_part: f64,
    imaginary_part: Vector3d
}

trait Rotation {
    type R: Rotation;
    fn identity() -> Self::R;
    fn inverse(&self) -> Self::R;
    fn as_quaternion(&self) -> Quaternion;
    fn before<T: Rotation<R = T>>(&self, r: &T) -> T;
    fn after<T: Rotation<R = T>>(&self, r: &T) -> T;
    fn multiply<T: Rotation>(&self, r: &T) -> Self::R;
}

trait Orientation {
}

impl Quaternion {
    fn new(real_part: f64, imaginary_part: Vector3d) -> Quaternion {
        Quaternion {
            real_part,
            imaginary_part
        }
    }

    fn conjugate(&self) -> Quaternion {
        Quaternion::new(self.real_part, self.imaginary_part.negate())
    }

    fn norm_squared(&self) -> f64 {
        self.real_part * self.real_part + self.imaginary_part.norm_squared()
    }

}

impl Rotation for Quaternion {
    type R = Quaternion;

    fn identity() -> Quaternion {
        Quaternion::new(1.0, Vector3d::zero())
    }

    fn inverse(&self) -> Quaternion {
        // Check that norm is > 0
        let norm_squared = self.norm_squared();
        if norm_squared < DBL_EPSILON {
            panic!("Quaternion close to zero; cannot invert.")
        }

        let inv_norm_squared = 1.0 / norm_squared;
        let c = self.conjugate();
        let real_part = c.real_part * inv_norm_squared;
        let imaginary_part = c.imaginary_part.scalar_multiple(inv_norm_squared);
        Quaternion::new(real_part, imaginary_part)
    }

    fn as_quaternion(&self) -> Quaternion {
        self.clone()
    }
    
    fn multiply<T: Rotation>(&self, r: &T) -> Quaternion {
        let rr = r.as_quaternion();
        let real_part = self.real_part * rr.real_part - self.imaginary_part.dot(&rr.imaginary_part);
        let q_i = self.imaginary_part.data[1] * rr.imaginary_part.data[2] - self.imaginary_part.data[2] * rr.imaginary_part.data[1];
        let q_j = self.imaginary_part.data[2] * rr.imaginary_part.data[0] - self.imaginary_part.data[0] * rr.imaginary_part.data[2];
        let q_k = self.imaginary_part.data[0] * rr.imaginary_part.data[1] - self.imaginary_part.data[1] * rr.imaginary_part.data[0];
        let imaginary_part = Vector3d::new( [q_i, q_j, q_k] );

        Quaternion::new(real_part, imaginary_part)
    }

    fn before<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.multiply(self)
    }

    fn after<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.inverse().multiply(&self.inverse()).inverse()
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn vector3d_dot() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = 32.0;
        let actual = x.dot(&y);
        assert_eq!(actual, expected);
    }
}
