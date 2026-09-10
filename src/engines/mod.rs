pub mod bash;
pub mod batch;
pub mod brainfuck;
pub mod c;
pub mod csharp;
pub mod go;
pub mod hq9plus;
pub mod javascript;
pub mod lisp;
pub mod lua;
pub mod ocaml;
pub mod pascal;
pub mod perl;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;

use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;

/// Contrato formal para cada motor de generación de quines
pub trait QuineEngine: Send + Sync {
    /// Lenguaje correspondiente al motor
    fn language(&self) -> Language;

    /// Genera el código fuente ejecutable del quine (con payload opcional inyectado)
    fn generate(&self, payload: Option<&Payload>) -> Result<String, DomainError>;
}
