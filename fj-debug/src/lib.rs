//! Debug information definitions for the Fornjot ecosystem
//!
//! This crate contains debug information that is used by other crates within
//! the Fornjot ecosystem. The types in here aren't very useful in themselves,
//! but they define an interface that other crates use to communicate between
//! each other.

#![deny(missing_docs)]

use parry3d_f64::query::Ray;

/// Debug info from the CAD kernel that can be visualized
///
/// At this point, this is a placeholder that will be filled with life later.
#[derive(Default)]
pub struct DebugInfo {
    /// Rays being used during face triangulation
    pub triangle_edge_checks: Vec<TriangleEdgeCheck>,
}

impl DebugInfo {
    /// Construct an empty instance of `DebugInfo`
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all information within this instance
    ///
    /// The resulting instance is the same, as if created by [`DebugInfo::new`],
    /// but calling `clear` might be more efficient in regard to heap
    /// allocations.
    pub fn clear(&mut self) {
        self.triangle_edge_checks.clear();
    }
}

/// Record of a check to determine if a triangle edge is within a face
pub struct TriangleEdgeCheck {
    /// The ray used to perform the check
    pub ray: Ray,

    /// Where the ray hit any edges of the face
    pub hits: Vec<f64>,
}

impl TriangleEdgeCheck {
    /// Construct a new instance
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            hits: Vec::new(),
        }
    }
}
