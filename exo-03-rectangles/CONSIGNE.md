# Exercice 3 : Des rectangles avec `struct` et `impl`

**Notions :** définir une `struct`, ajouter des méthodes avec `impl`, `&self`, fonctions associées (constructeurs), `#[derive(Debug)]`.

**Chapitres du livre :** 5.1 à 5.3 (structures et méthodes).

## Consigne

La structure `Rectangle` est déjà déclarée dans `src/main.rs` avec deux champs `largeur` et `hauteur` (de type `u32`). Complète le bloc `impl Rectangle` :

1. `fn new(largeur: u32, hauteur: u32) -> Rectangle` : constructeur.
2. `fn carre(cote: u32) -> Rectangle` : crée un rectangle dont les deux côtés valent `cote`.
3. `fn aire(&self) -> u32`
4. `fn perimetre(&self) -> u32`
5. `fn est_carre(&self) -> bool`
6. `fn peut_contenir(&self, autre: &Rectangle) -> bool` : `true` si `autre` tient **strictement** à l'intérieur de `self`, sans le faire pivoter (sa largeur et sa hauteur sont toutes les deux strictement plus petites).

Puis, dans `main`, crée les trois rectangles ci-dessous et affiche le résultat attendu :

- `r1` : 30 × 50
- `r2` : 10 × 40
- `r3` : un carré de côté 60

## Résultat attendu

```text
$ cargo run -p exo-03-rectangles
r1 = Rectangle { largeur: 30, hauteur: 50 }
r1 : aire = 1500, périmètre = 160, carré ? false
r3 : aire = 3600, périmètre = 240, carré ? true
r1 peut contenir r2 ? true
r1 peut contenir r3 ? false
r3 peut contenir r1 ? true
```

```text
$ cargo test -p exo-03-rectangles
test result: ok. 4 passed; 0 failed
```

## Indices

- `#[derive(Debug)]` (déjà présent) permet d'afficher la structure avec `{:?}`.
- Dans une méthode, `self.largeur` accède au champ. `Self` est un raccourci pour le type `Rectangle`.
- Quand le nom d'une variable est identique au nom du champ, on peut écrire `Rectangle { largeur, hauteur }`.
- Une fonction associée sans `self` s'appelle avec `Rectangle::carre(60)`.

## Bonus

Ajoute une méthode `fn agrandir(&mut self, facteur: u32)` qui multiplie les deux côtés par `facteur`. Pourquoi faut-il `&mut self` ici et pas `&self` ? Et que doit-on changer à la déclaration de la variable dans `main` pour pouvoir l'appeler ?
