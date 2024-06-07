use core::f32::consts::TAU;

/*
    Taylor-Maclaurin formula for sin(x) around 0:

    sin(x) ≈ x - (x^3) / 3! + (x^5) / 5! - (x^7) / 7! + ...

    By limiting the development to 4 terms, we obtain a reasonable approximation:

    sin(x) ≈ x - (x^3) / 6 + (x^5) / 120 - (x^7) / 5040
*/
pub fn sin(x: f32) -> f32 {
    let mut x = x % TAU;

    // if x is in [PI, TAU], use identity sin(x) = -sin(x - PI)
    if x > core::f32::consts::PI {
        x = TAU - x;
        return -(x - (pow(x, 3) / 6.0) + (pow(x, 5) / 120.0) - (pow(x, 7) / 5040.0));
    }

    // Use Taylor approximation in [0, PI]
    x - (pow(x, 3) / 6.0) + (pow(x, 5) / 120.0 - (pow(x, 7) / 5040.0))
}

pub fn pow(base: f32, exponent: i32) -> f32 {
    let mut result = 1.0;
    for _ in 0..exponent {
        result *= base;
    }
    result
}
