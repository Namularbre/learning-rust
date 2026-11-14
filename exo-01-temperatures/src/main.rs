/// Convertit une température de degrés Celsius en degrés Fahrenheit.
fn celsius_vers_fahrenheit(celsius: f64) -> f64 {
    todo!("convertir {celsius} °C en °F")
}

/// Convertit une température de degrés Fahrenheit en degrés Celsius.
fn fahrenheit_vers_celsius(fahrenheit: f64) -> f64 {
    todo!("convertir {fahrenheit} °F en °C")
}

fn main() {
    // À toi : affiche le tableau de conversion décrit dans CONSIGNE.md.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eau_qui_gele() {
        assert_eq!(celsius_vers_fahrenheit(0.0), 32.0);
        assert_eq!(fahrenheit_vers_celsius(32.0), 0.0);
    }

    #[test]
    fn eau_qui_bout() {
        assert_eq!(celsius_vers_fahrenheit(100.0), 212.0);
        assert_eq!(fahrenheit_vers_celsius(212.0), 100.0);
    }

    #[test]
    fn aller_retour() {
        let depart = 37.5;
        let retour = fahrenheit_vers_celsius(celsius_vers_fahrenheit(depart));
        assert!((retour - depart).abs() < 1e-9);
    }
}
