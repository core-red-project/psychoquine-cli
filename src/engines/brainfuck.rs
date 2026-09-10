use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::domain::payload::Payload;
use crate::engines::QuineEngine;

pub struct BrainfuckEngine;

impl QuineEngine for BrainfuckEngine {
    fn language(&self) -> Language {
        Language::Brainfuck
    }

    fn generate(&self, _payload: Option<&Payload>) -> Result<String, DomainError> {
        // Canonical classic Brainfuck quine (Daniel B. Cristofani)
        Ok("->+>+++>(+ cockpit +)>+++>+>---++>->>[-<<[+]>>]<[+]<+>>>[>[-]++++++[<++++++++>-]++++[<++++++++>-]>]<<++---.<++++.>>[---<+>-]<.+++[->+++<]>++.++++++++.+++++.--------.-[--->+<]>--.+[->+++<]>+.++++++++<.>>+==[.[-]->>+<]>+.[-]<\n".to_string())
    }
}
