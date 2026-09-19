use std::time::Instant;
use steam::arvore::Arvore;
use steam::bst::Bst;
use steam::jogo::{carregar_csv, Jogo};

fn medir_insercao(arvore: &mut dyn Arvore, jogos: &[&Jogo], rotulo: &str) {
    let inicio = Instant::now();
    for j in jogos {
        arvore.inserir(j.app_id, (*j).clone());
    }
    let tempo = inicio.elapsed();
    println!(
        "[{}] {}: {} nós | altura {} | {} comparações | {:.1?}",
        arvore.nome(),
        rotulo,
        arvore.tamanho(),
        arvore.altura(),
        arvore.contador_comparacoes(),
        tempo
    );
}

fn main() {
    let caminho = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/games_enxuto.csv".to_string());

    let jogos = match carregar_csv(&caminho) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Erro ao ler {caminho}: {e}");
            return;
        }
    };
    println!("{} jogos carregados de {caminho}", jogos.len());

    let todos: Vec<&Jogo> = jogos.iter().collect();
    let mut bst = Bst::nova();
    medir_insercao(&mut bst, &todos, "ordem do arquivo");

    if let Some(primeiro) = jogos.first() {
        bst.zerar_contador();
        let achou = bst.buscar(primeiro.app_id).map(|j| j.name.clone());
        println!(
            "busca por {}: {:?} ({} comparações)",
            primeiro.app_id,
            achou,
            bst.contador_comparacoes()
        );
    }

    let mut ordenados: Vec<&Jogo> = jogos.iter().take(20_000).collect();
    ordenados.sort_by_key(|j| j.app_id);
    let mut bst_ord = Bst::nova();
    medir_insercao(&mut bst_ord, &ordenados, "chaves ordenadas (pior caso)");
}
