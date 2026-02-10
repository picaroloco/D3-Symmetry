/// Ataques ECDLP: módulo contenedor.
///
/// - `bsgs`: Baby-step Giant-step (estándar y con GLV/D=-3)
/// - (futuro) `pollard_rho`: Pollard's Rho con clases de equivalencia hexagonales

pub mod bsgs;

// Re-exportar para acceso directo: d3_symmetry::attacks::*
pub use bsgs::*;
