use rfd::FileDialog;
use std::env;

pub fn arq_load() -> String {
    let args: Vec<String> = env::args().collect();
    
    let file_path_str = if args.len() > 1 {
        println!("📁 Usando arquivo: {}", args[1]);
        args[1].clone()
    } else {
        println!("Selecione o arquivo CSV de ordens:");
        match FileDialog::new()
            .set_title("Selecione o arquivo CSV")
            .pick_file()
        {
            Some(path) => {
                let p = path.to_str()
                    .expect("❌ Caminho inválido")
                    .to_string();
                println!("✅ Arquivo selecionado: {}", p);
                p
            },
            None => {
                eprintln!("❌ Nenhum arquivo selecionado.");
                eprintln!("\n💡 Dica: cargo run -- ./arquivo.csv");
                std::process::exit(1);
            }
        }
    };

    println!("\n⏳ Processando CSV...");
    file_path_str

}