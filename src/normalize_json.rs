use crate::models::Ordem;

use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn normalize_json(input: &[Ordem]) -> Result<(), Box<dyn Error>> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let json_filename = format!("output/ordens_{}.json", timestamp);

    println!("📄 Salvando em: {}", json_filename);

    let json_str = serde_json::to_string_pretty(input)?;

    let mut file = File::create(&json_filename)?;

    file.write_all(json_str.as_bytes())?;

    println!("✅ JSON salvo com sucesso!");
    println!("📍 Arquivo: {}", json_filename);
    println!("📊 Tamanho: {} bytes", json_str.len());
    println!("\n🚀 Processo concluído!");

    Ok(())
}
pub fn normalize_by_action(ordens: &[Ordem], ativo_nome: &str) -> Result<(), Box<dyn Error>> {

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let json_filename = format!("output/ordens_{}_{}.json", ativo_nome, timestamp);
    
    println!("📄 Salvando {} ordens em: {}", ordens.len(), json_filename);
    
    let json_str = serde_json::to_string_pretty(ordens)?;

    let mut file = File::create(&json_filename)?;
    file.write_all(json_str.as_bytes())?;

    println!("✅ JSON salvo com sucesso!");
    Ok(())
}
