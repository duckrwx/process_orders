use serde::{Deserialize, Serialize};
use std::fs::File;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ordem {
    // --- CAMPOS PRIVADOS ---
    #[serde(rename = "Corretora")]
    pub corretora: String,
    #[serde(rename = "Conta")]
    pub conta: i64,
    #[serde(rename = "Titular")]
    pub titular: String,
    #[serde(rename = "ClOrdID")]
    pub cl_ord_id: String,
    #[serde(rename = "Ativo")]
    pub ativo: String,
    #[serde(rename = "Lado")]
    pub lado: String,

    // --- CAMPOS PÚBLICOS ---
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Criação")]
    pub criacao: String,
    #[serde(rename = "Última Atualização")]
    pub ultima_atualizacao: String,
    #[serde(rename = "Preço")]
    pub preco: String,
    #[serde(rename = "Preço Stop")]
    pub preco_stop: String,
    #[serde(rename = "Qtd")]
    pub qtd: i64,
    #[serde(rename = "Preço Médio")]
    pub preco_medio: String,
    #[serde(rename = "Qtd Executada")]
    pub qtd_executada: i64,
    #[serde(rename = "Qtd restante")]
    pub qtd_restante: i64,
    #[serde(rename = "Total")]
    pub total: String,
    #[serde(rename = "Total Executado")]
    pub total_executado: String,
    #[serde(rename = "Validade")]
    pub validade: String,
    #[serde(rename = "Data Validade")]
    pub data_validade: String,
    #[serde(rename = "Estratégia")]
    pub estrategia: String,
    #[serde(rename = "Mensagem")]
    pub mensagem: String,
    #[serde(rename = "Carteira")]
    pub carteira: String,
}
impl Ordem {
    pub fn stream_from_csv(path: &str) -> Result<impl Iterator<Item = Result<Ordem, csv::Error>>, Box<dyn Error>> {
        {
            let reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_path(path)?;
            Ok(reader.into_deserialize())
        }
    }
}
