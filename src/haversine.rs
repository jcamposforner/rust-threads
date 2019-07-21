struct Haversine {
    lat1: f64,
    lat2: f64,
    lon1: f64,
    lon2: f64,
}

impl Haversine {
    fn distance(&self) -> f64 {
        let r = 6371.00;
        let lat1_rad = self.lat1.to_radians();
        let lat2_rad = self.lat2.to_radians();
        let lon1_rad = self.lon1.to_radians();
        let lon2_rad = self.lon2.to_radians();

        let a = ( (lat2_rad - lat1_rad) / 2.00).sin().powf(2.00) + lat1_rad.cos() * lat2_rad.cos() * ( (lon2_rad - lon1_rad) / 2.00).sin().powf(2.00);
        let c = 2.00 * ((a).sqrt().atan2((1.00 - a).sqrt()));
        r * c  
    }
}

fn main() {
    let a = Haversine { lat1:38.898556, lat2:38.897147, lon1:-77.037852, lon2:-77.043934};
    println!("{}", a.distance());
}