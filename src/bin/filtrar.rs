//! Uso:
//!     cargo run --release -- games.csv games_enxuto.csv

use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::error::Error;

const COLUNAS: [(usize, &str); 9] = [
    (0, "AppID"),
    (1, "Name"),
    (2, "ReleaseDate"),
    (4, "PeakCCU"),
    (6, "Price"),
    (23, "Positive"),
    (24, "Negative"),
    (27, "Recommendations"),
    (36, "Genres"),
];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let entrada = args.get(1).map(String::as_str).unwrap_or("games.csv");
    let saida = args.get(2).map(String::as_str).unwrap_or("games_enxuto.csv");

    let mut leitor = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(entrada)?;

    let mut escritor = WriterBuilder::new().from_path(saida)?;


    let novo_cabecalho: Vec<&str> = COLUNAS.iter().map(|(_, nome)| *nome).collect();
    escritor.write_record(&novo_cabecalho)?;

    let mut lidas: u64 = 0;
    let mut escritas: u64 = 0;
    let mut descartadas: u64 = 0;


    for resultado in leitor.records().skip(1) {
        lidas += 1;


        let registro = match resultado {
            Ok(r) => r,
            Err(_) => {
                descartadas += 1;
                continue;
            }
        };


        let app_id_valido = registro
            .get(COLUNAS[0].0)
            .map(|s| s.trim().parse::<u64>().is_ok())
            .unwrap_or(false);
        if !app_id_valido {
            descartadas += 1;
            continue;
        }


        let linha: Vec<&str> = COLUNAS
            .iter()
            .map(|(pos, _)| registro.get(*pos).unwrap_or(""))
            .collect();

        escritor.write_record(&linha)?;
        escritas += 1;
    }

    escritor.flush()?;

    println!("Linhas de dados lidas: {lidas}");
    println!("Linhas escritas:       {escritas}");
    println!("Linhas descartadas:    {descartadas}");
    println!("Arquivo gerado:        {saida}");

    Ok(())
}
