# Exercice 2 : Jouer avec les chaînes de caractères

**Notions :** `&str` et `String`, emprunt (références), itération avec `.chars()`, `match`.

**Chapitres du livre :** 4.1 et 4.2 (possession et références), 4.3 (slices), 8.2 (`String`).

## Consigne

Dans `src/main.rs`, complète ces trois fonctions. Remarque bien leurs signatures : elles **empruntent** le texte (`&str`) au lieu d'en prendre possession.

1. `compter_voyelles(texte: &str) -> usize`
   Compte les voyelles `a e i o u y`, minuscules **et** majuscules. On ignore les accents (`é` ne compte pas).
2. `premier_mot(texte: &str) -> &str`
   Renvoie le premier mot (tout ce qui précède le premier espace). Si le texte ne contient pas d'espace, renvoie le texte entier. Le résultat est une *slice* du texte d'origine : pas de nouvelle `String` !
3. `inverser(texte: &str) -> String`
   Renvoie le texte à l'envers. Ici on doit créer une nouvelle `String`.

Le `main` fourni appelle déjà tes fonctions sur quelques phrases : ne le modifie pas (sauf pour le bonus).

## Résultat attendu

```text
$ cargo run -p exo-02-chaines
« Bonjour le monde »
  voyelles    : 6
  premier mot : Bonjour
  à l'envers  : ednom el ruojnoB
« Rust »
  voyelles    : 1
  premier mot : Rust
  à l'envers  : tsuR
« AEIOUY et xyz »
  voyelles    : 8
  premier mot : AEIOUY
  à l'envers  : zyx te YUOIEA
```

```text
$ cargo test -p exo-02-chaines
test result: ok. 3 passed; 0 failed
```

## Indices

- `texte.chars()` donne un itérateur sur les caractères. Une boucle `for c in texte.chars()` suffit pour commencer.
- `match c { 'a' | 'e' | ... => ..., _ => ... }` permet de tester plusieurs valeurs d'un coup. `c.to_ascii_lowercase()` peut t'éviter d'écrire les majuscules.
- Pour `premier_mot`, regarde `texte.find(' ')` (renvoie une `Option<usize>`) et la syntaxe de slice `&texte[..i]`.
- Pour `inverser`, les itérateurs ont une méthode `.rev()` et peuvent être rassemblés avec `.collect()`.

## Bonus

Quand tes fonctions marchent avec des boucles, réécris `compter_voyelles` en une seule chaîne d'itérateurs avec `.filter(...)` et `.count()`.
