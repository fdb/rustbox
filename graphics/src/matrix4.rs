use std::ops;
use super::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Matrix4 {
    pub m: [f32; 16],
}

impl Matrix4 {
    pub fn new() -> Matrix4 {
        Matrix4 { m: [0.; 16] }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 { m: [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.] }
    }

    pub fn translate_matrix(tx: f32, ty: f32, tz: f32) -> Matrix4 {
        Matrix4 { m: [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., tx, ty, tz, 1.] }
    }

    pub fn rotate_x_matrix(theta: f32) -> Matrix4 {
        let c = theta.cos();
        let s = theta.sin();
        Matrix4 { m: [
            1., 0., 0.,  0.,
            0., c, s, 0.,
            0., -s, c, 0.,
            0., 0., 0., 1.] }
    }

    pub fn rotate_y_matrix(theta: f32) -> Matrix4 {
        let c = theta.cos();
        let s = theta.sin();
        Matrix4 { m: [
            c, 0., s,  0.,
            0., 1., 0., 0.,
            -s, 0., c, 0.,
            0., 0., 0., 1.] }
    }

    pub fn rotate_z_matrix(theta: f32) -> Matrix4 {
        let c = theta.cos();
        let s = theta.sin();
        Matrix4 { m: [
            c, s, 0.,  0.,
            -s, c, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.] }
    }

    pub fn look_at_matrix(eye: &Vector3, target: &Vector3, up: &Vector3) -> Matrix4 {
        let mut z = *eye - *target;

        if z.length_squared() == 0.0 {
            // Eye and target are in the same position.
            z.z = 1.;
        }
        z = z.normalized();
        let mut x = up.cross(&z);
        if x.length_squared() == 0.0 {
            // Up and z are parallel.
            if up.z.abs() == 1.0 {
                z.x += 0.0001;
            } else {
                z.z += 0.0001;
            }
            z = z.normalized();
            x = up.cross(&z);
        }
        x = x.normalized();
        let y = z.cross(&x);
        Matrix4 { m: [x.x, x.y, x.z, 0., y.x, y.y, y.z, 0., z.x, z.y, z.z, 0., 0., 0., 0., 1.] }
    }

    pub fn perspective_matrix(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Matrix4 {
        let x = 2. * near / (right - left);
        let y = 2. * near / (top - bottom);

        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(far + near) / (far - near);
        let d = -2. * far * near / (far - near);

        Matrix4 { m: [x, 0., 0., 0., 0., y, 0., 0., a, b, c, -1., 0., 0., d, 0.] }
    }

    pub fn orthographic_matrix(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Matrix4 {
        let w = 1.0 / (right - left);
        let h = 1.0 / (top - bottom);
        let p = 1.0 / (far - near);

        let x = (right + left) * w;
        let y = (top + bottom) * h;
        let z = (far + near) * p;


        Matrix4 { m: [2. * w, 0., 0., 0., 0., 2. * h, 0., 0., 0., 0., -2. * p, 0., -x, -y, -z, 1.] }
    }

    pub fn inverse(&self) -> Option<Matrix4> {
        let me = &self.m;

        let (n11, n21, n31, n41) = (me[0], me[1], me[2], me[3]);
        let (n12, n22, n32, n42) = (me[4], me[5], me[6], me[7]);
        let (n13, n23, n33, n43) = (me[8], me[9], me[10], me[11]);
        let (n14, n24, n34, n44) = (me[12], me[13], me[14], me[15]);

        let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 -
            n23 * n32 * n44 + n22 * n33 * n44;
        let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 +
            n13 * n32 * n44 - n12 * n33 * n44;
        let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 -
            n13 * n22 * n44 + n12 * n23 * n44;
        let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 +
            n13 * n22 * n34 - n12 * n23 * n34;

        let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

        if det == 0. {
            return None;
        }

        let det_inv = 1. / det;

        let mut r: [f32; 16] = [0.; 16];
        r[0] = t11 * det_inv;
        r[1] = (n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 +
                    n23 * n31 * n44 - n21 * n33 * n44) * det_inv;
        r[2] = (n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 -
                    n22 * n31 * n44 + n21 * n32 * n44) * det_inv;
        r[3] = (n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 +
                    n22 * n31 * n43 - n21 * n32 * n43) * det_inv;

        r[4] = t12 * det_inv;
        r[5] = (n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 -
                    n13 * n31 * n44 + n11 * n33 * n44) * det_inv;
        r[6] = (n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 +
                    n12 * n31 * n44 - n11 * n32 * n44) * det_inv;
        r[7] = (n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 -
                    n12 * n31 * n43 + n11 * n32 * n43) * det_inv;

        r[8] = t13 * det_inv;
        r[9] = (n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 +
                    n13 * n21 * n44 - n11 * n23 * n44) * det_inv;
        r[10] = (n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 -
                     n12 * n21 * n44 + n11 * n22 * n44) * det_inv;
        r[11] = (n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 +
                     n12 * n21 * n43 - n11 * n22 * n43) * det_inv;

        r[12] = t14 * det_inv;
        r[13] = (n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 -
                     n13 * n21 * n34 + n11 * n23 * n34) * det_inv;
        r[14] = (n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 +
                     n12 * n21 * n34 - n11 * n22 * n34) * det_inv;
        r[15] = (n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 -
                     n12 * n21 * n33 + n11 * n22 * n33) * det_inv;




        Some(Matrix4 { m: r })



    }

    pub fn transform(&self, v: &Vector3) -> Vector3 {
        let m = &self.m;
        let w = 1. / (m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15]);
        let x = (m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12]) * w;
        let y = (m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13]) * w;
        let z = (m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14]) * w;
        Vector3 { x, y, z }
    }
}

