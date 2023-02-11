struct Body {
    weight: f64,
    height: f64,
}

fn main() {
    let ichiro = Body {
        weight: 80.0,
        height: 154.0,
    };
    let jiro = Body {
        weight: 65.0,
        height: 170.0,
    };
}

fn calc_bmi(body: &body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0);
}
