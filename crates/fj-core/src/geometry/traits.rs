//! # Traits that abstract over curve or surface geometry
//!
//! Fornjot's geometry is built on the concept of a uniform representation:
//! Polylines to represent curves and triangle meshes to represent surfaces. The
//! traits in this module provide the interface between this uniform
//! representation and specific geometry code.
//!
//! ## Implementation Note
//!
//! As of this writing, the transition from the previous, more limited, geometry
//! system to the new one based on uniform representation is still ongoing. As a
//! result of that, this module might still be incomplete.

use fj_math::Point;

use super::{CurveBoundary, Path, Tolerance};

/// # Generate polylines, the uniform representation of curve geometry
///
/// This trait provides a generic and uniform interface to curve geometry. It is
/// implemented by types that represent specific kinds of curve geometry.
///
/// It is generic over the dimensionality of the generated polyline. Typically,
/// two variants should be implemented per curve geometry type:
///
/// - `CurveGeom2<2>` for surface-local geometry.
/// - `CurveGeom2<3>` for global 3D geometry.
///
///
/// ## Determinism
///
/// For a given curve and a given tolerance, the uniform representation of a
/// curve must be deterministic. This means that the same representation must be
/// returned, regardless of which points on the curve are queried, and in what
/// order.
pub trait GenPolyline<const D: usize> {
    /// # Access the origin of the curve
    fn origin(&self) -> Point<D>;

    /// # Compute a line segment to approximate the curve at this point
    ///
    /// ## Degenerate Case
    ///
    /// If the curve requires no approximation (meaning it is a line), then per
    /// convention, a degenerate line segment is returned, that collapses to the
    /// provided point.
    fn line_segment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
    ) -> [Point<D>; 2];

    /// # Generate a polyline within the provided boundary
    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        tolerance: Tolerance,
    ) -> Vec<Point<1>>;
}

// This implementation is temporary, to ease the transition towards a curve
// geometry trait. Eventually, `CurveGeom2` is expected to replace `Path`.
impl<const D: usize> GenPolyline<D> for Path<D> {
    fn origin(&self) -> Point<D> {
        match self {
            Self::Circle(circle) => circle.origin(),
            Self::Line(line) => line.origin(),
        }
    }

    fn line_segment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
    ) -> [Point<D>; 2] {
        match self {
            Self::Circle(circle) => circle.line_segment_at(point, tolerance),
            Self::Line(line) => line.line_segment_at(point, tolerance),
        }
    }

    fn generate_polyline(
        &self,
        boundary: CurveBoundary<Point<1>>,
        tolerance: Tolerance,
    ) -> Vec<Point<1>> {
        match self {
            Self::Circle(circle) => {
                circle.generate_polyline(boundary, tolerance)
            }
            Self::Line(line) => line.generate_polyline(boundary, tolerance),
        }
    }
}

/// # A line segment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LineSegment<const D: usize> {
    /// # The points that bound the line segment
    pub points: [Point<D>; 2],
}
