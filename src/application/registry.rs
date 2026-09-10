use std::collections::HashMap;
use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::engines::{
    bash::BashEngine,
    batch::DosBatchEngine,
    brainfuck::BrainfuckEngine,
    c::CEngine,
    csharp::CSharpEngine,
    go::GoEngine,
    hq9plus::Hq9PlusEngine,
    javascript::JavascriptEngine,
    lisp::{CommonLispEngine, SchemeEngine},
    lua::LuaEngine,
    ocaml::OcamlEngine,
    pascal::PascalEngine,
    perl::PerlEngine,
    php::PhpEngine,
    python::PythonEngine,
    ruby::RubyEngine,
    rust::RustEngine,
    QuineEngine,
};

/// Registry desacoplado y extensible de motores de quines
#[derive(Clone)]
pub struct EngineRegistry {
    engines: HashMap<Language, Arc<dyn QuineEngine>>,
}

impl Default for EngineRegistry {
    fn default() -> Self {
        let mut registry = Self {
            engines: HashMap::new(),
        };

        registry.register(Arc::new(PythonEngine));
        registry.register(Arc::new(JavascriptEngine));
        registry.register(Arc::new(CEngine));
        registry.register(Arc::new(CSharpEngine));
        registry.register(Arc::new(RustEngine));
        registry.register(Arc::new(BashEngine));
        registry.register(Arc::new(PerlEngine));
        registry.register(Arc::new(PhpEngine));
        registry.register(Arc::new(RubyEngine));
        registry.register(Arc::new(GoEngine));
        registry.register(Arc::new(LuaEngine));
        registry.register(Arc::new(SchemeEngine));
        registry.register(Arc::new(CommonLispEngine));
        registry.register(Arc::new(OcamlEngine));
        registry.register(Arc::new(PascalEngine));
        registry.register(Arc::new(DosBatchEngine));
        registry.register(Arc::new(BrainfuckEngine));
        registry.register(Arc::new(Hq9PlusEngine));

        registry
    }
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, engine: Arc<dyn QuineEngine>) {
        self.engines.insert(engine.language(), engine);
    }

    pub fn get(&self, language: Language) -> Result<Arc<dyn QuineEngine>, DomainError> {
        self.engines
            .get(&language)
            .cloned()
            .ok_or_else(|| DomainError::UnsupportedLanguage(language.to_string()))
    }

    pub fn supported_languages(&self) -> Vec<Language> {
        let mut langs: Vec<Language> = self.engines.keys().copied().collect();
        langs.sort_by_key(|l| l.slug());
        langs
    }
}
