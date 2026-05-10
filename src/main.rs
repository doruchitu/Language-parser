use language_parser::compile_and_run;
use std::io::{self, Write};

fn main() {
    print_banner();
    print_help();

    let mut buffer = String::new();

    loop {
        print!("lp> ");
        io::stdout().flush().unwrap();
        
        buffer.clear();
        if io::stdin().read_line(&mut buffer).is_err() {
            break;
        }
        
        let input = buffer.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            break;
        }
        
        if input == "help" {
            print_help();
            continue;
        }

        match compile_and_run(input) {
            Ok(Some(result)) => println!("→ {}\n", result),
            Ok(None) => println!("→ (no value)\n"),
            Err(e) => eprintln!("✗ {}\n", e),
        }
    }
}

fn print_banner() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║      Language Parser v0.1.0              ║");
    println!("║  Lexer → Parser → Checker → TAC → Run   ║");
    println!("╚═══════════════════════════════════════════╝\n");
}

fn print_help() {
    println!(r#"
━━━ EXAMPLES ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  let x = 42; return x;
  let x = 2 + 3 * 4; return x;
  let a = 5; if a > 3 {{ return a; }} else {{ return 0; }}
  let s = 0; let i = 0; while i < 5 {{ s = s + i; i = i + 1; }} return s;

━━━ COMMANDS ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  help    Show this help
  exit    Exit the parser
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

"#);
}