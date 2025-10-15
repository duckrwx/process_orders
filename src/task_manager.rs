use std::error::Error;
use crate::models::Ordem;
pub struct TaskManager {
    pub tasks: Vec<Ordem>,
}
impl TaskManager {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let loaded_orders = Ordem::stream_from_csv(file_path)?.collect::<Result<Vec<Ordem>, csv::Error>>()?;
        println!("Total de ordens carregadas: {}", loaded_orders.len());
        Ok(Self { tasks: loaded_orders })
    }
    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }
}