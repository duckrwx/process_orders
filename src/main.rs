mod models;
mod task_manager;
mod arq_load;
mod normalize_json;
mod raiz_rapidomacd5_2st;
mod enum_acao;
mod action;

use crate::task_manager::TaskManager;
use crate::normalize_json::{normalize_json, normalize_by_action};
use crate::action::agrupar_ordens_por_ativo;
use enum_acao::Acao;

fn main() {
    let file_path_str = arq_load::arq_load();
    
    match TaskManager::new(file_path_str) {
        Ok(manager) => {
            // Passo 1: Salvar um arquivo geral com TODAS as ordens.
            // Esta parte já estava correta.
            println!("\n--- 💾 Salvando arquivo geral com todas as ordens ---");
            match normalize_json(&manager.tasks) {
                Ok(_) => {}, // Se der certo, não faz nada.
                Err(e) => {
                    eprintln!("❌ Erro ao salvar JSON geral: {}", e);
                    // Decidi não sair do programa aqui para que ele continue e tente salvar os arquivos individuais.
                }
            }

            // Passo 2: Agrupar as ordens.
            let acoes_agrupadas = agrupar_ordens_por_ativo(&manager);

            println!("\n--- 💾 Salvando arquivos individuais para cada ativo ---");
            // Passo 3: Iterar sobre os grupos e salvar cada um.
            for acao in acoes_agrupadas {
                match acao {
                    // Para o caso COGN3...
                    Acao::COGN3(ordens) => {
                        // 👇 ADICIONADO: Chamamos a função para salvar o arquivo específico de COGN3.
                        if let Err(e) = normalize_by_action(&ordens, "COGN3") {
                            eprintln!("❌ Erro ao salvar JSON para COGN3: {}", e);
                        }
                    }
                    // Para o caso WINV25...
                    Acao::WINV25(ordens) => {
                        // 👇 ADICIONADO: Chamamos a função para salvar o arquivo específico de WINV25.
                        if let Err(e) = normalize_by_action(&ordens, "WINV25") {
                            eprintln!("❌ Erro ao salvar JSON para WINV25: {}", e);
                        }
                    }
                    // Para o caso AAPL34...
                    Acao::AAPL34(ordens) => {
                        // 👇 ADICIONADO: Chamamos a função para salvar o arquivo específico de AAPL34.
                        if let Err(e) = normalize_by_action(&ordens, "AAPL34") {
                            eprintln!("❌ Erro ao salvar JSON para AAPL34: {}", e);
                        }
                    }
                }
            }
            println!("\n🎉 Processo de salvamento concluído!");
        },
        Err(e) => {
            eprintln!("❌ Erro ao carregar ordens: {}", e);
            std::process::exit(1);
        }
    };
}