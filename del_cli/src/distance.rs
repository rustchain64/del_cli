pub fn calc_distance(Pickup_Coord, Delivery_Coord) -> f64 {
    //Calculate distance from latitude and longitude
    let theta = Pickup_Coord.long - Delivery_Coord.long;
    let dist = sin(deg2rad(Pickup_Coord.lat)) * sin(deg2rad(Delivery_Coord.lat)) +
    cos(deg2rad(Pickup_Coord.lat)) * cos(deg2rad(Delivery_Coord.lat)) *
    cos(deg2rad(theta));
    let dist = acos(dist);
    let dist = rad2deg(dist);
    let miles = dist * 60 * 1.8515;
    let unit = strtoupper(unit);
    // for units in KM
    let kilometers = round( miles * 1.609344, PHP_ROUND_HALF_UP ).' km';
}

// unit test
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(1, 2), 3);
    }
}