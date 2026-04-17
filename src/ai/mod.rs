pub mod grammar_db;

use self::grammar_db::GrammarRegistry;

pub struct OmniAi {
    registry: GrammarRegistry,
}

impl OmniAi {
    pub fn new() -> Self {
        Self {
            registry: GrammarRegistry::new(),
        }
    }

    pub fn analyze(&self, code: &str) -> String {
        self.registry.deep_inference(code).to_string()
    }
}
