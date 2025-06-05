use std::{
    fs,
    io::{self, BufRead, Write},
    process::{self, exit},
};


pub fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    // Processar o conteúdo do arquivo
    println!("Executing file: {}", path);
    println!("File content:\n{}", contents);
    Ok(())
}


pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    println!("Rust Lox REPL (press Ctrl+C to exit)");

    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        
        let line = input.trim();

        // Processar a linha de entrada
        if line.is_empty() {
            continue;
        }

        run(&line);
    }
}


pub fn run(line: &str) {
    let mut scanner = Scanner::new(line);
    let tokens = scanner.scan_tokens();

    // Verifica se houve erros durante o scan
    if scanner.had_error {
        // report de erros aqui
        return;
    }

    // Por enquanto, apenas imprime os tokens
    for token in tokens {
        println!("{}", token);
    }
}


fn error(line: u32, message: &str) {
    println!("{} {}", line, message)
}

// reporte de erro bem simples, apenas mostrando a linha, localização e mensagem
fn report(line: u32, location: &str, message: &str) {
    println!("[Line {} ] Error {} : {} ", line, location, message);
}