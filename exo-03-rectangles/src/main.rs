#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    // À toi : écris ici `new`, `carre`, `aire`, `perimetre`, `est_carre`
    // et `peut_contenir` (voir CONSIGNE.md).
}

fn main() {
    // À toi : crée r1, r2 et r3, puis affiche les résultats décrits dans CONSIGNE.md.
    todo!()
}

// Les tests sont commentés car ils utilisent des méthodes qui n'existent pas encore :
// le projet ne compilerait pas. Décommente-les (sélectionne puis Ctrl+/ dans la plupart
// des éditeurs) quand tes méthodes sont écrites, puis lance `cargo test -p exo-03-rectangles`.
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn constructeurs() {
//         let r = Rectangle::new(3, 4);
//         assert_eq!((r.largeur, r.hauteur), (3, 4));
//         let c = Rectangle::carre(5);
//         assert_eq!((c.largeur, c.hauteur), (5, 5));
//     }
//
//     #[test]
//     fn aire_et_perimetre() {
//         let r = Rectangle::new(30, 50);
//         assert_eq!(r.aire(), 1500);
//         assert_eq!(r.perimetre(), 160);
//     }
//
//     #[test]
//     fn carre_ou_pas() {
//         assert!(Rectangle::carre(7).est_carre());
//         assert!(!Rectangle::new(7, 8).est_carre());
//     }
//
//     #[test]
//     fn contenance() {
//         let grand = Rectangle::new(30, 50);
//         let petit = Rectangle::new(10, 40);
//         let meme_taille = Rectangle::new(30, 50);
//         assert!(grand.peut_contenir(&petit));
//         assert!(!petit.peut_contenir(&grand));
//         assert!(!grand.peut_contenir(&meme_taille));
//     }
// }
