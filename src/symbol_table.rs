use std::collections::HashMap;
use crate::ast::Type;

/// A symbol represents a variable or function in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
    pub defined: bool,
}

/// Symbol table with scope management
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    pub errors: Vec<String>,
}

impl SymbolTable {
    /// Create a new symbol table with global scope
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
        }
    }

    /// Enter a new scope (for blocks, functions, etc)
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Declare a symbol in current scope
    pub fn declare(&mut self, sym: Symbol) {
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(sym.name.clone(), sym);
    }

    /// Look up a symbol (searches from current scope upwards)
    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym.clone());
            }
        }
        None
    }

    /// Check if symbol exists in current scope only
    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        self.scopes.last().unwrap().contains_key(name)
    }

    /// Record an error
    pub fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}