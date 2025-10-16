use crate::models::Ordem;

use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn normalize_json(input: &[Ordem]) -> Result<(), Box<dyn Error>> {
    // Nome do arquivo com timestamp
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let json_filename = format!("output/ordens_{}.json", timestamp);

    println!("📄 Salvando em: {}", json_filename);

    // 1. Serializa os dados para uma string JSON. O `?` cuida do erro.
    let json_str = serde_json::to_string_pretty(input)?;

    // 2. Cria o arquivo. O `?` cuida do erro.
    let mut file = File::create(&json_filename)?;

    // 3. Escreve a string no arquivo. O `?` cuida do erro.
    file.write_all(json_str.as_bytes())?;

    // Se o código chegou até aqui, tudo deu certo!
    println!("✅ JSON salvo com sucesso!");
    println!("📍 Arquivo: {}", json_filename);
    println!("📊 Tamanho: {} bytes", json_str.len());
    println!("\n🚀 Processo concluído!");

    // Retorna `Ok` para indicar sucesso.
    Ok(())
}
pub fn normalize_by_action(ordens: &[Ordem], ativo_nome: &str) -> Result<(), Box<dyn Error>> {

    // Nome do arquivo com timestamp e o nome do ativo
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    // 👇 MUDANÇA AQUI: Incorporamos o nome do ativo no nome do arquivo
    let json_filename = format!("output/ordens_{}_{}.json", ativo_nome, timestamp);
    
    println!("📄 Salvando {} ordens em: {}", ordens.len(), json_filename);
    
    // Serializa os dados para uma string JSON. O `?` cuida do erro.
    let json_str = serde_json::to_string_pretty(ordens)?;

    // Cria e escreve no arquivo.
    let mut file = File::create(&json_filename)?;
    file.write_all(json_str.as_bytes())?;

    println!("✅ JSON salvo com sucesso!");
    Ok(())
}