use std::hash::{Hash, Hasher};

use fj_math::Triangle;

use crate::{geometry::Surface, shape::Handle};

use super::edges::Cycle;

/// A face of a shape
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub enum Face {
    /// A face of a shape
    ///
    /// A face is defined by a surface, and is bounded by edges that lie in that
    /// surface.
    Face {
        /// The surface that defines this face
        surface: Handle<Surface>,

        /// The cycles that bound the face
        ///
        /// # Implementation Note
        ///
        /// Since these cycles bound the face, the edges they consist of must
        /// lie in the surface. The data we're using here is 3-dimensional
        /// though, so no such limitation is enforced.
        ///
        /// It might be less error-prone to specify the edges in surface
        /// coordinates.
        cycles: Vec<Handle<Cycle>>,

        /// The color of the face
        color: [u8; 4],
    },

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle<3>>),
}

impl Face {
    /// Access the surface that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn surface(&self) -> Surface {
        match self {
            Self::Face { surface, .. } => *surface.get(),
            _ => {
                // No code that still uses triangle representation is calling
                // this method.
                unreachable!()
            }
        }
    }

    /// Access the cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn cycles(&self) -> impl Iterator<Item = Cycle> + '_ {
        match self {
            Self::Face { cycles, .. } => {
                cycles.iter().map(|handle| handle.get().clone())
            }
            _ => {
                // No code that still uses triangle representation is calling
                // this method.
                unreachable!()
            }
        }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.surface() == other.surface() && self.cycles().eq(other.cycles())
    }
}

impl Hash for Face {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.surface().hash(state);
        for cycle in self.cycles() {
            cycle.hash(state);
        }
    }
}
