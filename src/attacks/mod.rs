/// Ataques ECDLP: módulo contenedor.
///
/// - `bsgs`: Baby-step Giant-step (estándar y con GLV/D=-3)
/// - `pollard_rho`: Pollard's Rho con Equivalence Class Search (D=-3)

pub mod bsgs;
pub mod pollard_rho;

// Re-exportar para acceso directo: d3_symmetry::attacks::*
pub use bsgs::*;
pub use pollard_rho::*;
