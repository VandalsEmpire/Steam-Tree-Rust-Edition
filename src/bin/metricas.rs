// Mede, para cada árvore, o TEMPO e o número de COMPARAÇÕES de cada operação:
// inserir, buscar e remover.
//
// Como funciona (para cada cenário e para cada árvore):
//   1) INSERIR: insere todos os jogos e mede o tempo e as comparações.
//   2) BUSCAR:  busca uma amostra de chaves que existem na árvore.
//   3) REMOVER: remove essa mesma amostra de chaves.
// As 3 árvores recebem os mesmos jogos e a mesma amostra de chaves.
//
// Dois cenários:
//   "arquivo"  = todos os jogos, na ordem em que estão no CSV
//   "ordenado" = os 20.000 primeiros jogos ordenados por AppID (pior caso da BST)
//
// No fim grava um CSV (padrão: data/metricas.csv) que é lido por
// scripts/gerar_graficos.py para desenhar os gráficos de desempenho.
//
// Como rodar (na raiz do projeto, SEMPRE com --release para o tempo ser real):
//   cargo run --release --bin metricas
//   cargo run --release --bin metricas -- data/games_enxuto.csv data/metricas.csv 10000
//
// Argumentos (todos opcionais, nessa ordem):
//   1) caminho do CSV de jogos          (padrão: data/games_enxuto.csv)
//   2) caminho do CSV de saída          (padrão: data/metricas.csv)
//   3) tamanho da amostra de buscas e remoções (padrão: 10000)

use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

use steam::aa::Aa;
use steam::arvore::Arvore;
use steam::avl::Avl;
use steam::bst::Bst;
use steam::jogo::{carregar_csv, Jogo};

/// Uma linha do CSV de saída: o resultado de UMA operação em UMA árvore.
struct Medicao {
    cenario: &'static str,
    arvore: &'static str,
    operacao: &'static str,
    operacoes: usize,
    comparacoes: u64,
    tempo_ms: f64,
}

/// Roda `trabalho`, mede o tempo e as comparações que ele gastou na árvore.
/// (zera o contador antes e lê depois, assim só conta o que `trabalho` fez)
fn medir(
    arvore: &mut dyn Arvore,
    cenario: &'static str,
    operacao: &'static str,
    operacoes: usize,
    trabalho: impl FnOnce(&mut dyn Arvore),
) -> Medicao {
    arvore.zerar_contador();
    let inicio = Instant::now();
    trabalho(arvore);
    let tempo = inicio.elapsed();

    Medicao {
        cenario,
        arvore: arvore.nome(),
        operacao,
        operacoes,
        comparacoes: arvore.contador_comparacoes(),
        tempo_ms: tempo.as_secs_f64() * 1000.0,
    }
}

/// Faz as 3 operações em uma árvore e devolve as 3 medições.
fn testar_arvore(
    arvore: &mut dyn Arvore,
    cenario: &'static str,
    jogos: &[&Jogo],
    amostra: &[u32],
) -> Vec<Medicao> {
    let mut medicoes = Vec::new();

    // 1) INSERIR todos os jogos
    medicoes.push(medir(arvore, cenario, "inserir", jogos.len(), |a| {
        for j in jogos {
            a.inserir(j.app_id, (*j).clone());
        }
    }));

    // 2) BUSCAR a amostra (black_box impede o compilador de "pular" a busca)
    medicoes.push(medir(arvore, cenario, "buscar", amostra.len(), |a| {
        for &chave in amostra {
            black_box(a.buscar(chave));
        }
    }));

    // 3) REMOVER a amostra
    medicoes.push(medir(arvore, cenario, "remover", amostra.len(), |a| {
        for &chave in amostra {
            black_box(a.remover(chave));
        }
    }));

    // confere: depois de remover a amostra, tem que sobrar (inseridos - removidos)
    let esperado = jogos.len() - amostra.len();
    if arvore.tamanho() != esperado {
        eprintln!(
            "AVISO [{}] {}: sobraram {} nós, esperado {}",
            arvore.nome(),
            cenario,
            arvore.tamanho(),
            esperado
        );
    }

    medicoes
}

/// Escolhe `quantas` chaves espalhadas por toda a lista (de tantos em tantos).
/// Todas existem na árvore, então toda busca e remoção "acha" a chave.
fn escolher_amostra(jogos: &[&Jogo], quantas: usize) -> Vec<u32> {
    let quantas = quantas.min(jogos.len());
    let passo = (jogos.len() / quantas).max(1);
    jogos.iter().step_by(passo).take(quantas).map(|j| j.app_id).collect()
}

fn rodar_cenario(cenario: &'static str, jogos: &[&Jogo], tamanho_amostra: usize) -> Vec<Medicao> {
    let amostra = escolher_amostra(jogos, tamanho_amostra);
    println!("Cenário \"{cenario}\": {} jogos, amostra de {} chaves", jogos.len(), amostra.len());

    let mut todas = Vec::new();
    todas.extend(testar_arvore(&mut Bst::nova(), cenario, jogos, &amostra));
    todas.extend(testar_arvore(&mut Aa::nova(), cenario, jogos, &amostra));
    todas.extend(testar_arvore(&mut Avl::nova(), cenario, jogos, &amostra));

    for m in &todas {
        println!(
            "  [{:<4}] {:<8} {:>7} ops | {:>11} comparações | {:>10.2} ms",
            m.arvore, m.operacao, m.operacoes, m.comparacoes, m.tempo_ms
        );
    }
    todas
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let caminho = args.get(1).cloned().unwrap_or_else(|| "data/games_enxuto.csv".to_string());
    let saida = args.get(2).cloned().unwrap_or_else(|| "data/metricas.csv".to_string());
    let tamanho_amostra: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(10_000);

    if tamanho_amostra == 0 {
        eprintln!("A amostra precisa ser maior que 0.");
        return;
    }

    let jogos = match carregar_csv(&caminho) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Erro ao ler {caminho}: {e}");
            return;
        }
    };

    // Cenário 1: todos os jogos, na ordem do arquivo
    let todos: Vec<&Jogo> = jogos.iter().collect();

    // Cenário 2: os 20.000 primeiros, ordenados por AppID (pior caso da BST)
    let mut ordenados: Vec<&Jogo> = jogos.iter().take(20_000).collect();
    ordenados.sort_by_key(|j| j.app_id);

    let mut resultado = rodar_cenario("arquivo", &todos, tamanho_amostra);
    resultado.extend(rodar_cenario("ordenado", &ordenados, tamanho_amostra));

    let mut arquivo = match File::create(&saida) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Não consegui criar {saida}: {e}");
            return;
        }
    };
    writeln!(
        arquivo,
        "cenario,arvore,operacao,operacoes,comparacoes,comparacoes_por_op,tempo_ms,tempo_us_por_op"
    )
    .unwrap();
    for m in &resultado {
        let comp_por_op = m.comparacoes as f64 / m.operacoes as f64;
        let us_por_op = m.tempo_ms * 1000.0 / m.operacoes as f64;
        writeln!(
            arquivo,
            "{},{},{},{},{},{:.3},{:.3},{:.4}",
            m.cenario, m.arvore, m.operacao, m.operacoes, m.comparacoes, comp_por_op, m.tempo_ms, us_por_op
        )
        .unwrap();
    }
    println!("{} medições gravadas em {saida}", resultado.len());
}
