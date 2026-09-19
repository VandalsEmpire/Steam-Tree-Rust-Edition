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
