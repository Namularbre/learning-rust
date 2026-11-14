# Exercice 1 : Conversion de températures

**Notions :** fonctions, types `f64` et `i32`, boucles `for`, intervalles, affichage formaté avec `println!`.

**Chapitres du livre :** 3.2 (types de données), 3.3 (fonctions), 3.5 (structures de contrôle).

## Consigne

Dans `src/main.rs`, complète :

1. `celsius_vers_fahrenheit(celsius: f64) -> f64`
   Formule : `F = C × 9 / 5 + 32`
2. `fahrenheit_vers_celsius(fahrenheit: f64) -> f64`
   Formule : `C = (F − 32) × 5 / 9`
3. `main` : affiche un tableau de conversion de **−10 °C à 40 °C, par pas de 5 °C**, avec une décimale pour les Fahrenheit.

## Résultat attendu

```text
$ cargo run -p exo-01-temperatures
Celsius | Fahrenheit
--------+-----------
    -10 |      14.0
     -5 |      23.0
      0 |      32.0
      5 |      41.0
     10 |      50.0
     15 |      59.0
     20 |      68.0
     25 |      77.0
     30 |      86.0
     35 |      95.0
     40 |     104.0
```

Et les tests doivent passer :

```text
$ cargo test -p exo-01-temperatures
test result: ok. 3 passed; 0 failed
```

## Indices

- Un intervalle qui avance de 5 en 5 : regarde la méthode `step_by` sur `(-10..=40)`.
- Pour convertir un `i32` en `f64` : `valeur as f64` (ou `f64::from(valeur)`, que clippy préfère parfois).
- Formatage : `{:>7}` aligne à droite sur 7 caractères, `{:.1}` affiche une décimale, et on peut combiner : `{:>10.1}`.

## Bonus

Écris une fonction `est_glacial(celsius: f64) -> bool` qui renvoie `true` si l'eau gèle (≤ 0 °C), et ajoute une colonne « Gel ? » au tableau.
