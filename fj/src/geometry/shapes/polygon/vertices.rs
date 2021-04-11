use std::collections::HashSet;

use crate::geometry::point::Pnt2;

use super::Polygon;

pub struct Vertices<'r>(pub(super) &'r mut Polygon);

impl Vertices<'_> {
    pub fn neighbors_of(&self, vertex: impl Into<Pnt2>) -> HashSet<Pnt2> {
        // TASK: Convert to use `self.edges`.

        // TASK: Support zero or multiple vertex chains.
        assert_eq!(self.0.chains.len(), 1);
        let neighbors = self.0.chains[0].neighbors_of(vertex);

        let mut vertices = HashSet::new();

        if let Some(neighbors) = neighbors {
            for neighbor in neighbors.0 {
                vertices.insert(neighbor);
            }
        }

        vertices
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        point::Pnt2,
        shapes::{Polygon, VertexChain},
    };

    #[test]
    fn neighbors_of_should_return_neighbors_of_vertex() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));

        let neighbors_of_a = polygon.vertices().neighbors_of(a);
        let neighbors_of_b = polygon.vertices().neighbors_of(b);
        let neighbors_of_c = polygon.vertices().neighbors_of(c);

        assert!(neighbors_of_a.contains(&b));
        assert!(neighbors_of_a.contains(&c));

        assert!(neighbors_of_b.contains(&a));
        assert!(neighbors_of_b.contains(&c));

        assert!(neighbors_of_c.contains(&a));
        assert!(neighbors_of_c.contains(&b));
    }
}
