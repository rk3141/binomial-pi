struct Term {
    coef: f64,
    exp: f64,
}

fn eval(x: f64, terms: &Vec<Term>) -> f64 {
    let mut result = 0.0;
    // Solve polynomial
    for term in terms {
        result += term.coef * x.powf(term.exp)
    }
    result
}

fn integrate(from: f64, to: f64, terms: Vec<Term>) -> f64 {
    let terms = terms
        .iter()
        .map(|v| Term {
            // Raise the exponent and divide the coefficient by the new exponent
            coef: v.coef / (v.exp + 1.0),
            exp: v.exp + 1.0,
        })
        .collect();

    eval(to, &terms) - eval(from, &terms)
}

fn factorial(n: f64) -> f64 {
    if n != 0.0 {
        n * factorial(n - 1.0)
    } else {
        1.0
    }
}

fn falling_factorial(base: f64, count: u32) -> f64 {
    // https://en.wikipedia.org/wiki/Falling_and_rising_factorials
    let mut result = 1.0;

    for k in 0..count {
        result *= -1.0 * (base - k as f64); // -1 for the -x^2
    }

    result / factorial(count as f64)
}

fn generate_terms() -> Vec<Term> {
    let base = 1.0 / 2.0;
    let mut terms = vec![];

    for k in 0..173 {
        // polynomial approximation of f(x) = sqrt (1 - x^2)
        terms.push(Term {
            coef: falling_factorial(base, k),
            exp: 2.0 * k as f64, // multiply by 2 for the x^2
        })
    }

    // Subtract the triangle formed by y = sqrt 3 * x
    terms.push(Term {
        coef: -3.0f64.powf(0.5),
        exp: 1.0,
    });

    terms
}

fn main() {
    // Approximation of PI to whatever precision possible with f64
    let terms = generate_terms();

    println!("{}", 12.0 * (integrate(0.0, 0.5, terms)));
}
