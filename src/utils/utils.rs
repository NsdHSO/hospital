use nanoid::nanoid;

pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS: f64 = 6371.0;
    let (lat1_rad, lon1_rad) = (lat1.to_radians(), lon1.to_radians());
    let (lat2_rad, lon2_rad) = (lat2.to_radians(), lon2.to_radians());
    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    let a =
        (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS * c
}
pub fn generate_ic() -> i32 {
    generate_ic_with_length(Some(30)) // Default to 6 digits
}
pub fn generate_ic_with_length(length: Option<usize>) -> i32 {
    let length = length.unwrap_or(30);
    // Generate numeric string using nanoid
    let numeric_string = nanoid!(length, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);

    // Ensure the string isn't too long for i32 (max 10 digits for i32)
    let safe_length = std::cmp::min(length, 9);
    let trimmed_string = if numeric_string.len() > safe_length {
        &numeric_string[0..safe_length]
    } else {
        &numeric_string
    };

    // Parse the string into an i32, with a fallback in case of errors
    trimmed_string.parse::<i32>().unwrap_or_else(|_| {
        // If parsing fails (very unlikely), generate a simpler fallback
        let fallback = nanoid!(4, &['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        fallback.parse::<i32>().unwrap_or(1000) // Final fallback value
    })
}