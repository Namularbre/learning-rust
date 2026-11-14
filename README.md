# Apprendre le Rust

Une série d'exercices progressifs pour apprendre le Rust, organisés en workspace Cargo.
Chaque dossier `exo-XX-...` est un petit projet indépendant.

## Comment travailler

1. Lis la consigne : `exo-XX-.../CONSIGNE.md`.
2. Complète `exo-XX-.../src/main.rs` (remplace les `todo!()` par ton code).
3. Vérifie ton travail depuis la racine du dépôt :

```sh
cargo run -p exo-01-temperatures     # lance le programme
cargo test -p exo-01-temperatures    # lance les tests fournis
cargo fmt                            # formate le code
cargo clippy -p exo-01-temperatures  # conseils de style et pièges courants
```

4. Envoie ta réponse pour la correction : avis sur le code, plus la sortie de `rustfmt` et `cargo clippy`.

Au départ, les tests échouent avec `not yet implemented` et le compilateur signale du code « never used » : c'est normal, ces avertissements disparaissent quand tu complètes l'exercice. À toi de jouer !

## Exercices

| N° | Dossier | Notions |
|----|---------|---------|
| 1 | [exo-01-temperatures](exo-01-temperatures/CONSIGNE.md) | fonctions, types numériques, boucles `for`, affichage formaté |
| 2 | [exo-02-chaines](exo-02-chaines/CONSIGNE.md) | `&str` et `String`, emprunt, itération sur les caractères |
| 3 | [exo-03-rectangles](exo-03-rectangles/CONSIGNE.md) | `struct`, `impl`, méthodes, fonctions associées |

## Ressources

- [Le livre Rust en français](https://jimskapt.github.io/rust-book-fr/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) (en anglais)
