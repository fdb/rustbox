use std::ops;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0., 0., 0.)
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        let (ax, ay, az) = (self.x, self.y, self.z);
        let (bx, by, bz) = (rhs.x, rhs.y, rhs.z);

        let x = ay * bz - az * by;
        let y = az * bx - ax * bz;
        let z = ax * by - ay * bx;

        return Vector3 { x, y, z };
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vector3 {
        *self / self.length()
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use vector3::*;

    #[test]
    fn test_add_vector() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(10., 20., 30.);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 11.);
        assert_eq!(v3.y, 22.);
        assert_eq!(v3.z, 33.);
    }

    #[test]
    fn test_add_scalar() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = v1 + 100.;
        assert_eq!(v2.x, 101.);
        assert_eq!(v2.y, 102.);
        assert_eq!(v2.z, 103.);
    }

    #[test]
    fn test_sub_vector() {
        let v1 = Vector3::new(11., 22., 33.);
        let v2 = Vector3::new(10., 20., 30.);
        let v3 = v1 - v2;
        assert_eq!(v3.x, 1.);
        assert_eq!(v3.y, 2.);
        assert_eq!(v3.z, 3.);
    }

    #[test]
    fn test_mul_vector() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(10., 100., 1000.);
        let v3 = v1 * v2;
        assert_eq!(v3.x, 10.);
        assert_eq!(v3.y, 200.);
        assert_eq!(v3.z, 3000.);
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = v1 * 10.;
        assert_eq!(v2.x, 10.);
        assert_eq!(v2.y, 20.);
        assert_eq!(v2.z, 30.);
    }

    #[test]
    fn test_normalized() {
        let v1 = Vector3::new(100., 50., -25.);
        let v2 = v1.normalized();
        assert_eq!((v2.x, v2.y, v2.z), (1., 0.5, -0.25));
    }
}
