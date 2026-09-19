use steam::jogo::Jogo;

fn main() {
    let j = Jogo {
        app_id: 496350,
        name: String::from("Supipara"),
        release_date: String::from("Jul 29, 2016"),
        peak_ccu: 0,
        price: 5.24,
        positive: 252,
        negative: 3,
        recommendations: 231,
        genres: String::from("Adventure"),
    };
    println!("{:?}", j);
}
