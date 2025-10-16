use serde::{Deserialize, Serialize};
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;
use encoding_rs_io::DecodeReaderBytesBuilder;
use encoding_rs::WINDOWS_1252;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Serialize, Clone)]   
pub struct Ordem {

    #[serde(rename = "Corretora")]
    pub corretora: Option<String>, 
    
    #[serde(rename = "Conta")]
    pub conta: Option<String>,  
    
    #[serde(rename = "Titular")]
    pub titular: Option<String>,  
    
    #[serde(rename = "ClOrdID")]
    pub cl_ord_id: Option<String>,  
    
    #[serde(rename = "Ativo")]
    pub ativo: Option<String>,  
    
    #[serde(rename = "Lado")]
    pub lado: Option<String>,  
    
    #[serde(rename = "Status")]
    pub status: String, 
    
    #[serde(rename = "Criação")]
    pub criacao: String,
    
    #[serde(rename = "Última Atualização")]
    pub ultima_atualizacao: Option<String>,  
    
    #[serde(rename = "Preço")]
    pub preco: Option<String>,
    
    #[serde(rename = "Preço Stop")]
    pub preco_stop: Option<String>,
    
    #[serde(rename = "Qtd")]
    pub qtd: Option<String>,
    
    #[serde(rename = "Preço Médio")]
    pub preco_medio: Option<String>,
    
    #[serde(rename = "Qtd Executada")]
    pub qtd_executada: Option<String>,
    
    #[serde(rename = "Qtd restante")]
    pub qtd_restante: Option<String>,
    
    #[serde(rename = "Total")]
    pub total: Option<String>,  
    
    #[serde(rename = "Total Executado")]
    pub total_executado: Option<String>,
    
    #[serde(rename = "Validade")]
    pub validade: Option<String>,  
    
    #[serde(rename = "Data Validade")]
    pub data_validade: Option<String>,
    
    #[serde(rename = "Estratégia")]
    pub estrategia: Option<String>,  
    
    #[serde(rename = "Mensagem")]
    pub mensagem: Option<String>,
    
    #[serde(rename = "Carteira")]
    pub carteira: Option<String>,  
}

impl Ordem {
    pub fn stream_from_csv(path: String) -> Result<impl Iterator<Item = Result<Ordem, csv::Error>>, Box<dyn Error>> {
        let file = File::open(path)?;
        
        let transcoded = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file);
        
        let mut buf_reader = BufReader::new(transcoded);
        
        let mut first_line = String::new();
        buf_reader.read_line(&mut first_line)?;
        
        let reader = ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(buf_reader);
        
        Ok(reader.into_deserialize())
    }
}
