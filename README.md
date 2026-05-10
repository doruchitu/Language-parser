# Language Parser

Compilator complet funcțional: lexer → parser → type checker → code generator → interpreter.

## Descriere rapidă

5 faze de compilare:
1. **Lexer** - Tokenizare
2. **Parser** - AST generation
3. **Type Checker** - Type checking și symbol tables
4. **Code Generator** - TAC (Three-Address Code)
5. **Interpreter** - Execuție

## Quick Start

```bash
cargo run
lp> let x = 2 + 3 * 4; return x;
→ 14
```

## Limbaj suportat

```rust
let x = 42;
if x > 10 { return x; } else { return 0; }
while x < 100 { x = x + 1; }
```

## Structură
src/
├── phases/
│   ├── lexer.rs       # Faza 1: Tokenizare
│   ├── parser.rs      # Faza 2: Analiză sintactică
│   ├── semantic.rs    # Faza 3: Type checking
│   └── codegen.rs     # Faza 4: Generare TAC
├── ast.rs             # Definiții AST
├── tac.rs             # Instrucțiuni TAC
├── interpreter.rs     # Faza 5: Execuție
├── token.rs           # Definiții tokeni
├── symbol_table.rs    # Tabela de simboluri
├── error.rs           # Gestionare errori unificată
├── lib.rs             # Orchestrare librărie
└── main.rs            # CLI interactiv