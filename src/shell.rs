use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub async fn run_shell() {
    println!("coinpaprika-cli interactive shell: type commands without 'coinpaprika-cli' prefix");
    println!("Type 'exit' or 'quit' to leave. Ctrl+D also exits.\n");

    let mut rl = match DefaultEditor::new() {
        Ok(rl) => rl,
        Err(e) => {
            eprintln!("Failed to initialize shell: {e}");
            return;
        }
    };

    loop {
        match rl.readline("coinpaprika> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if line == "exit" || line == "quit" {
                    break;
                }

                let _ = rl.add_history_entry(line);

                let args = split_args(line);
                let mut full_args = vec!["coinpaprika-cli".to_string()];
                full_args.extend(args);

                match crate::Cli::try_parse_from(&full_args) {
                    Ok(cli) => {
                        if let Err(e) = crate::run(cli).await {
                            eprintln!("Error: {e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C: continue
                println!("(Ctrl+C, or type 'exit' to quit)");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D: exit
                break;
            }
            Err(e) => {
                eprintln!("Shell error: {e}");
                break;
            }
        }
    }
}

fn split_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for ch in input.chars() {
        if in_quotes {
            if ch == quote_char {
                in_quotes = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            in_quotes = true;
            quote_char = ch;
        } else if ch.is_whitespace() {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}
