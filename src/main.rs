mod haversine;

fn main() {
    let a = haversine::Haversine { lat1:38.898556, lat2:38.897147, lon1:-77.037852, lon2:-77.043934};
    println!("{}", a.distance());
}