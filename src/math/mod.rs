/// Fundamentos matemáticos: aritmética modular y operaciones de curva elíptica.

pub mod field;
pub mod curve;

// Re-exportar todo para acceso directo: d3_symmetry::math::*
pub use field::*;
pub use curve::*;
