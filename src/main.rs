mod models;
mod task_manager;

use crate::task_manager::TaskManager;
use rfd::FileDialog;
use std::process;
use crate::models::Ordem;
fn main() {
    println!("Selecione o arquivo CSV de ordens:");
    
    let file_path = match FileDialog::new()
        .add_filter("Todos os arquivos", &["*"])
        .pick_file()
    {
        Some(path) => path,
        None => {
            println!("❌ Nenhum arquivo selecionado. Encerrando...");
            return;  // 👈 Sai graciosamente sem panic
        }
    };
    
    let file_path_str = file_path.to_str()
        .expect("Caminho inválido");
    
    println!("✅ Arquivo selecionado: {}", file_path_str);

    let loaded_orders: Vec<Ordem> = Ordem::stream_from_csv(file_path_str)
        .expect("Erro ao abrir CSV")
        .collect::<Result<Vec<_>, _>>()
        .expect("Erro ao ler linhas do CSV");

    println!("📊 Total de ordens carregadas: {}", loaded_orders.len());
}