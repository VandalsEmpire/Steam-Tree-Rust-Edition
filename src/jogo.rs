use std::error::Error;

#[derive(Debug, Clone)]
pub struct Jogo {
    pub app_id: u32,
    pub name: String,
    pub release_date: String,
    pub peak_ccu: u32,
    pub price: f32,
    pub positive: u32,
    pub negative: u32,
    pub recommendations: u32,
    pub genres: String,
}

pub fn carregar_csv(caminho: &str) -> Result<Vec<Jogo>, Box<dyn Error>> {
    let mut leitor = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(caminho)?;

    let mut jogos = Vec::new();

    for resultado in leitor.records() {
        let r = match resultado {
            Ok(r) => r,
            Err(_) => continue,
        };

        let app_id = match r.get(0).and_then(|s| s.trim().parse::<u32>().ok()) {
            Some(id) => id,
            None => continue,
        };

        jogos.push(Jogo {
            app_id,
            name: r.get(1).unwrap_or("").to_string(),
            release_date: r.get(2).unwrap_or("").to_string(),
            peak_ccu: como_u32(r.get(3)),
            price: r.get(4).and_then(|s| s.trim().parse().ok()).unwrap_or(0.0),
            positive: como_u32(r.get(5)),
            negative: como_u32(r.get(6)),
            recommendations: como_u32(r.get(7)),
            genres: r.get(8).unwrap_or("").to_string(),
        });
    }

    Ok(jogos)
}

fn como_u32(campo: Option<&str>) -> u32 {
    campo.and_then(|s| s.trim().parse().ok()).unwrap_or(0)
}
