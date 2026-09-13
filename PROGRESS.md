# PROGRESS — parcours Rust de Nassim

> Journal des notions abordées. Mis à jour au fil des sessions par Claude.
> Sert de base aux quizz de révision.

---

## État du parcours

**Ressource principale** : [The Book](https://doc.rust-lang.org/book/)

| Chapitre | Titre | État |
|---|---|---|
| 1 | Getting Started | ✅ fait |
| 3 | Common Programming Concepts | ✅ fait |
| 2 | Programming a Guessing Game | 🔄 **en cours** |
| 4 | Understanding Ownership | ⏳ à venir — **le vrai mur** |

**Pourquoi ce détour 1 → 3 → 2 ?** Le Book lui-même suggère cet ordre à ceux qui
préfèrent comprendre les fondations avant de coder un projet. Le chapitre 2 est
un projet complet qui utilise des notions non encore expliquées ; l'avoir abordé
après le 3 le rend beaucoup plus lisible.

**Fichiers du dépôt** : `src/`, un fichier par notion, numérotés dans l'ordre
d'étude. `src/main.rs` = le jeu de devinettes (chapitre 2, en cours).

---

## Chapitre 1 — Getting Started

*Fichier : `src/1-hello-world.rs`*

- `fn main()` : point d'entrée du programme
- Commentaires : `//` (ligne) et `/* */` (bloc, insérable au milieu d'une expression)
- `println!` et son interpolation : `{}` avec argument positionnel, `{x}` avec
  capture directe du nom de variable

### Hors tutoriel (creusé à part)

- Rôle de Cargo comme package manager **et** build system
- Contenu d'un binaire : format PE, sections, désassemblage, fichier `.pdb`
- **Fonctions vs macros** : le `!` marque une macro, qui génère du code à la
  compilation. C'est ce qui permet à `println!` d'accepter un nombre variable
  d'arguments et de vérifier le format à la compilation
- Rôle sémantique du `;` et type unité `()`

---

## Chapitre 3 — Common Programming Concepts

### 3.1 Variables et mutabilité

*Fichier : `src/2-variables-and-mutability.rs`*

- `let` est **immuable par défaut** → il faut `let mut` pour muter
- `const` : toujours immuable, **annotation de type obligatoire**, évaluée à la
  compilation, nommée en `SCREAMING_SNAKE_CASE`
- **Shadowing** : redéclarer un `let` avec le même nom crée une *nouvelle*
  variable — donc le type peut changer
- Le shadowing est **limité à sa portée** : en sortant du bloc, la variable
  précédente réapparaît
- ⚠️ **Distinction clé** : `let mut` permet de changer la *valeur*, pas le
  *type*. Seul le shadowing permet de changer de type.

### 3.2 Types de données

*Fichier : `src/3-data-types.rs`*

**Inférence et annotation**
- L'annotation est requise seulement quand le type est ambigu (typiquement
  `parse()`, qui peut produire plusieurs types) → `error[E0284]`
- Préfixe `_` sur un nom de variable : signale une non-utilisation
  intentionnelle, supprime le warning

**Types scalaires** (une seule valeur)

| Famille | Détail |
|---|---|
| Entiers | `i8`…`i128` / `u8`…`u128`, `isize`/`usize` (taille = archi machine). Défaut : `i32` |
| Flottants | `f32`, `f64`. Défaut : `f64` (précision supérieure, coût comparable) |
| Booléen | `bool` — `true` / `false` |
| Caractère | `char`, **4 octets**, valeur scalaire Unicode, guillemets **simples** |

⚠️ `'z'` est un `char`, `"z"` est un `&str`. Les guillemets décident du type.

**Types composés** (*compound types*, fr. « types composés »)
- **Tuple** : longueur fixe, **types hétérogènes** — `(i32, f64, u8)`
  - déstructuration : `let (x, y, z) = tup;`
  - accès par position : `tup.0`
- **Tableau** (*array*) : longueur fixe, **type homogène**
  - annotation : `[type; longueur]`
  - initialisation répétée : `[3; 5]` → `[3, 3, 3, 3, 3]`
  - accès par index comme en JS : `a[0]`

### 3.2b Index hors limites

*Fichier : `src/3a-array-index-out-of-bounds.rs`*

- Rust **vérifie les bornes à l'exécution** et déclenche un `panic!`
- Contraste JS : `arr[99]` renvoie `undefined` ; contraste C : lecture mémoire
  arbitraire. Rust refuse les deux.
- Premier contact avec `io::stdin()`, `read_line`, `.trim()`, `.parse()`, `.expect()`

### 3.3 Fonctions

*Fichier : `src/4-functions.rs`*

- `fn nom(param: Type, ...) -> TypeRetour { }` — **annotation obligatoire** sur
  chaque paramètre et sur le retour
- Convention de nommage : `snake_case`
- **Statements vs expressions** — le point central du chapitre :
  - un *statement* ne produit pas de valeur → `let x = let y = 6;` est invalide
  - une *expression* produit une valeur : `5 + 6`, un appel de fonction, un appel
    de macro, **un bloc `{}`**
- La **dernière expression sans `;`** est la valeur de retour du bloc
- ⚠️ Ajouter un `;` transforme le retour en `()` → `expected i32, found ()`
- `return` réservé aux **retours anticipés** (clippy : `needless_return`, warn
  par défaut)

### 3.4 Contrôle de flux

*Fichier : `src/5-control-flow.rs`*

**Conditions**
- La condition **doit être un `bool`**, strictement. Aucune *truthiness*.
- `if` est une **expression** → sert de ternaire : `let n = if c { 1 } else { 2 };`
- Les branches doivent avoir des **types homogènes** → sinon `error[E0308]`

**Boucles**
- `loop` : boucle infinie ; `break valeur` en fait une **expression qui renvoie
  une valeur**
- ⚠️ `break` sort de la boucle, `return` sort **toujours de la fonction**
- **Étiquettes de boucle** : `'nom: loop { ... break 'nom; }` pour cibler une
  boucle externe depuis une imbriquée
- `while` : boucle conditionnelle
- `for x in collection` : la forme idiomatique, sans index ni risque de
  dépassement de bornes
- **Ranges** : `(1..4)`, avec `.rev()` pour inverser

---

## Chapitre 2 — Jeu de devinettes (en cours)

*Fichier : `src/main.rs`*

### Acquis

- **Premier projet Cargo** → fin de la compilation manuelle au `rustc`
- `use std::io;` — import du module d'entrées/sorties
- `io::stdin().read_line(&mut guess).expect("...")`
- Chaînage de méthodes sur plusieurs lignes (style rustfmt)
- Ajout d'une **dépendance externe** dans `Cargo.toml` (`rand`)
- Écriture d'une version personnelle du jeu avant de lire la suite du tuto

### Notions creusées en détail

**`let mut guess = String::new();` — décorticage complet**

- `::` = séparateur de **chemin** (namespace), à distinguer du `.` qui appelle
  une méthode **sur une valeur**
- `String::new()` est une **fonction associée** (*associated function*) :
  définie dans le `impl` du type, sans paramètre `self`. Équivalent conceptuel
  de la méthode statique TS.
- ⚠️ **`new` n'est pas un mot-clé** en Rust (contrairement au `new` de JS) —
  c'est une simple convention de nommage. `String::tintin()` ne compile pas
  parce que la fonction n'existe pas, pas parce que le nom serait interdit.
- Autres constructeurs : `String::from(x)`, `String::with_capacity(n)`,
  `String::from_utf8(...)` — une douzaine au total
- Le trait `Default` et sa fonction `default()` sont un mécanisme **distinct**
  de `new`, bien que souvent équivalents en pratique

**`String` vs `&str`** (vérifié expérimentalement)

| | `String` | `&str` |
|---|---|---|
| Structure | `[ptr \| len \| capacity]` — 24 o | `[ptr \| len]` — 16 o |
| Rapport à la donnée | **possède** | **emprunte** (vue) |
| Redimensionnable | oui | non |
| Emplacement | tas | n'importe où (binaire, tas, pile) |

- `&s[0..3]` **ne copie rien** : même pointeur que la source, longueur réduite.
  Analogie JS correcte : `TypedArray` sur un `ArrayBuffer` — surtout **pas**
  `String.slice()`, qui copie.
- Convention : **`&str` en paramètre, `String` en retour**
- `read_line` exige une `String` parce qu'il fait **grandir** le tampon
- ⚠️ `read_line` **ajoute** à la chaîne (sans l'écraser) et **conserve le `\n`**
  → d'où le `.trim()`

**Allocation mémoire** (vérifié expérimentalement)
- `String::new()` et `String::from("")` sont **strictement identiques** :
  `capacity 0`, aucune allocation
- **Rust n'alloue jamais zéro octet** — le pointeur affiché (`0x1`) est un
  pointeur factice, pas une adresse réelle
- Le choix de `new()` sur `from("")` est une question d'**intention**, pas de
  performance

**`std` et son organisation**
- `std` = *standard library*, `io` = *input/output*
- Trois couches : **`core`** (sans OS ni allocation) → **`alloc`** (`String`,
  `Vec`, `Box`) → **`std`** (fichiers, réseau, threads). C'est ce découpage qui
  rend possible le `#![no_std]` pour l'embarqué.
- ⚠️ `use` **ne charge rien** — contrairement à l'`import` JS. C'est un pur
  raccourci de nommage, résolu à la compilation.
- La **prelude** importe automatiquement `String`, `Vec`, `Option`, `Result`,
  les macros d'affichage… d'où l'absence de `use` pour `println!`
- `std::io` n'est **pas limité à la console** : il définit les traits `Read` /
  `Write`, que `std::fs` (fichiers) et `std::net` (TCP/UDP) implémentent aussi.
  Seuls `stdin()`, `stdout()`, `stderr()` sont spécifiques au terminal.
- ⚠️ **Il faut importer un trait pour appeler ses méthodes** — oublier
  `use std::io::Read` produit un « méthode inexistante » trompeur

### Gestion d'erreurs — `Result` et `expect`

- `Result<T, E>` est un **enum à deux variantes** : `Ok(T)` (succès + valeur) ou
  `Err(E)` (échec + erreur). Une seule valeur de retour contenant *soit* l'un
  *soit* l'autre — l'échec est **dans** la valeur, pas dans un canal parallèle.
- ⚠️ **`expect(msg)` n'est PAS une valeur de repli.** Si `Ok(v)` → il rend `v`.
  Si `Err(e)` → il fait **paniquer** le programme (code de sortie 101). Le
  message est une étiquette collée sur le crash, pas un remplacement.
- Analogie JS correcte : `if (err) throw new Error("...")`. **Pas** un `catch`,
  **pas** un `?? valeurDefaut`.
- Le nom `expect` décrit **l'hypothèse du programmeur** : « je m'attends à ce que
  ce soit `Ok` ». Rien à voir avec l'`expect()` de Jest/Vitest.
- Convention officielle : le message doit dire **pourquoi on s'attend à un
  succès**, pas décrire l'échec. Le Book écrit `"Failed to read line"`, donc à
  contre-courant de sa propre convention (choix pédagogique assumé).

| Méthode | Lecture |
|---|---|
| `expect(msg)` | « ça devrait être `Ok`, sinon crash avec mon message » |
| `unwrap()` | idem, sans message |
| `unwrap_or(v)` | ← **la vraie** valeur de repli (≈ `??` de JS) |
| `unwrap_or_else(f)` | repli calculé paresseusement |

- `expect` est un outil de **prototype**. Planter n'est pas gérer une erreur.
- ⚠️ Le `Ok` de `read_line` contient un `usize` (nombre d'octets lus), **pas** le
  texte saisi — celui-ci part dans le `&mut`.
- Vérifié : sur une saisie vide (juste Entrée), `read_line` renvoie `Ok(1)` et la
  chaîne vaut `"\n"`, **pas** `""`. Le `.trim()` la ramène à `""`.
  Conséquence : c'est le `parse()` **suivant** qui paniquera, pas la lecture.

### Affichage — `Display` et `Debug`

- Ce n'est pas `println!` qui sait afficher un `i32` — c'est **`i32` qui sait
  s'afficher**, via le trait `Display`.
- `{}` exige `Display`. La macro se contente de **router** : elle génère un appel
  à `<TypeConcret as Display>::fmt`, résolu à la compilation.
- Un type sans `Display` → erreur de **compilation** (`E0277`), pas de plantage
  au runtime.
- `{:?}` utilise un **autre trait**, `Debug` : pour le développeur, obtenable
  gratuitement avec `#[derive(Debug)]`. `Display` est pour l'utilisateur final et
  s'écrit à la main.
- 💡 **Réflexe de débogage** : `{:?}` échappe les caractères invisibles (`"\n"`
  visible au lieu d'un saut de ligne indiscernable).
- C'est la réponse au « comment sans `typeof` » : rien à interroger à
  l'exécution, la décision est déjà prise à la compilation.

### Crates externes — `rand` et le versionnage

*Première dépendance externe ajoutée au projet.*

- ⚠️ **Le Book épingle `rand = "0.8.5"`, le projet est sur `0.10.2`.** L'API a été
  renommée entre-temps : recopier le Book tel quel **ne compile pas**.

| Book (0.8) | rand 0.10 |
|---|---|
| `rand::thread_rng()` | `rand::rng()` |
| `.gen_range(..)` | `.random_range(..)` |
| trait `Rng` porte `gen_range` | trait **`RngExt`** porte `random_range` |

- Vérifié dans le source du crate : `rand::random_range(r)` **est littéralement**
  `rng().random_range(r)`. Même générateur, même tirage.
- `rng()` rend un `ThreadRng` : local au thread, amorcé paresseusement depuis
  l'entropie de l'OS, ré-alimenté périodiquement.
- Garder le générateur explicite pour : tirages **répétés** en boucle,
  **reproductibilité** (`StdRng::seed_from_u64` pour les tests), ou le passer en
  paramètre.
- 💡 **La doc de TA version fait autorité, pas le tutoriel.** `cargo doc --open`
  ouvre la doc des dépendances réellement compilées, hors-ligne.

### ⚠️ Piège majeur : `E0599` = trait non importé

```
error[E0599]: no method named `random_range` found for struct `ThreadRng`
```

- Ne veut **presque jamais** dire « la méthode n'existe pas ». Veut dire **« le
  trait qui la porte n'est pas dans la portée »**.
- Déjà rencontré deux fois : `use std::io::Read`, `use rand::RngExt`.
- **Pourquoi cette contrainte** : puisqu'un trait peut être implémenté sur un
  type qu'on n'a pas écrit, le compilateur ne peut pas deviner lequel on veut —
  deux traits définissant la même méthode entreraient en conflit. L'import est
  la façon de dire lequel compte.
- Réflexe : devant un `E0599`, ouvrir la doc du type, repérer dans quel **trait**
  vit la méthode, importer ce trait.
- Une **fonction libre** n'appartient à aucun trait → aucun import nécessaire.
- Raccourci : `use rand::prelude::*;` importe `Rng`, `RngExt`, `SeedableRng` et
  les générateurs courants d'un coup.


---

## Transversal — concepts hors chapitre

### Paradigmes

- Rust est **multi-paradigme** : cœur impératif, forte influence fonctionnelle
  (famille ML / OCaml)
- **Fonctionnel s'oppose à impératif, pas à l'OO** — ce sont deux axes
  indépendants (*comment exprimer le calcul* vs *comment organiser le code*)
- Piliers du fonctionnel : fonctions pures, immuabilité, fonctions comme valeurs
- Ce que Rust en prend : immuabilité par défaut, types algébriques, pattern
  matching, tout est expression, itérateurs paresseux
- Ce qu'il n'en prend pas : mutation idiomatique, aucune notion de pureté,
  évaluation stricte
- **L'axe réel de Rust n'est ni OO ni fonctionnel : c'est l'ownership.** Sa
  question n'est pas « faut-il muter ? » mais « qui a le droit de muter, et
  quand ? »

### Rust et l'orienté objet

Les trois piliers OO, verdict :

| Pilier | Rust |
|---|---|
| Objets (données + comportement) | ✅ struct + bloc `impl` |
| Encapsulation | ✅ **privé par défaut**, `pub` pour exposer |
| Héritage | ❌ aucun |

- Substituts à l'héritage : **composition** pour la réutilisation, **traits**
  (`dyn Trait`) pour le polymorphisme
- ⚠️ Une struct **n'est pas un objet** : elle ne contient que des données. Les
  méthodes vivent dans un bloc `impl` séparé, potentiellement ajouté plus tard
  et ailleurs.
- Il n'y a **pas d'objets** parmi les types primitifs — les structs sont le
  chapitre 5, les enums le chapitre 6

### La notion de « type » : Rust vs JS/TS

Trois notions distinctes qui se recouvrent mal :

- **JS** : étiquette portée par la valeur, **à l'exécution** (`typeof`)
- **TS** : annotation **effacée** à la compilation, posée *par-dessus* des
  valeurs dynamiques
- **Rust** : le type **est** la définition des bits — taille, alignement,
  disposition mémoire, code machine généré

Vérifié expérimentalement : `bool` 1 o, `char` 4 o, `i32` 4 o, `i64` 8 o,
`()` **0 octet**, `(i32, bool)` **8 o** (et non 5 — bourrage d'alignement).

- ⚠️ **Une valeur Rust ne porte aucune étiquette de type à l'exécution.** Pas de
  `typeof`, et ça n'aurait aucun sens : l'information a été entièrement
  consommée à la compilation.
- Renversement contre-intuitif : Rust est bien plus typé que JS, et pourtant son
  programme en sait **moins** à l'exécution
- **TS est structurel, Rust est nominal** : deux structs aux champs identiques
  mais aux noms différents sont deux types incompatibles

### Outillage

- `clippy` : linter officiel. `needless_return` est **warn par défaut**.
- `rustup doc --book` : The Book hors-ligne
- `rustup doc --std` : doc de la bibliothèque standard hors-ligne
- **Lire les blocs `note:` et `help:` de `rustc`** — le compilateur suggère
  souvent la correction exacte (ex. la liste des constructeurs de `String`)

---

## Faux amis JS/TS — récapitulatif

| Réflexe JS | Réalité Rust |
|---|---|
| `let` = mutable | `let` est **immuable** (≈ `const` JS) |
| `if (valeur)` truthy | condition **strictement `bool`** |
| `new Foo()` mot-clé | `Foo::new()` = fonction ordinaire, `new` est une convention |
| `import` charge un module | `use` est un **pur raccourci de nommage** |
| `"abc".slice()` copie | `&s[0..3]` **ne copie pas**, c'est une vue |
| tous les nombres = `f64` | `i8`…`u128`, `f32`/`f64` — le type fixe la taille |
| types structurels (TS) | types **nominaux** |
| `null` / `undefined` | `Option<T>` (chapitre 6) |
| `try` / `catch` | `Result<T, E>` (chapitre 9) |
| `expect()` de Jest | `expect()` = assertion « ça devrait être `Ok` », sinon **crash** |
| `?? valeurDefaut` | c'est `unwrap_or(v)`, surtout pas `expect()` |
| `console.log` inspecte au runtime | `Display`/`Debug` résolus à la **compilation** |
| GC | **ownership** (chapitre 4) |

---

## À venir

- **Chapitre 2 (fin)** : `rand`, `match`, `Ordering`, la boucle de jeu
- **Chapitre 4 — Ownership** ⚠️ le seul concept sans aucun équivalent JS.
  `String`/`&str` en est déjà le premier avant-goût. Prendre le temps qu'il faut.
- Chapitre 5 : structs et blocs `impl`
- Chapitre 6 : enums, `Option`, `match`, `if let`
- Chapitre 9 : gestion d'erreurs, `Result`
- Chapitre 17 : « Rust est-il orienté objet ? », `dyn Trait`

## Questions ouvertes / défis en suspens

- Écrire une fonction `i32 -> &str` renvoyant `"pair"` / `"impair"`, en
  utilisant `if` **comme expression** (sans `return`)
- Qu'apporte une struct `Point` qu'un tuple `(i32, i32)` n'apporte pas ?
  (piste : types nominaux)
