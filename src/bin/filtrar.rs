//! Filtra o CSV do "Steam Games Dataset" e gera um CSV enxuto.
//!
//! O cabeçalho original do dataset tem 39 nomes, mas as linhas de dados têm 40
//! campos (a coluna "DiscountDLC count" são, na verdade, "Discount" e
//! "DLC count" coladas). Por isso este programa IGNORA o cabeçalho original e
//! lê os campos pela POSIÇÃO, escrevendo um cabeçalho novo e correto.
//!
//! Uso:
//!     cargo run --release -- games.csv games_enxuto.csv

use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::error::Error;

/// (posição do campo no arquivo original, nome da coluna no arquivo novo)
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

    // has_headers(false): o cabeçalho original vem como um registro comum,
    // que descartamos manualmente abaixo.
    // flexible(true): não falha se alguma linha tiver quantidade de campos
    // diferente das outras.
    let mut leitor = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(entrada)?;

    let mut escritor = WriterBuilder::new().from_path(saida)?;

    // Cabeçalho novo, com os nomes corretos.
    let novo_cabecalho: Vec<&str> = COLUNAS.iter().map(|(_, nome)| *nome).collect();
    escritor.write_record(&novo_cabecalho)?;

    let mut lidas: u64 = 0;
    let mut escritas: u64 = 0;
    let mut descartadas: u64 = 0;

    // skip(1) descarta o cabeçalho original (o desalinhado).
    for resultado in leitor.records().skip(1) {
        lidas += 1;

        // Linha com erro de parsing: descarta e segue.
        let registro = match resultado {
            Ok(r) => r,
            Err(_) => {
                descartadas += 1;
                continue;
            }
        };

        // O AppID precisa ser um número; senão a linha está corrompida.
        let app_id_valido = registro
            .get(COLUNAS[0].0)
            .map(|s| s.trim().parse::<u64>().is_ok())
            .unwrap_or(false);
        if !app_id_valido {
            descartadas += 1;
            continue;
        }

        // Campos ausentes viram string vazia.
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