impl ops::Mul<Self> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = &self.m;
        let b = &rhs.m;
        let mut r: [f32; 16] = [0.; 16];
        let (a11, a12, a13, a14) = (a[0], a[4], a[8], a[12]);
        let (a21, a22, a23, a24) = (a[1], a[5], a[9], a[13]);
        let (a31, a32, a33, a34) = (a[2], a[6], a[10], a[14]);
        let (a41, a42, a43, a44) = (a[3], a[7], a[11], a[15]);

        let (b11, b12, b13, b14) = (b[0], b[4], b[8], b[12]);
        let (b21, b22, b23, b24) = (b[1], b[5], b[9], b[13]);
        let (b31, b32, b33, b34) = (b[2], b[6], b[10], b[14]);
        let (b41, b42, b43, b44) = (b[3], b[7], b[11], b[15]);

        r[0] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        r[4] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        r[8] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        r[12] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

        r[1] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        r[5] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        r[9] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        r[13] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

        r[2] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        r[6] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        r[10] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        r[14] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

        r[3] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        r[7] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        r[11] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        r[15] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;

        Matrix4 { m: r }
    }
}

impl ops::Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let m = &self.m;
        let mut r: [f32; 16] = [0.; 16];
        r[0] = m[0] * rhs;
        r[1] = m[1] * rhs;
        r[2] = m[2] * rhs;
        r[3] = m[3] * rhs;
        r[4] = m[4] * rhs;
        r[5] = m[5] * rhs;
        r[6] = m[6] * rhs;
        r[7] = m[7] * rhs;
        r[8] = m[8] * rhs;
        r[9] = m[9] * rhs;
        r[10] = m[10] * rhs;
        r[11] = m[11] * rhs;
        r[12] = m[12] * rhs;
        r[13] = m[13] * rhs;
        r[14] = m[14] * rhs;
        r[15] = m[15] * rhs;
        Matrix4 { m: r }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn increasing_numbers_matrix() -> Matrix4 {
        let mut m: [f32; 16] = [0.; 16];
        for i in 0..16 {
            m[i] = i as f32 + 1.;
        }
        Matrix4 { m }
    }

    fn prime_numbers_matrix() -> Matrix4 {
        let m = [101., 103., 107., 109., 113., 127., 131., 137., 139., 149., 151., 157., 163.,
                 167., 173., 179.];
        Matrix4 { m }
    }

    #[test]
    fn test_zero() {
        let m1 = Matrix4::new();
        assert_eq!(m1.m, [0.; 16]);
    }

    #[test]
    fn test_identity() {
        let m1 = Matrix4::identity();
        let m2 = increasing_numbers_matrix();
        let m3 = m1 * m2.clone();
        assert_eq!(m3.m, m2.m);
    }

    #[test]
    fn test_multiply_matrix() {
        let m1 = increasing_numbers_matrix();
        let m2 = prime_numbers_matrix();
        let m3 = m1 * m2;
        assert_eq!(m3.m[0], 2996.);
        assert_eq!(m3.m[5], 4216.);
        assert_eq!(m3.m[10], 5476.);
        assert_eq!(m3.m[15], 6928.);
    }

    #[test]
    fn test_multiply_scalar() {
        let m1 = increasing_numbers_matrix();
        let m2 = m1 * 10.;
        assert_eq!(m2.m[0], 10.);
        assert_eq!(m2.m[5], 60.);
        assert_eq!(m2.m[10], 110.);
        assert_eq!(m2.m[15], 160.);
    }

    #[test]
    fn test_vector_transform() {
        let m1 = Matrix4::translate_matrix(10., 20., 30.);
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = m1.transform(&v1);
        assert_eq!(v2.x, 11.);
        assert_eq!(v2.y, 22.);
        assert_eq!(v2.z, 33.);
    }
}
