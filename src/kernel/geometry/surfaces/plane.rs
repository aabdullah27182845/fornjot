use nalgebra::point;
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A plane
///
/// For the time being, only planes parallel to the x-y plane are supported.
/// Making this code more flexible to support all planes is subject of an
/// ongoing effort.
#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    /// The origin point of the plane
    ///
    /// The point on the plane that is the origin of the 2-dimensional
    /// surface coordinate system.
    pub origin: Point<3>,

    /// First direction that defines the plane orientation
    ///
    /// It might be most reasonable, if this were a unit vector that is
    /// orthogonal to `v`. As an experiment, this isn't required right now,
    /// to allow for the definition of interesting coordinate systems. It's
    /// unclear how well all algorithms will handle those though.
    ///
    /// Must not be parallel to `v`.
    pub u: Vector<3>,

    /// Second direction that defines the plane orientation
    ///
    /// It might be most reasonable, if this were a unit vector that is
    /// orthogonal to `u`. As an experiment, this isn't required right now,
    /// to allow for the definition of interesting coordinate systems. It's
    /// unclear how well all algorithms will handle those though.
    ///
    /// Must not be parallel to `u`.
    pub v: Vector<3>,
}

impl Plane {
    /// Transform the plane
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        Self {
            origin: transform.transform_point(&self.origin),
            u: transform.transform_vector(&self.u),
            v: transform.transform_vector(&self.v),
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point: Point<3>) -> Point<2> {
        let normal = self.u.cross(&self.v);

        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let d = -(a * self.origin.x + b * self.origin.y + c * self.origin.z);

        let distance = (a * point.x + b * point.y + c * point.z + d).abs()
            / (a * a + b * b + c * c).sqrt();

        // I'm not sure about this. That epsilon is going to be either to small
        // or too large, depending on use case. Maybe it's better to just define
        // that model points are projected into the plane before conversion,
        // like curves do it.
        // - @hannobraun
        if distance > <f64 as approx::AbsDiffEq>::default_epsilon() {
            panic!("Model point is not in surface");
        }

        let p = point - self.origin;

        // scalar projection
        let u = p.dot(&self.u.normalize()) / self.u.magnitude();
        let v = p.dot(&self.v.normalize()) / self.v.magnitude();

        point![u, v]
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: &Point<2>) -> Point<3> {
        self.origin + self.vector_surface_to_model(&point.coords)
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: &Vector<2>) -> Vector<3> {
        vector.x * self.u + vector.y * self.v
    }
}

#[cfg(test)]
impl approx::AbsDiffEq for Plane {
    type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.u.abs_diff_eq(&other.u, epsilon)
            && self.v.abs_diff_eq(&other.v, epsilon)
    }
}

#[cfg(test)]
impl approx::RelativeEq for Plane {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.origin
            .relative_eq(&other.origin, epsilon, max_relative)
            && self.u.relative_eq(&other.u, epsilon, max_relative)
            && self.v.relative_eq(&other.v, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::assert_relative_eq;
    use nalgebra::{point, vector, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use crate::math::{Point, Vector};

    use super::Plane;

    #[test]
    fn test_transform() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![1., 0., 0.],
            v: vector![0., 1., 0.],
        };

        let plane = plane.transform(&Isometry::from_parts(
            Translation::from([2., 4., 6.]),
            UnitQuaternion::from_axis_angle(&Vector::z_axis(), FRAC_PI_2),
        ));

        assert_relative_eq!(
            plane,
            Plane {
                origin: point![0., 5., 9.],
                u: vector![0., 1., 0.],
                v: vector![-1., 0., 0.],
            },
            epsilon = 1e-8,
        );
    }

    #[test]
    fn test_model_to_surface_point_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 2., 0.],
            v: vector![0., 0., 3.],
        };

        verify(&plane, point![-1., -1.]);
        verify(&plane, point![0., 0.]);
        verify(&plane, point![1., 1.]);
        verify(&plane, point![2., 3.]);

        fn verify(plane: &Plane, surface_point: Point<2>) {
            let point = plane.point_surface_to_model(&surface_point);
            let result = plane.point_model_to_surface(point);

            assert_eq!(result, surface_point);
        }
    }

    #[test]
    fn test_surface_to_model_point_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 1., 0.],
            v: vector![0., 0., 1.],
        };

        assert_eq!(
            plane.point_surface_to_model(&point![2., 4.]),
            point![1., 4., 7.],
        );
    }

    #[test]
    fn test_surface_to_model_vector_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 1., 0.],
            v: vector![0., 0., 1.],
        };

        assert_eq!(
            plane.vector_surface_to_model(&vector![2., 4.]),
            vector![0., 2., 4.],
        );
    }
}
