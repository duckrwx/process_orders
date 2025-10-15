use std::io::Write;

pub fn read_string (prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer){
        Ok(_) => buffer.trim().to_string(), 
        Err(_) => {
            println!("Erro de leitura");
            std::process::exit(1);
        }
    }
    //Metodo de leitura do teclado, que pode falhar! Por isso o match.
}