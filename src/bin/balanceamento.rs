// Mede a ALTURA de cada árvore enquanto os jogos vão sendo inseridos.
//
// Ideia: insere os jogos um por um; a cada `passo` inserções (500, 1000...)
// anota quantos jogos já entraram e qual a altura da árvore naquele momento.
// No fim grava um CSV com as colunas: n, bst, aa, avl
// (esse CSV é lido por scripts/gerar_graficos.py para desenhar o gráfico).
//
// Como rodar (na raiz do projeto):
//   cargo run --release --bin balanceamento
//   cargo run --release --bin balanceamento -- data/games_enxuto.csv 500 arquivo data/balanceamento.csv
//
// Argumentos (todos opcionais, nessa ordem):
//   1) caminho do CSV de jogos        (padrão: data/games_enxuto.csv)
//   2) passo de medição               (padrão: 1000)
//   3) "arquivo" ou "ordenado"        (padrão: arquivo)
//        arquivo  = ordem em que os jogos estão no CSV
//        ordenado = jogos ordenados por AppID (pior caso da BST)
//   4) caminho do CSV de saída        (padrão: data/balanceamento.csv)

use std::fs::File;
use std::io::Write;

use steam::aa::Aa;
use steam::arvore::Arvore;
use steam::avl::Avl;
use steam::bst::Bst;
use steam::jogo::{carregar_csv, Jogo};


fn medir_alturas(arvore: &mut dyn Arvore, jogos: &[&Jogo], passo: usize) -> Vec<(usize, usize)> {
    let mut pontos = Vec::new();

    for (i, jogo) in jogos.iter().enumerate() {
        arvore.inserir(jogo.app_id, (*jogo).clone());

        let inseridos = i + 1;

        if inseridos % passo == 0 || inseridos == jogos.len() {
            pontos.push((inseridos, arvore.altura()));
        }
    }

    pontos
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let caminho = args.get(1).cloned().unwrap_or_else(|| "data/games_enxuto.csv".to_string());
    let passo: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let modo = args.get(3).cloned().unwrap_or_else(|| "arquivo".to_string());
    let saida = args.get(4).cloned().unwrap_or_else(|| "data/balanceamento.csv".to_string());

    if passo == 0 {
        eprintln!("O passo precisa ser maior que 0.");
        return;
    }
    if modo != "arquivo" && modo != "ordenado" {
        eprintln!("Modo inválido: {modo}. Use \"arquivo\" ou \"ordenado\".");
        return;
    }

    let jogos = match carregar_csv(&caminho) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Erro ao ler {caminho}: {e}");
            return;
        }
    };

    let mut lista: Vec<&Jogo> = jogos.iter().collect();
    if modo == "ordenado" {
        lista.sort_by_key(|j| j.app_id);
    }
    println!("{} jogos | passo {} | modo {}", lista.len(), passo, modo);

    
    let mut bst = Bst::nova();
    let mut aa = Aa::nova();
    let mut avl = Avl::nova();

    let alturas_bst = medir_alturas(&mut bst, &lista, passo);
    let alturas_aa = medir_alturas(&mut aa, &lista, passo);
    let alturas_avl = medir_alturas(&mut avl, &lista, passo);

    
    let mut arquivo = match File::create(&saida) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Não consegui criar {saida}: {e}");
            return;
        }
    };
    writeln!(arquivo, "n,bst,aa,avl").unwrap();
    for i in 0..alturas_bst.len() {
        let (n, h_bst) = alturas_bst[i];
        let (_, h_aa) = alturas_aa[i];
        let (_, h_avl) = alturas_avl[i];
        writeln!(arquivo, "{n},{h_bst},{h_aa},{h_avl}").unwrap();
    }

    let (n, h_bst) = *alturas_bst.last().unwrap();
    let (_, h_aa) = *alturas_aa.last().unwrap();
    let (_, h_avl) = *alturas_avl.last().unwrap();
    println!("Altura final com {n} jogos: BST {h_bst} | AA {h_aa} | AVL {h_avl}");
    println!("{} medições gravadas em {saida}", alturas_bst.len());
}
