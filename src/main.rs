// =====================================================================
// RÉVISIONS — strictement ce qui a déjà été vu
//
// Périmètre : The Book ch. 1, 3, 2, 4 et 5.1 — RBE ch. 1 (formatage).
// Rien au-delà : ni blocs `impl`, ni itérateurs, ni `derive`.
//
// Règle du jeu : tout le code est à écrire par toi.
// Pour les exercices marqués [PRÉDIS], écris ta réponse en commentaire
// AVANT de compiler. L'intérêt est dans l'écart entre les deux.
//
// Vérifier sans exécuter :  cargo check
// Exécuter :                cargo run
// =====================================================================
//

fn main() {
    // -----------------------------------------------------------------
    // EX. 1 — `if` comme expression                        [ch. 3.5]
    // -----------------------------------------------------------------
    // Écris `parite(n: i32) -> &'static str` qui renvoie "pair" ou
    // "impair".
    //
    println!("»»» EXERCICE 1 »»»");
    fn parite(n: i32) -> &'static str {
        if n % 2 == 0 { "pair" } else { "impair" }
    }

    println!("7 est un nombre {}", parite(7));
    println!("10 est un nombre {}", parite(10));
    //
    // Contraintes :
    //   - AUCUN `return`, AUCUN `;` sur la dernière expression
    //   - un seul `if`, utilisé comme EXPRESSION
    //
    // Appelle-la sur 7 et 10, affiche les deux résultats.
    //
    // Question : pourquoi `-> &'static str` fonctionne-t-il ici, alors
    //            qu'on a vu qu'une fonction ne peut pas renvoyer une
    //            reference vers une valeur qu'elle a créée ?
    // "pair" et "impair" sont des string literals stockés sur le binaire
    // 'static str indique qu'il s'agit d'une référence dont la durée de vie est la même que celle du programme
    // donc sa valeur reste valide en sortant du scope de la fonction

    // -----------------------------------------------------------------
    // EX. 2 — Shadowing vs mutabilité                      [ch. 3.1]
    // -----------------------------------------------------------------
    // Pars du littéral "  42  " (avec espaces).
    // Obtiens l'entier 42, puis son double.

    println!("»»» EXERCICE 2 »»»");

    let value_without_mut = "  42  ";
    let value_without_mut = value_without_mut.trim();
    let value_without_mut: u8 = value_without_mut
        .parse()
        .expect("Value should be a valid number");
    let value_without_mut = value_without_mut * 2;

    println!("Value without mut is {value_without_mut}");

    let mut value_with_mut = "  42  ";
    value_with_mut = value_with_mut.trim();

    let mut parsed_value_with_mut: u8 = value_with_mut
        .parse()
        .expect("Value should be a valid number");
    parsed_value_with_mut = parsed_value_with_mut * 2;

    println!("Value with mut is {parsed_value_with_mut}");
    // Fais-le DEUX fois :
    //   a) par shadowing — le même nom réutilisé, jamais de `mut`
    //   b) avec des noms distincts et `let mut` pour le doublement
    //
    // [PRÉDIS] Une seule des deux versions autorise la variable à
    // changer de TYPE en cours de route. Laquelle, et pourquoi ?
    // on peut seulement le TYPE en cours via le shadowing

    // -----------------------------------------------------------------
    // EX. 3 — Formatage                                    [RBE ch. 1]
    // -----------------------------------------------------------------
    // Affiche exactement ceci, avec un `println!` par ligne :
    //
    //   Article      Qte
    //   ----------------
    //   Pain         ..2
    //   Cafe         .12
    //   Chocolat     ..1
    // 
    println!("»»» EXERCICE 3 »»»");

    const NAME_PADDING: usize = 12;
    const QUANTITY_PADDING: usize = 3;

    fn print_line(name: &'static str, quantity: i8) {
        println!("{0:<NAME_PADDING$}{1:.>QUANTITY_PADDING$}", name, quantity);
    }

    println!("{0:<NAME_PADDING$}{1:>QUANTITY_PADDING$}", "Article", "Qte");
    println!("{0:-<1$}", "", { NAME_PADDING + QUANTITY_PADDING });
    print_line("Pain", 2);
    print_line("Cafe", 12);
    print_line("Chocolat", 1);

    //
    // Contraintes :
    //   - nom cadré à GAUCHE sur 12 colonnes
    //   - quantité cadrée à DROITE sur 3, remplie avec des points
    //   - la largeur 12 doit venir d'une VARIABLE, pas d'un littéral
    //     (donc `width$`, et la variable doit être du bon type)
    //
    // [PRÉDIS] Si la quantité vaut -2, qu'affiche `{:0>3}` ?
    //          Et `{:03}` ? Les deux diffèrent — vérifie.
    // {:0>3} afficge 0-2
    // {:03} affiche -02

    // -----------------------------------------------------------------
    // EX. 4 — Move                                         [ch. 4.1]
    // -----------------------------------------------------------------
    // Crée une `String`, affecte-la à une seconde variable, puis essaie
    // d'afficher la PREMIÈRE.

    println!("»»» EXERCICE 4 »»»");
    let super_string = String::from("Wow quelle belle string");
    let _other_string = &super_string;
    println!("{super_string}");

    let super_number = 42;
    let _other_number = super_number;
    println!("{super_number}");

    //   a) Note le code d'erreur exact : E0382
    //   b) Fais compiler SANS supprimer l'affichage de la première
    //   c) Recommence avec un `i32` : aucune erreur.
    //      Explique en une phrase pourquoi, en employant le mot
    //      « invalidation ».
    // Dans le cas de String, c'est un type composé complexe (collection), copier sa valeur reviendrait à copier la référence au tampon et donc avoir deux propriétaires pour la même valeur ce qui est interdit. La propriété de super_string est donnée à _other_string ce qui provoque son invalidation
    // le type super_number, ici par défait i32 est un type scalaire simple qui implémente Copy
    // Donc pendant le move, la valeur est automatiquement dupliquée et super_number reste valide

    // -----------------------------------------------------------------
    // EX. 5 — Copy ou pas Copy                             [ch. 4.1]
    // -----------------------------------------------------------------
    // Pour chaque valeur, [PRÉDIS] si la source reste utilisable après
    // `let b = a;`, puis vérifie une par une :
    //
    //   1. let a = 5i64; // a reste valide
    //   2. let a = [1, 2, 3];               // tableau de i32 // a reste valide
    //   3. let a = (1, 2.5);                // tuple // a reste valide
    //   4. let a = (1, String::from("x"));  // tuple mixte // a devient invalide
    //   5. let a = "littéral";              // &str  // a reste valide
    //   6. let a = String::from("x");       // a devient invalide
    //
    // Formule en UNE phrase la règle générale que tu en tires.
    // la propriété est transférée pour les types complexes qui n'implémentent pas Copy
    // un type est Copy quand la duplication de ses octets donne une valeur indépendante et valide

    // -----------------------------------------------------------------
    // EX. 6 — References                                   [ch. 4.2]
    // -----------------------------------------------------------------
    // Écris `taille(s: &String) -> usize` qui renvoie la longueur.
    //
    // Contrainte : après l'appel, la chaîne d'origine doit rester
    // utilisable — affiche-la APRÈS, pour le prouver.
    //
    //   a) Change ensuite la signature pour prendre `&str`.
    //      Qu'est-ce que la fonction accepte en plus, du coup ?
    //   b) Le paramètre `s` lui-même (pas la chaîne pointée) occupe
    //      combien d'octets dans les deux cas ? Pourquoi la différence ?

    // -----------------------------------------------------------------
    // EX. 7 — Emprunt mutable et règle XOR                 [ch. 4.2]
    // -----------------------------------------------------------------
    // Écris `ajouter_point(s: &mut String)` qui ajoute " ." à la fin.
    //
    //   a) Fais-la fonctionner. DEUX choses doivent être rendues
    //      mutables — lesquelles ? (l'une provoque E0596)
    //   b) Provoque volontairement E0499 : deux emprunts mutables
    //      simultanés. Note le message, puis corrige.
    //   c) Provoque volontairement E0502 : un emprunt immuable encore
    //      vivant pendant un emprunt mutable. Note le message,
    //      puis corrige.
    //
    // [PRÉDIS] avant (c) : si tu crées l'emprunt immuable mais ne
    // l'utilises JAMAIS après l'emprunt mutable, est-ce que ça compile ?

    // -----------------------------------------------------------------
    // EX. 8 — Slices                                       [ch. 4.3]
    // -----------------------------------------------------------------
    // Écris `dernier_mot(s: &str) -> &str` — le miroir du `first_word`
    // du Book. Même technique : `as_bytes()`, un `for`, le byte
    // literal b' '.
    //
    // Contraintes :
    //   - renvoie une SLICE : aucune allocation, aucun `String::from`
    //   - si la chaîne n'a pas d'espace, renvoie la chaîne entière
    //
    // Puis : garde le résultat dans une variable, et essaie de vider la
    // chaîne source avec `.clear()` AVANT de l'afficher.
    //   a) Note le code d'erreur
    //   b) Quel bug classique de JS cette erreur rend-elle impossible ?

    // -----------------------------------------------------------------
    // EX. 9 — UTF-8 et frontières de caractères            [ch. 4.3]
    // -----------------------------------------------------------------
    // Soit la chaîne "café ☕".
    //
    //   a) [PRÉDIS] la valeur de `.len()`, puis vérifie.
    //      D'où vient l'écart avec le nombre de caractères visibles ?
    //   b) Découpe `&s[0..4]`, puis `&s[0..3]`. L'un des deux panique :
    //      [PRÉDIS] lequel, et note le message exact.
    //   c) Écris une boucle `while` qui part de l'indice 4 et DESCEND
    //      jusqu'à trouver une frontière de caractère valide
    //      (`is_char_boundary`). Affiche l'indice trouvé et la slice
    //      correspondante.
    //
    // Question : pourquoi cette recherche est-elle garantie de
    //            s'arrêter, et pourquoi est-elle si rapide ?

    // -----------------------------------------------------------------
    // EX. 10 — Définir une struct                          [ch. 5.1]
    // -----------------------------------------------------------------
    // Définis `Livre { titre: String, pages: u32, emprunte: bool }`.
    //
    //   a) Crée une instance, affiche ses trois champs
    //   b) Écris `nouveau(titre: String, pages: u32) -> Livre` qui met
    //      `emprunte` à false. Utilise le FIELD INIT SHORTHAND.
    //   c) Rends un livre empruntable : change `emprunte` à true après
    //      coup. Qu'as-tu dû ajouter, et où ?
    //
    // Question : pourquoi `titre: String` et non `titre: &str` ?
    //            Essaie `&str` et lis l'erreur — elle nomme une notion
    //            que tu n'as pas encore vue. Laquelle ?

    // -----------------------------------------------------------------
    // EX. 11 — Tuple structs et typage nominal             [ch. 5.1]
    // -----------------------------------------------------------------
    // Définis `Metres(f64)` et `Pieds(f64)`.
    // Écris `en_pieds(m: Metres) -> Pieds` (1 m = 3.28084 pieds).
    //
    //   a) Essaie de lui passer un `Pieds`. Code d'erreur ?
    //   b) Essaie de lui passer un `f64` nu. Même erreur ?
    //   c) En TypeScript, `type Metres = number` et `type Pieds = number`
    //      auraient-ils bloqué ces deux appels ? Pourquoi ?
    //   d) Déstructure le résultat pour en extraire le f64.
    //      Deux façons : par `.0`, et par motif.

    // -----------------------------------------------------------------
    // EX. 12 — Partial move                                [ch. 5.1]
    // -----------------------------------------------------------------
    // Reprends `Livre` (deux champs `Copy`, un champ `String`).
    // Ajoute-lui un second champ `String` : `auteur`.
    //
    // Crée `a`, puis `b` avec la syntaxe `..a`, en fournissant
    // explicitement UN SEUL des deux champs `String`.
    //
    // [PRÉDIS] pour chacun des quatre champs : lequel reste lisible
    // depuis `a` après cette ligne ? Puis vérifie CHAMP PAR CHAMP.
    //
    // Ensuite : fais en sorte que `a` reste ENTIÈREMENT utilisable
    // après la création de `b`. Quel est le coût de ta solution ?

    // -----------------------------------------------------------------
    // EX. 13 — Result, match, expect                       [ch. 2]
    // -----------------------------------------------------------------
    // Écris `moitie(saisie: &str) -> i32` qui parse la chaîne et renvoie
    // la moitié de l'entier.
    //
    // Écris-la en TROIS versions. Pour chacune, dis ce qui se passe sur
    // l'entrée "abc" :
    //   a) avec `expect`
    //   b) avec `match`, en renvoyant 0 sur la branche `Err`
    //   c) avec `unwrap_or`
    //
    // Questions :
    //   - laquelle est l'équivalent honnête du `?? 0` de JS ?
    //   - laquelle est un piège pour qui vient de JS, et pourquoi ?
    //   - dans la version (b), quel est le TYPE de la valeur que le
    //     `match` produit ?
}
