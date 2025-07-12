pub fn estimate_price(mileage: f32, theta0: f32, theta1: f32) -> f32 {
    return theta0 + (theta1 * mileage);
}
