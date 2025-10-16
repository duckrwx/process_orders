
use crate::models::Ordem;

pub enum Acao {
    COGN3(Vec<Ordem>),
    WINV25(Vec<Ordem>),
    AAPL34(Vec<Ordem>)
}
// pub enum Estrategia {
//     RAIZ_RapidoMacd5_2St,
//     RAIZacoes_RapidoMacd12,
//     RAIZacoes_RapidoMacd15
// }
// pub enum lado {
//     COMPRA,
//     VENDA
// }
