use std::fs::File;
use std::io::Write;
use std::time::Instant;
use steam::arvore::Arvore;
use steam::aa::Aa;
use steam::bst::Bst;
use steam::jogo::carregar_csv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jogos = carregar_csv("data/games_enxuto.csv")?;
    let mut ficheiro_csv = File::create("metricas.csv")?;

    writeln!(
        ficheiro_csv, "algoritmo,cenario,quantidade_nos,altura,comparacoes,tempo_ms"
    )?;

    let tamanhos = [1000, 5000, 10000, 20000];

    for &n in &tamanhos {
        let subset = &jogos[..n.min(jogos.len())];

        //BST
        let mut bst = Bst::nova();
        let inicio = Instant::now();
        for j in subset {
            bst.inserir(j.app_id, j.clone());
        }
        let tempo = inicio.elapsed().as_secs_f64() * 1000.0; // Convertendo para milissegundos
        writeln!(
            ficheiro_csv,
            "BST,ordem do arquivo,{}, {}, {}, {:.3}",
            bst.tamanho(),bst.altura(),bst.contador_comparacoes(),
            tempo
        )?;

        //AA
        let mut aa = Aa::nova();
        let inicio = Instant::now();
        for j in subset {
            aa.inserir(j.app_id, j.clone());
        }
        let tempo_aa = inicio.elapsed().as_secs_f64() * 1000.0; // Convertendo para milissegundos
        writeln!(
            ficheiro_csv,
            "AA,ordem do arquivo,{}, {}, {}, {:.3}",
            aa.tamanho(),aa.altura(),aa.contador_comparacoes(),
            tempo_aa
        )?;
    }
    println!("Métricas salvas em metricas");
    Ok(())
}