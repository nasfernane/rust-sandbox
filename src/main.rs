// =====================================================================
// RÉVISIONS — chapitres 5 et 6
//
// Périmètre : structs + `impl` (5.2, 5.3), enums avec données, `Option`,
// `match`, `if let`, `if let ... else`, `let ... else`.
// Rien au-delà : ni itérateurs, ni génériques écrits à la main, ni `Vec`.
//
// Tout le code est à écrire par toi.
// [PRÉDIS] = écris ta réponse en commentaire AVANT de compiler.
//
//   cargo check   |   cargo clippy   |   cargo run
// =====================================================================

fn main() {
    // -----------------------------------------------------------------
    // EX. 1 — Méthode, fonction associée, `Self`            [ch. 5.3]
    // -----------------------------------------------------------------
    // Définis `Temperature { celsius: f64 }`.
    //
    // #[derive(Debug)]
    // struct Temperature {
    //     celsius: f64,
    // }

    // impl Temperature {
    //     fn en_fahrenheit(&self) -> f64 {
    //         self.celsius * 1.8 + 32.0
    //     }

    //     fn depuis_fahrenheit(f: f64) -> Self {
    //         Self {
    //             celsius: (f - 32.0) / 1.8,
    //         }
    //     }

    //     fn est_gelee(&self) -> bool {
    //         self.celsius <= 0.0
    //     }
    // }

    // let temp = Temperature { celsius: 32.0 };
    // let temp_f = temp.en_fahrenheit();
    // println!(
    //     "Une température de {}°C correspond à {}F",
    //     temp.celsius, temp_f
    // );

    // let temp_from_f = Temperature::depuis_fahrenheit(89.6);
    // println!("Température depuis 89.6F donne {}", temp_from_f.celsius);

    // if temp_from_f.est_gelee() {
    //     println!("ça caille un peu");
    // } else {
    //     println!("Il fait bon en fait");
    // }

    //   a) `en_fahrenheit(&self) -> f64`          (°F = °C × 1.8 + 32)
    //   b) `depuis_fahrenheit(f: f64) -> Self`    fonction associée
    //   c) `est_gelee(&self) -> bool`
    //
    // Contrainte : en (b), écris `Self` et non `Temperature`.
    //
    // Questions :
    //   - qu'est-ce qui distingue (a) de (b) dans la SIGNATURE ?
    //   (a) porte la référence de l'instance (&self) en paramètre
    //   Ce n'est pas le cas de (b) qui a juste un paramètre de type f64 et qui sert de constructeur
    //   pour retourner une nouvelle instance
    //   - `Self` avec une majuscule et `self` en minuscule : deux choses
    //     différentes. Lesquelles ?
    //   Self est le type de l'instance, en l'occurence ça correspond à Temperature
    //   self est la valeur de l'instance

    // -----------------------------------------------------------------
    // EX. 2 — Les trois receveurs                           [ch. 5.3]
    // -----------------------------------------------------------------
    // Ajoute à `Temperature` :
    //
    //   a) `rechauffer(&mut self, degres: f64)`   modifie sur place
    //   b) `consommer(self) -> f64`               rend le celsius et
    //                                             détruit l'instance
    //
    // impl Temperature {
    //     fn rechauffer(&mut self, degres: f64) {
    //         self.celsius += degres;
    //     }

    //     fn consommer(self) -> f64 {
    //         self.celsius
    //     }
    // }

    // let mut temp2 = Temperature { celsius: 24.2 };
    // println!("Température initiale {}", temp2.celsius);

    // temp2.rechauffer(2.5);
    // println!("Température après réchauffement {}", temp2.celsius);

    // let celsius = temp2.consommer();
    // println!("La temp consommée: {}", celsius);

    // println!("{}", temp2.celsius); // on ne peut plus utiliser l'instance

    // [PRÉDIS] pour chacune, avant de compiler :
    //   - que faut-il déclarer `mut` pour appeler (a) ?
    //   Il faut que l'instance de Temperature (ici mut_temp) soit mutable
    //   - après avoir appelé (b), peut-on encore utiliser l'instance ?
    //    non on ne peut plus, l'instance est consommée car le paramètre n'est pas une référence
    //    (b) récupère l'ownership de l'instance mais ne la rend pas, (b) renvoie une nouvelle valeur qui correspond
    //    à la valeur initiale du champ celsius de l'instance
    //   - et après avoir appelé (a) ?
    //    Oui on peut toujours, c'est une référence qui est utilisée en paramètre
    //
    // Vérifie les trois. Note le code d'erreur quand ça refuse.
    // code erreur E0382 après (b)

    // -----------------------------------------------------------------
    // EX. 3 — `Debug` : trois façons d'afficher              [ch. 5.2]
    // -----------------------------------------------------------------
    // Affiche une `Temperature` de trois manières :
    //   a) `{:?}`     b) `{:#?}`     c) `dbg!`

    // let temp3 = Temperature {
    //     celsius: 25.7897770078,
    // };

    // println!("Première temp {:?}", temp3);
    // println!("Deuxième temp {:#?}", temp3);
    // // let temp3 = dbg!(temp3);
    // dbg!(temp3);
    // Questions :
    //   - qu'est-ce qui change entre (a) et (b) ?
    //  le # de (b) est le drapeau alternatif. Il ajoute un formatage (une ligne par champ, indentation et virgule finale sur le dernier champ)
    //   - `dbg!` affiche deux choses que les autres n'affichent pas.
    //     Lesquelles ?
    //    l'emplacement (fichier, ligne et colonne) et le texte de l'expression
    //   - [PRÉDIS] `dbg!(ma_temp)` — l'instance est-elle encore
    //     utilisable après ? (indice : regarde la signature dans la doc,
    //     `rustup doc --std` puis cherche `dbg`)
    //     dbg! récupère la propriété, modifie puis retourne la valeur.
    //     Ici temp3 n'est plus utilisable car je n'ai pas utilisé une référence
    //     j'aurais pu faire aussi temp3 = dbg!(temp3) pour que ça fonctionne toujours
    //   - sur quel FLUX `dbg!` écrit-il ? Et `println!` ?
    //     (teste : `cargo run > sortie.txt` puis regarde le fichier)
    //     println! écrit sur le flux stdout
    //     dbg! écrit sur le flux stderr

    // -----------------------------------------------------------------
    // EX. 4 — Un enum qui porte des données                  [ch. 6.1]
    // -----------------------------------------------------------------
    // Modélise le résultat d'une commande de café :
    //
    //   Boisson::Espresso                       rien
    //   Boisson::Allonge { eau_ml: u32 }        champs nommés
    //   Boisson::Latte(u32, bool)               volume, sucre ?
    //
    // Écris une méthode `prix_centimes(&self) -> u32` dans un bloc
    // `impl Boisson`, avec un tarif de ton choix, mais où le prix du
    // Latte DÉPEND de ses données.
    //
    //
    enum Boisson {
        Expresso,
        Allonge { eau_ml: u32 },
        Latte(u32, bool),
        The,
    }

    // impl Boisson {
    //     fn prix_centimes(&self) -> u32 {
    //         match self {
    //             Boisson::Expresso => 300,
    //             Boisson::Allonge { eau_ml } => eau_ml * 3,
    //             Boisson::Latte(vol, sucre) => vol * 3 + 50 + if *sucre { 50 } else { 0 },
    //             Boisson::The => 350,
    //             // _ => 350,
    //         }
    //     }

    //     fn mention_sucre(&self) -> Option<String> {
    //         let Boisson::Latte(_, sucre) = self else {
    //             return None;
    //         };

    //         Some(format!("{} sucre", if *sucre { "Avec" } else { "Sans" }))
    //     }
    // }

    // let expresso = Boisson::Expresso;
    // let allonge = Boisson::Allonge { eau_ml: 150 };
    // let latte = Boisson::Latte(200, true);

    // println!("Prix d'un expresso: {}", expresso.prix_centimes());
    // println!("Prix d'un allonge: {}", allonge.prix_centimes());

    // let latte_str_base = format!("Prix d'un latte: {}", latte.prix_centimes());
    // let latte_str = match latte.mention_sucre() {
    //     None => latte_str_base,
    //     Some(val) => format!("{latte_str_base} - {val}"),
    // };

    // println!("{}", latte_str);
    // Question : pourquoi un enum plutôt que trois structs distinctes ?
    //            Écris la réponse en une phrase — c'est la raison d'être
    //            des enums à données.
    // l'enum permet de passer en argument de fonction un type qui peut prendre plusieurs formes (variantes constructeurs) sous forme de liste fermée et vérifiable

    // -----------------------------------------------------------------
    // EX. 5 — Exhaustivité                                   [ch. 6.2]
    // -----------------------------------------------------------------
    // Ajoute une quatrième variante `Boisson::The` à ton enum, SANS
    // toucher à `prix_centimes`.
    //
    //   a) [PRÉDIS] que dit le compilateur ? Code d'erreur : E0___
    //    j'ai pas le code d'erreur en tête mais il va me dire que tous les valeurs possibles ne sont pas couvertes par le match
    //    après vérification, E0004. C'est important de connaitre les codes d'erreur par coeur ?
    //   b) Corrige en traitant la variante explicitement
    //   c) Corrige plutôt avec `_`. Que perds-tu ?
    //    si je met _ ça fonctionne mais ça devient un cas générique qui traite tous les variantes non traitées
    //    on perd la valeur éventuelle de la variante si il y'en a une, et tous les cas sont traités de la même façon
    //
    //
    // Question : si demain tu ajoutes `Boisson::Chocolat`, laquelle des
    //            deux corrections te PRÉVIENT, et laquelle te laisse
    //            partir avec un prix faux silencieusement ?
    // _ me laisse partir avec un prix faux silencieusement (sauf si je met un panic ou log d'erreur dans l'expression qui vient après)
    // si je met rien, ça ne compile pas et je suis obligé d'implémenter le bras pour Chocolat

    // -----------------------------------------------------------------
    // EX. 6 — `match` sur `Option`                           [ch. 6.2]
    // -----------------------------------------------------------------
    // Écris `etiquette(note: Option<u32>) -> String` :
    //
    //   Some(20)  -> "parfait"
    //   Some(n)   -> "note : {n}"
    //   None      -> "pas encore notée"
    //
    // fn etiquette(note: Option<u32>) -> String {
    //     match note {
    //         Some(20) => String::from("parfait"),
    //         Some(n) => format!("note: {n}"),
    //         // None => "Pas encore notée",
    //         None => String::from("pas encore notée"),
    //     }
    // }

    // let etiquette_note1 = etiquette(Some(20));
    // let etiquette_note2 = etiquette(Some(9));
    // let etiquette_note3 = etiquette(None);

    // println!("{etiquette_note1}");
    // println!("{etiquette_note2}");
    // println!("{etiquette_note3}");
    // Contraintes :
    //   - un seul `match`, qui est la DERNIÈRE EXPRESSION de la fonction
    //   - aucun `return`, aucune variable intermédiaire
    //
    // Question : les trois bras produisent-ils bien le même type ?
    //            Que se passe-t-il si l'un renvoie un `&str` et les
    //            autres une `String` ? Essaie, et lis l'erreur.
    // Oui les trois bras produisent le même type, sinon j'ai une erreur
    // E0308 expected String, found &'static str

    // -----------------------------------------------------------------
    // EX. 7 — `if let` : ce que tu gagnes, ce que tu perds   [ch. 6.3]
    // -----------------------------------------------------------------
    let super_option: Option<u32> = Some(17);
    // let super_option: Option<u32> = None;

    // match super_option {
    //     Some(val) => println!("Quel joli nombre: {}", val),
    //     _ => (),
    // }

    if let Some(val) = super_option {
        println!("Quel joli nombre: {}", val);
    } else {
        println!("y'a pas de nombre c'est triste");
    }

    //   a) Écris un `match` sur une `Option<u32>` qui n'agit QUE sur
    //      `Some`, avec un bras `_ => ()`
    //   b) Réécris-le en `if let`. Lance `cargo clippy` sur la version
    //      (a) : que te dit-il ?
    //      warning: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    //   c) Ajoute un `else` à ton `if let`
    //
    // Question : en passant de (a) à (b), quelle garantie du compilateur
    //            as-tu abandonnée ? Dans quel cas ça te coûtera cher ?
    // Je ne sais pas trop j'ai pas l'impression que le if let simple me donne réellement une garantie.
    // Ici dans mon exemple Some(val) rattrape toutes les valeurs et else récupère None donc je vois pas en quoi c'est plus dangereux que le if let simple

    // -----------------------------------------------------------------
    // EX. 8 — `let ... else`                                 [ch. 6.3]
    // -----------------------------------------------------------------
    // Écris `volume_latte(b: Boisson) -> Option<u32>` qui renvoie le
    // volume UNIQUEMENT si la boisson est un Latte.
    //
    impl Boisson {
        // version if let ... else
        // fn volume_latte(b: Boisson) -> Option<u32> {
        //     if let Boisson::Latte(volume, _) = b {
        //         Some(volume)
        //     } else {
        //         None
        //     }
        // }

        // version let ... else
        fn volume_latte(&self) -> Option<u32> {
            let Boisson::Latte(volume, _) = self else {
                // 0 // expected !, found i32 E0308
                return None;
            };

            Some(*volume)
        }

        fn mention_volume(&self) {
            let volume = Self::volume_latte(self);

            match volume {
                Some(vol) => println!("Volume de ma super boisson: {vol}"),
                None => println!("Volume non disponible"),
            }
        }
    }

    let cafe_latte = Boisson::Latte(200, false);
    let cafe_allonge = Boisson::Allonge { eau_ml: 150 };

    cafe_latte.mention_volume();
    cafe_allonge.mention_volume();

    //   a) Version avec `if let ... else`
    //   b) Version avec `let ... else`
    //   c) Dans la version (b), remplace le contenu du bloc `else` par
    //      une simple valeur (ex. `0`) au lieu de `return None`.
    //      [PRÉDIS] est-ce que ça compile ? Vérifie et note l'erreur.
    //     Ca ne fonctionnera pas il faudrait que je renvoie Some(0)
    //     le type u32 ne correspond pas à la signature de la fonction qui doit retourner Option<u32>
    //     confirmé: expected Option<u32>, found i32
    //
    // Question : quelle contrainte pèse sur le bloc `else` d'un
    //            `let else`, et POURQUOI est-elle logiquement nécessaire ?
    //            (indice : que doit-il être vrai de la variable liée
    //            pour la suite du code ?)
    // le bloc else doit forcément être divergent et donc soit retourner un break, un continue, un return ou un panic
    // c'est nécessaire pour s'assurer que la valeur récupérée par le if est valide quand on poursuit l'exécution de la fonction

    // -----------------------------------------------------------------
    // EX. 9 — ⚠️ Déstructurer, c'est déplacer                [ch. 4 + 6]
    // -----------------------------------------------------------------
    // Soit une `Boisson::Latte` stockée dans une variable.
    //
    //   a) Fais un `if let Boisson::Latte(volume, sucre) = boisson`
    //   b) APRÈS le `if let`, essaie de réutiliser `boisson`.
    //      [PRÉDIS] ça compile ? Pourquoi ?
    //
    //   c) Recommence avec `Boisson::Allonge { eau_ml }` — même question.
    //
    //   d) Refais (a) avec une variante qui contient une `String`.
    //      Cette fois ça refuse. Code d'erreur : E0___
    //      `rustc` propose DEUX corrections dans son `help`. Trouve-les
    //      et dis ce qui les distingue.
    //
    // Question : pourquoi (a) et (d) ne se comportent-ils pas pareil,
    //            alors que le motif a exactement la même forme ?

    // -----------------------------------------------------------------
    // EX. 10 — Conception : enum ou struct ?                 [ch. 5 + 6]
    // -----------------------------------------------------------------
    // Pour chacun de ces trois besoins, dis si tu modéliserais avec une
    // struct, un enum, ou les deux imbriqués — et justifie en une ligne :
    //
    //   1. un point dans un plan (x, y)
    //   2. l'état d'une commande : en attente / expédiée avec un numéro
    //      de suivi / annulée avec un motif
    //   3. un utilisateur avec un nom, un email, et un statut qui est
    //      soit actif, soit suspendu jusqu'à une date
    //
    // Implémente le n°2, et écris une méthode `resume(&self) -> String`
    // qui produit une phrase différente selon l'état.
}
