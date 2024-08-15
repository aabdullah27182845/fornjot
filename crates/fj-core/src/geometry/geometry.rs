use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    storage::Handle,
    topology::{Curve, Surface, Topology, Vertex},
};

use super::{
    vertex::LocalVertexGeom, CurveGeom, GlobalPath, LocalCurveGeom,
    SurfaceGeom, VertexGeom,
};

/// Geometric data that is associated with topological objects
pub struct Geometry {
    curve: BTreeMap<Handle<Curve>, CurveGeom>,
    surface: BTreeMap<Handle<Surface>, SurfaceGeom>,
    vertex: BTreeMap<Handle<Vertex>, VertexGeom>,

    space_2d: Handle<Surface>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(topology: &Topology) -> Self {
        let mut self_ = Self {
            curve: BTreeMap::new(),
            surface: BTreeMap::new(),
            vertex: BTreeMap::new(),

            space_2d: topology.surfaces.space_2d(),

            xy_plane: topology.surfaces.xy_plane(),
            xz_plane: topology.surfaces.xz_plane(),
            yz_plane: topology.surfaces.yz_plane(),
        };

        self_.define_surface_inner(
            self_.xy_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            },
        );
        self_.define_surface_inner(
            self_.xz_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            },
        );
        self_.define_surface_inner(
            self_.yz_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::y_axis(),
                v: Vector::unit_z(),
            },
        );

        self_
    }

    pub(crate) fn define_curve_inner(
        &mut self,
        curve: Handle<Curve>,
        surface: Handle<Surface>,
        geometry: LocalCurveGeom,
    ) {
        self.curve
            .entry(curve)
            .or_default()
            .definitions
            .insert(surface, geometry);
    }

    pub(crate) fn define_surface_inner(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeom,
    ) {
        if surface == self.space_2d {
            panic!("Attempting to define geometry for 2D space");
        }

        if self.surface.contains_key(&surface)
            && (surface == self.xy_plane
                || surface == self.xz_plane
                || surface == self.yz_plane)
        {
            panic!("Attempting to redefine basis plane.");
        }

        self.surface.insert(surface, geometry);
    }

    pub(crate) fn define_vertex_inner(
        &mut self,
        vertex: Handle<Vertex>,
        curve: Handle<Curve>,
        geometry: LocalVertexGeom,
    ) {
        self.vertex
            .entry(vertex)
            .or_default()
            .definitions
            .insert(curve, geometry);
    }

    /// # Access the geometry of the provided curve
    pub fn of_curve(&self, curve: &Handle<Curve>) -> Option<&CurveGeom> {
        self.curve.get(curve)
    }

    /// # Access the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of the surface is not defined.
    pub fn of_surface(&self, surface: &Handle<Surface>) -> &SurfaceGeom {
        self.surface
            .get(surface)
            .expect("Expected geometry of surface to be defined")
    }

    /// # Access the geometry of the provided vertex
    pub fn of_vertex(&self, vertex: &Handle<Vertex>) -> Option<&VertexGeom> {
        self.vertex.get(vertex)
    }

    /// Access the geometry of the xy-plane
    pub fn xy_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.xy_plane)
    }

    /// Access the geometry of the xz-plane
    pub fn xz_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.xz_plane)
    }

    /// Access the geometry of the yz-plane
    pub fn yz_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.yz_plane)
    }
}
