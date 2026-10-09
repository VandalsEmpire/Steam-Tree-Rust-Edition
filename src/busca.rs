use crate::arvore::Arvore;
use crate::jogo::Jogo;

pub struct ResultadoBusca<'a> {
    pub jogo: Option<&'a Jogo>,
    pub comparacoes: u64,
    pub tempo_micros: u128,
}

pub fn buscar_jogo<'a>(arvore: &'a mut dyn Arvore, app_id: u32) -> ResultadoBusca<'a> {
    arvore.zerar_contador();
    let inicio = std::time::Instant::now();
    let jogo = arvore.buscar(app_id);
    let tempo_micros = inicio.elapsed().as_micros();
    let comparacoes = arvore.contador_comparacoes();

    ResultadoBusca {
        jogo,
        comparacoes,
        tempo_micros,
    }
}