fn main() {
    let var1 = RocketShipType::Soyuz;
    let answer = var1.first_flight_in(1940);
    println!("{answer}");
}

enum RocketShipType {
    Buran,
    Energia,
    Soyuz,
    Bulava,
}

impl RocketShipType {
    fn first_flight_in(&self, year: u32) -> bool {
        match self {
            RocketShipType::Buran => year >= 1950,
            _ => year <= 1945,
        }
    }
}
