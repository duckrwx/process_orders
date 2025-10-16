

use crate::task_manager::TaskManager;
use crate::models::Ordem;
use crate::enum_acao::Acao; 


pub fn agrupar_ordens_por_ativo(task_manager: &TaskManager) -> Vec<Acao> {
    // 1. Crie vetores mutáveis e vazios, um para cada tipo de ativo que queremos agrupar.
    let mut cogn3_ordens: Vec<Ordem> = Vec::new();
    let mut winv25_ordens: Vec<Ordem> = Vec::new();
    let mut aapl34_ordens: Vec<Ordem> = Vec::new();

    for ordem in &task_manager.tasks {
        // 3. Use `match` para verificar o campo `ativo` de cada ordem.
        // `as_deref()` converte &Option<String> para Option<&str>, ideal para o match.
        match ordem.ativo.as_deref() {
            Some("COGN3") => {
                // Se for COGN3, clone a ordem e adicione ao vetor correspondente.
                cogn3_ordens.push(ordem.clone());
            }
            Some("WINV25") => {
                winv25_ordens.push(ordem.clone());
            }
            Some("AAPL34") => {
                aapl34_ordens.push(ordem.clone());
            }
            // O braço `_` ignora quaisquer outros ativos que não nos interessam.
            _ => {} 
        }
    }

    // 4. Crie o vetor de resultado que irá conter as variantes do enum.
    let mut resultado: Vec<Acao> = Vec::new();

    // 5. Adicione os vetores preenchidos ao resultado, dentro das suas respectivas variantes do enum.
    // Verificamos se não estão vazios para não adicionar enums sem dados.
    if !cogn3_ordens.is_empty() {
        resultado.push(Acao::COGN3(cogn3_ordens));
    }
    if !winv25_ordens.is_empty() {
        resultado.push(Acao::WINV25(winv25_ordens));
    }
    if !aapl34_ordens.is_empty() {
        resultado.push(Acao::AAPL34(aapl34_ordens));
    }
    
    // 6. Retorne o vetor com os enums preenchidos.
    resultado
}