/// Compte les voyelles (a, e, i, o, u, y), minuscules et majuscules.
fn compter_voyelles(texte: &str) -> usize {
    todo!("compter les voyelles de {texte:?}")
}

/// Renvoie le premier mot du texte (avant le premier espace).
fn premier_mot(texte: &str) -> &str {
    todo!("trouver le premier mot de {texte:?}")
}

/// Renvoie le texte à l'envers.
fn inverser(texte: &str) -> String {
    todo!("inverser {texte:?}")
}

fn main() {
    let phrases = ["Bonjour le monde", "Rust", "AEIOUY et xyz"];

    for phrase in phrases {
        println!("« {phrase} »");
        println!("  voyelles    : {}", compter_voyelles(phrase));
        println!("  premier mot : {}", premier_mot(phrase));
        println!("  à l'envers  : {}", inverser(phrase));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_voyelles() {
        assert_eq!(compter_voyelles(""), 0);
        assert_eq!(compter_voyelles("bcd"), 0);
        assert_eq!(compter_voyelles("Bonjour"), 3);
        assert_eq!(compter_voyelles("AEIOUY"), 6);
    }

    #[test]
    fn test_premier_mot() {
        assert_eq!(premier_mot("Bonjour le monde"), "Bonjour");
        assert_eq!(premier_mot("Rust"), "Rust");
        assert_eq!(premier_mot(""), "");
    }

    #[test]
    fn test_inverser() {
        assert_eq!(inverser("Rust"), "tsuR");
        assert_eq!(inverser(""), "");
        assert_eq!(inverser("été"), "été");
        assert_eq!(inverser("abc def"), "fed cba");
    }
}
