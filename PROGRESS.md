# PROGRESS — parcours Rust de Nassim

> Journal des notions abordées. Mis à jour au fil des sessions par Claude.
> Sert de base aux quizz de révision.

---

## État du parcours

**Quatre ressources menées en parallèle** (Exercism ajouté le 2026-09-24) :

| Ressource | Mode travaillé | Sources |
|---|---|---|
| [The Book](https://doc.rust-lang.org/book/) | **compréhension** — le *pourquoi* | `src/the-book/` |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example) | **reconnaissance** — le *à quoi ça ressemble* | `src/by-example/` |
| [Rustlings](https://github.com/rust-lang/rustlings) | **production** — correction automatique | autre dépôt |
| [Exercism](https://exercism.org) | **production** — correction par un mentor humain | autre dépôt |

**Pourquoi les ordres diffèrent autant** (question du 2026-09-24) :

| Ressource | Ordonne par | Ownership arrive à |
|---|---|---|
| The Book | dépendance **conceptuelle** | ch. **4** / 21 → **19 %** |
| RBE | dépendance **syntaxique** | ch. **15** / 24 → **62 %** |
| Rustlings | gradation d'**exercices** | — |

- L'ownership est une couche **sémantique quasiment sans syntaxe propre** (`=`,
  `&`, un appel de fonction). Un tutoriel ordonné par la syntaxe peut donc le
  repousser très loin sans jamais montrer de syntaxe inconnue ; un tutoriel
  ordonné par les concepts ne le peut pas.
- RBE s'en tire d'autant mieux que ses extraits sont **minuscules et jetables** :
  le borrow checker mord rarement à cette échelle.
- The Book est en **spirale**, pas linéaire : `Result` apparaît informellement au
  ch. 2, formellement au ch. 9. Choix de **charge cognitive**, pas de rigueur.
- ⚠️ Risque de mener les quatre de front : croire qu'avoir **vu** une notion
  trois fois vaut l'avoir **comprise** une fois.

### The Book

| Chapitre | Titre | État |
|---|---|---|
| 1 | Getting Started | ✅ fait |
| 3 | Common Programming Concepts | ✅ fait |
| 2 | Programming a Guessing Game | ✅ fait |
| 4 | Understanding Ownership | ✅ fait (references, slices compris) |
| 5 | Using Structs to Structure Related Data | 🔄 **en cours** |

### Rust by Example

| Chapitre | Titre | État |
|---|---|---|
| 1 | Hello World / formatted print | 🔄 **en cours** — formatage |

### Exercism — piste Rust

| Exercice | État |
|---|---|
| Matching Brackets | ✅ fait — 15/15 cas, voir la section dédiée |

⚠️ **Chaque exercice Exercism est un projet Cargo autonome** (son `Cargo.toml`,
son `src/lib.rs`, ses `tests/`). Il ne peut donc pas vivre dans `rust-sandbox`,
qui est un package unique. Dépôt séparé, **sans `Cargo.toml` à la racine** —
sinon Cargo tente d'absorber les crates imbriquées et chacune échoue avec
*« current package believes it's in a workspace when it's not »*.

**Pourquoi ce détour 1 → 3 → 2 ?** Le Book lui-même suggère cet ordre à ceux qui
préfèrent comprendre les fondations avant de coder un projet. Le chapitre 2 est
un projet complet qui utilise des notions non encore expliquées ; l'avoir abordé
après le 3 le rend beaucoup plus lisible.

**Fichiers du dépôt** : `src/the-book/` et `src/by-example/`, un fichier par
notion, numérotés dans l'ordre d'étude. `src/main.rs` = l'atelier de la notion
en cours, archivé ensuite dans le dossier du tutoriel correspondant.

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

*Fichier : `src/the-book/3a-data-types.rs`*

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

## Chapitre 4 — Ownership (en cours)

*Fichier : `src/main.rs`*

### Les trois zones mémoire

| Zone | Contenu | Libération |
|---|---|---|
| **Binaire** (lecture seule) | littéraux `&'static str` | jamais |
| **Pile** (*stack*) | taille **connue à la compilation** | automatique, au dépilement |
| **Tas** (*heap*) | taille inconnue ou variable | quand le propriétaire sort de portée |

- ⚠️ **Le critère pile/tas est la taille connue à la compilation — PAS la
  mutabilité.** Les quatre combinaisons existent : `let x = 5` (pile immuable),
  `let mut x = 5` (pile mutable), `let s = String::from("hi")` (tas immuable),
  `let mut s = String::new()` (tas mutable). Vérifié expérimentalement.
- « taille inconnue à la compilation » ≠ « taille qui change ». La seconde
  implique la première, jamais l'inverse (ex. `let s = lire_un_fichier();`).
- **pile/tas** = décision de disposition, déduite du **type**, prise par le
  compilateur. **`mut`** = permission d'écriture que l'on s'accorde. Deux axes
  **indépendants**.
### Pourquoi le tas est plus lent (mesuré : 0,7 ns vs 15,6 ns, **23×**)

1. **Coût d'allocation** — le principal. Pile : déplacer le pointeur de pile,
   *une* instruction (souvent déjà faite à l'entrée de la fonction). Tas :
   appeler l'allocateur, qui doit **chercher** un bloc libre, tenir sa
   comptabilité, gérer la fragmentation, parfois faire un appel système, et
   rester thread-safe (verrous/atomiques). **La pile ne cherche jamais.**
2. **Localité** — la pile réutilise les mêmes kilo-octets, toujours en cache L1.
   Les objets du tas sont éparpillés → défauts de cache. Coût d'**accès**.
3. **Indirection** — lire une `String` = deux accès mémoire (le pointeur, puis
   sa cible). Une valeur de pile = un seul.

💡 Le coût est réel mais modeste (~15 ns). Le danger est de le payer des
millions de fois sans le savoir — d'où `clone()` **explicite** : Rust n'interdit
pas la copie profonde, il oblige à l'écrire.

- `String::from("hello")` occupe **trois** emplacements : le littéral dans le
  binaire, le tampon copié sur le tas, la structure à 3 champs sur la pile.
  Le texte existe en **deux exemplaires** à l'exécution.

### Les trois règles

1. Toute valeur a un **propriétaire** (*owner*)
2. Un seul propriétaire à la fois
3. Quand le propriétaire sort de portée, la valeur est **libérée**

- ⚠️ La libération est automatique pour **toute** valeur possédante. Le trait
  `Drop` n'en est pas la cause : c'est un **point d'accroche** pour exécuter du
  code personnalisé au moment de la libération.

### Move vs Copy — le même mécanisme

> ⚠️ **Un *move* est une copie superficielle suivie d'une invalidation.
> Une `Copy` est la même copie superficielle, sans invalidation.**

| | Copie des bits | Source ensuite |
|---|---|---|
| `i32` (implémente `Copy`) | oui, 4 octets | **reste valide** |
| `String` (pas `Copy`) | oui, 24 octets (`ptr`/`len`/`cap`) | **invalidée** |

- Le type ne change pas l'opération de copie, il change le **verdict sur la
  source**.
- **Pourquoi invalider** : après la copie superficielle, deux structures
  pointent vers le même tampon. Sans invalidation, les deux le libéreraient →
  ***double free***, le bug que l'ownership existe pour éliminer.
- `clone()` force la copie **profonde** (le tampon du tas aussi) — coût réel,
  à envisager consciemment.
- Erreur associée : `error[E0382]: borrow of moved value`

### ⚠️⚠️ Le critère de `Copy` : la RESSOURCE, pas la taille

> **Erreur commise deux fois (2026-09-13, puis 2026-09-20 en révision).**
> Elle revient sous des habits différents — à relire avant chaque quizz.

❌ « la propriété est transférée pour les types complexes **dont la taille
n'est pas connue à l'avance** »

**Les six tailles suivantes sont TOUTES connues à la compilation** (vérifié
avec `size_of`) — et pourtant `Copy` les sépare en deux camps :

| Type | Taille | `Copy` ? |
|---|---|---|
| `i64` | 8 o | ✅ |
| `[i32; 3]` | 12 o | ✅ |
| `(i32, f64)` | 16 o | ✅ |
| `&str` | 16 o | ✅ |
| `String` | **24 o** | ❌ `E0277` |
| `(i32, String)` | 32 o | ❌ |

- **Aucune corrélation avec la taille** : `&str` (16 o) est `Copy`, `String`
  (24 o) ne l'est pas. `[i32; 3]` (12 o) est `Copy` alors que c'est un type
  composé.
- ⚠️ **Une `String` a une taille parfaitement connue : 24 octets.** Ce qui est
  inconnu, c'est la taille du **tampon sur le tas** — mais ce tampon n'est pas
  la `String`, il est *au bout d'un pointeur*.

> ✅ **Le critère juste : un type est `Copy` si dupliquer ses octets suffit à
> produire une valeur indépendante et valide.** Autrement dit : s'il ne
> **possède aucune ressource externe** (tampon sur le tas, fichier, socket).

- Dupliquer les 24 o d'une `String` donnerait deux triplets pointant vers **le
  même tampon** → deux propriétaires → ***double free***. D'où le refus.
- Dupliquer les 12 o de `[i32; 3]` ne pose aucun problème : rien derrière.
- 💡 Ni la taille, ni la « complexité », ni le nombre de champs : **la
  possession d'une ressource externe**.

### Deux correctifs possibles pour `E0382` — ils ne disent pas la même chose

| Correctif | Sens | Coût |
|---|---|---|
| `&valeur` | « je veux seulement **regarder** » | aucune allocation |
| `valeur.clone()` | « je veux une **seconde** valeur indépendante » | allocation + recopie |

- Réflexe par défaut : **emprunter**. On n'alloue que si on a une raison.
- Ils ne sont pas interchangeables : pour modifier la copie sans toucher
  l'original, seul `clone()` convient.

### `&'static str` — pourquoi une fonction peut le renvoyer

Le chapitre 4 interdit de renvoyer une reference vers une valeur **créée dans
la fonction** (`E0106`) : son propriétaire meurt à la sortie. Un literal échappe
à la règle.

- `'static` est une **durée de vie** (*lifetime*) : « vit aussi longtemps que le
  programme ».
- Les octets d'un literal sont **dans le binaire**, présents avant que `main`
  démarre, jamais libérés. **Il n'y a aucun propriétaire à faire mourir.**
- ⚠️ Formulation à corriger : « on transfère la propriété du literal » — **non**,
  un `&'static str` est une **reference**, elle ne transfère rien et personne ne
  possède ces octets.
- L'immuabilité d'un literal est une **conséquence** (section en lecture seule),
  pas la raison.

### Pourquoi `Copy` et `Drop` sont incompatibles

`error[E0184]: Copy not allowed on types with destructors` (vérifié).

```
possède une ressource → doit la libérer → un seul propriétaire → pas de Copy
ne possède rien       → rien à libérer  → autant de copies qu'on veut → Copy
```

- `Copy` dit « dupliquer les bits donne une seconde valeur indépendante ».
  `Drop` dit « cette valeur possède une ressource à nettoyer ». Ensemble : deux
  propriétaires qui nettoient **la même** ressource, deux fois.
- 💡 C'est pour ça que `String` n'est pas `Copy` — pas un choix arbitraire :
  elle possède un tampon sur le tas, donc du code de libération.
- Types `Copy` : les scalaires (`i32`, `bool`, `char`, `f64`) et les
  tuples/tableaux qui n'en contiennent que.

### Ownership et fonctions

- Passer une valeur non-`Copy` à une fonction **transfère** l'ownership → la
  variable d'origine est invalidée
- ⚠️ **Un `return` est un *move***. Une valeur retournée n'est pas libérée en
  fin de fonction : elle n'appartient plus à la fonction. La règle « fin de
  portée → libération » ne vaut que pour les valeurs **encore possédées** à cet
  instant.

### Erreurs de raisonnement corrigées (2026-09-13)

- ❌ « la pile est pour les données immuables » → le critère est la **taille
  connue à la compilation**
- ❌ « `let x = 5; let y = x;` copie dans le tas » → tout est sur la **pile**,
  le tas n'est jamais touché
- ❌ « le défaut entier est `u32` » → c'est **`i32`** (et `f64` pour les
  flottants). Vérifié avec `type_name_of_val`. *(erreur présente aussi dans
  `src/the-book/3a-data-types.rs`)*
- ❌ « après un move, les deux variables pointent vers le même tampon » →
  la première est **invalidée**, il n'y a qu'un propriétaire
- ❌ « le mécanisme move/copy est différent » → **c'est le même**, seul le
  sort de la source change

### References et borrowing

- Une **reference** (`&s`) *emprunte* la valeur : elle donne l'accès **sans
  prendre l'ownership**. Fin de portée de la reference → **rien n'est libéré**,
  elle ne possédait rien.
- `&mut s` = **emprunt mutable** (*mutable borrow*). Règle : **soit** N emprunts
  immuables, **soit** 1 seul emprunt mutable — jamais les deux en même temps.
- Motif : cette règle élimine les *data races* **à la compilation**, sans aucun
  coût à l'exécution.
- ⚠️ Pour emprunter mutablement, la variable source doit elle-même être `mut`
  (`E0596`).

### Le borrow checker

- Ce n'est pas un programme séparé : c'est une **phase interne à `rustc`**, qui
  opère sur la **MIR**, *après* le type checking et *avant* la génération de
  code.
- Vérifié expérimentalement avec `--emit=metadata` : l'erreur d'emprunt apparaît
  alors qu'aucun code machine n'a été produit.
- 💡 C'est exactement ce qui rend **`cargo check` beaucoup plus rapide que
  `cargo build`** : il s'arrête avant le codegen tout en validant les emprunts.
- **Coût à l'exécution : zéro.** Rien de tout ça n'existe dans le binaire.

### Dangling references (*references pendantes*)

- Erreurs rencontrées : `E0597` (la valeur ne vit pas assez longtemps),
  `E0106` (*missing lifetime specifier*, sur un retour de fonction).
- ⚠️ Formulation à corriger : ce n'est **pas** « une reference vers une valeur
  qui ne lui appartient plus » — une reference **ne possède jamais rien**.
  C'est : **le propriétaire a disparu, la reference lui survit.**
- Retourner `&String` depuis une fonction qui a créé la `String` est refusé :
  le propriétaire meurt en fin de fonction. La solution est de **retourner la
  `String`** (donc l'ownership), pas une reference.

### The Slice Type

- Une slice `&s[0..5]` est une **vue** : pointeur + longueur, **aucune copie**.
- Le type d'un string literal **est** `&str` — donc un literal *est* déjà une
  slice.
- 💡 Prendre `&str` en parametre plutôt que `&String` : la fonction accepte
  alors les deux (deref coercion).
- Une slice **emprunte** : tant qu'elle vit, la source ne peut pas être mutée
  (`s.clear()` refusé) — le bug classique de l'index périmé devient impossible.

### UTF-8, byte literals et points de code

- `b' '` = **byte literal**, un `u8` (ici 32). À distinguer de `' '`, un `char`
  de **4 octets**.
- `as_bytes()` est utilisé par le tutoriel parce qu'on **ne peut pas indexer une
  `String`** par entier : `s[0]` n'a pas de sens univoque en UTF-8.
- **Point de code** = le numéro du caractère dans le catalogue Unicode. ⚠️
  `U+00E9` et `233` sont **deux notations du même nombre**, pas deux étapes.
- **Encodage** = la façon d'écrire ce nombre en octets. UTF-8 : 1 à 4 octets.
- **Auto-synchronisation d'UTF-8** : un octet de continuation commence toujours
  par `10xxxxxx`, un octet de tête jamais. On peut donc retrouver une frontière
  de caractère depuis n'importe quelle position → `is_char_boundary` est **O(1)**,
  et chercher un octet ASCII (comme l'espace) **ne peut pas** tomber au milieu
  d'un caractère multi-octets. C'est ce qui rend `as_bytes()` légitime ici.
- Découper hors frontière **panique** (`byte index N is not a char boundary`).
- ⚠️ Faux ami JS : `"👨‍👩‍👧"[0]` renvoie `"\ud83d"` — une **UTF-16 code unit**,
  une moitié de caractère. **JS corrompt silencieusement là où Rust panique.**

### Erreurs de raisonnement corrigées (suite)

- ❌ « une reference pendante pointe vers une valeur qui ne lui appartient
  plus » → une reference **ne possède jamais** ; c'est le **propriétaire** qui a
  disparu
- ❌ « un tableau contient des references » → les valeurs sont **inline**
  (`[i32; 3]` = 12 octets, adresses espacées de 4)
- ❌ « le point de code est la valeur base 10 du code hexadécimal » → une étape
  de trop : le point de code **est** le nombre
- ❌ « en JS on récupère les caractères Unicode » → des **UTF-16 code units**

---

## Série d'exercices 1–13 (2026-09-20 → 09-22)

Fichier : `src/main.rs`. Révision de ch. 1/3/2/4/5.1 + RBE ch. 1.
Notions rencontrées **en pratique avant d'être lues dans le Book**.

### Pointeur gras (*fat pointer*)

Taille du **paramètre lui-même**, mesurée :

| Type | Taille | Contenu |
|---|---|---|
| `&i32`, `&String` | **8 o** | une adresse |
| `&str`, `&[u8]` | **16 o** | adresse **+ longueur** |

- Une reference vers une `String` n'a pas besoin de la longueur : elle est déjà
  dans la struct, au bout du pointeur.
- Une slice vise **des octets bruts au milieu d'une chaîne** : rien n'indique où
  elle s'arrête, donc la longueur voyage **dans la reference**.
- 💡 C'est ce qui permet à `&s[2..5]` d'exister sans rien allouer.
- `taille(&string)` compile pour `fn taille(s: &str)` : ***deref coercion***,
  `&String` → `&str` automatique.

### ⚠️ NLL — un emprunt meurt à sa **dernière utilisation**

*Non-lexical lifetimes.* Les deux extraits ne diffèrent que par la **position de
la dernière lecture** de la vue :

```rust
let vue = &s;                 let vue = &s;
s.push_str(" monde");         println!("{vue}");     // dernière utilisation
println!("{vue}");            s.push_str(" monde");  // OK
// E0502                      // compile
```

- Message clé : *« immutable borrow **later used** here »*. C'est l'usage
  ultérieur qui déclenche l'erreur, pas la coexistence.
- ⚠️ Faux ami JS : en JS une variable vit jusqu'à la fin de son bloc. Le borrow
  checker raisonne sur les **usages**, pas sur les accolades.

### ⚠️ `usize` : débordement en soustraction

`while i >= 0` sur un `usize` → `warning: comparison is useless due to type
limits` (`unused_comparisons`) : un non-signé est **toujours** ≥ 0.

Et corriger la comparaison n'aurait rien réglé — `0usize - 1` :

| Profil | Comportement |
|---|---|
| **debug** (`cargo run`) | **panique** : *attempt to subtract with overflow* |
| **release** (`--release`) | **boucle silencieusement** à `usize::MAX` |

- ⚠️ **Un bug qui n'existe qu'en production.** Les vérifications de débordement
  sont désactivées en release.
- 💡 Leçon de conception : quand une boucle exige une garde compliquée, c'est
  souvent qu'elle **teste la mauvaise chose**. Ici, conditionner sur
  `!is_char_boundary(i)` rend le débordement *structurellement* impossible —
  l'indice 0 étant toujours une frontière, la boucle ne peut pas l'atteindre.

### Taille UTF-8 selon le point de code

| Point de code | Octets | Exemples |
|---|---|---|
| `U+0000`–`U+007F` | **1** | ASCII, espace, ponctuation |
| `U+0080`–`U+07FF` | **2** | `é` `ß` `Ω` `д`, arabe, hébreu |
| `U+0800`–`U+FFFF` | **3** | `€` **`☕`** `文`, symboles, CJK |
| `U+10000`–`U+10FFFF` | **4** | `😀` `🦀`, vrais emoji |

- ⚠️ **« emoji » n'est pas une catégorie de taille.** `☕` = `U+2615`, dans le
  BMP → **3 octets**, comme `€`. Les emoji colorés modernes sont au-dessus de
  `U+FFFF` → 4 octets.
- `"café ☕"` = 9 octets : 3 ASCII + `é` (2) + espace (1) + `☕` (3).
- `c.len_utf8()` donne la réponse sans deviner.
- Garanties de la recherche de frontière : **0 est toujours une frontière**
  (d'où l'arrêt), **au plus 3 crans** (4 octets max), et `is_char_boundary` est
  **O(1)** — il lit un seul octet et regarde ses bits de tête.

### ⚠️ `match` est une **expression**, typée par ses branches

```rust
let x = match saisie.trim().parse::<i32>() {
    Ok(n)  => n / 2,      // i32
    Err(_) => { …; 0 }    // i32
};                        // -> l'expression vaut i32, PAS Result
```

- ❌ Erreur commise : « le `match` produit un `Result` ». **Non** — il en
  **consomme** un. Le `Result` entre, le pattern matching l'ouvre, la valeur
  utile sort.
- Rust **exige que toutes les branches aient le même type**, sinon le type de
  l'expression serait indécidable.
- 💡 Clippy `let_and_return` : inutile de passer par `let x = match {…}; x` —
  le `match` peut être la dernière expression de la fonction.

### ⚠️ `unwrap_or` vs branche `Err` : le défaut n'agit pas au même endroit

```rust
Err(_) => 0                 // le défaut est le RÉSULTAT
.unwrap_or(0)  puis  / 2    // le défaut est l'ENTRÉE, ensuite transformée
```

Avec `0` les deux coïncident (`0 / 2 == 0`) ; **avec `10`, l'un rend `10` et
l'autre `5`**. Coïncidence trompeuse à ne pas généraliser.

### Déstructurer une tuple struct : le **nom du type** fait partie du motif

```rust
let (x) = Pieds(1.0);     // ⚠️ PAS une déstructuration : parenthèses inutiles
                          // warning: unnecessary parentheses around pattern
                          // x est un Pieds entier -> il faut encore x.0
let Pieds(valeur) = p;    // ✅ valeur est un f64
let (a, b) = (1.0, 2.0);  // tuple NU : aucun nom de type
```

### Typage nominal : aucune conversion implicite, même vers le type enveloppé

`fn en_pieds(m: Metres)` refuse un `Pieds` **et** un `f64` nu — `E0308` dans les
deux cas. `Metres` n'est pas « un `f64` étiqueté », c'est un **type neuf** ;
il faut écrire `Metres(x)`.

### Mutabilité : pas de granularité par champ

Modifier un seul champ exige que **l'instance entière** soit `mut`. Aucun
équivalent du `readonly` par champ de TS. Vaut aussi pour les emprunts :
`&mut livre` verrouille toute la struct.

---

## Chapitre 5 — Structs (en cours)

Fichier : `src/the-book/5-structures.rs` — compile sans erreur **ni warning**.

### Les trois formes de struct

| Forme | Exemple | Note |
|---|---|---|
| classique | `struct User { active: bool, … }` | champs nommés |
| tuple | `struct Color(i32, i32, i32);` | champs **anonymes**, accès `.0` |
| unité | `struct AlwaysEqual;` | **0 octet**, sert de support à un trait |

- ⚠️ « une struct a forcément des champs nommés » est **faux** : c'est vrai de la
  forme classique seulement. Ce qui définit une struct, c'est qu'elle **crée un
  type nommé**.
- Déstructurer une tuple struct exige de **nommer le type** :
  `let Point(x, y, z) = origin;` — contrairement à un tuple nu.
- **Field init shorthand** : `username,` au lieu de `username: username,`.

### ⚠️ *Struct update syntax* (`..autre`) — le piège du chapitre

**Ce n'est pas le spread `{...obj}` de JS.** `..autre` fait un **move** de chaque
champ non fourni explicitement. Vérifié sur `User` après `..user2` :

| Champ | Après `..user2` | Pourquoi |
|---|---|---|
| `active` (bool) | ✅ lisible | `Copy` → move sans invalidation |
| `sign_in_count` (u64) | ✅ lisible | `Copy` |
| `email` (String) | ✅ lisible | **fourni explicitement**, jamais touché |
| `username` (String) | ❌ `E0382` | non-`Copy` → déplacé |

- Notion clé : le ***partial move*** (**déplacement partiel**). Une struct n'est
  pas invalidée en bloc mais **champ par champ**. Elle reste utilisable pour
  tout ce qui n'a pas bougé.
- Conséquence vérifiée : la version verbeuse (`username: user2.username`) et la
  version `..user2` consomment le **même** champ — elles sont **alternatives**,
  pas équivalentes. Les deux à la suite ne compilent pas.

### Déstructurer, c'est déplacer

- `let Nom(a, b) = n;` sur des champs `String` déplace **tout** → `n` entièrement
  inutilisable.
- 💡 Sortie soufflée par `rustc` (*borrow this binding in the pattern*) :
  déstructurer **derrière une reference**, `let Nom(a, b) = &n;` → `a` et `b`
  sont des `&String`, `n` reste intact. (Formalisé au chapitre 18.)

### Pourquoi `String` et pas `&str` dans les champs

- Une struct **possède ses données**. Mettre `&str` exigerait une annotation de
  durée de vie (*lifetime*, chapitre 10).
- Règle pratique d'ici là : **une struct possède ses données.**

### ⚠️ Piège non lié aux structs : paramètres positionnels de même type

`fn build_user(username: String, email: String)` — inverser les deux arguments
**compile**. Le typage nominal ne protège pas : `String` et `String` sont le même
type. Remède : une tuple struct par concept (`struct Email(String)`).

### Struct vs `interface` TS — la correspondance correcte

| TypeScript | Rust |
|---|---|
| `interface` de **données** | `struct` — mais **nominale et concrète** |
| `interface` de **contrat** / méthodes | **`trait`** (ch. 10) |
| `class` | `struct` **+** bloc `impl`, **séparés** |
| duck typing structurel | rien — il faut nommer le type |

- ⚠️ Une `interface` TS est **effacée** à la compilation : zéro octet produit.
  Une struct **est** la disposition mémoire.
- Vérifié expérimentalement : `struct Mesure { a: u8, b: u64, c: u8 }` fait
  **16 octets** (et non 10) — bourrage d'alignement. Le compilateur
  **réordonne les champs** : offsets mesurés `b=0`, `a=8`, `c=9`. Avec
  `#[repr(C)]`, qui impose l'ordre déclaré, la même struct monte à **24 octets**.
- Rien n'« implémente » une struct : ce rôle revient aux traits.

---

## Rust by Example — ch. 1 : formatted print

### Les trois façons de nommer un argument

```rust
println!("{} days", 31);                          // positionnel implicite
println!("{0}, this is {1}. {1}, this is {0}", a, b); // index explicite
println!("{subject} {verb}", subject = "…", verb = "…"); // nommé
println!("{number:>width$}");        // capture implicite depuis la portée
```

- La **capture implicite** (`{number}` sans argument) lit la variable locale du
  même nom. Stabilisée en Rust 2021.
- ⚠️ Elle ne marche **qu'avec un nom de variable simple** : `{a.b}` ou
  `{f()}` sont refusés.

### Anatomie d'un spécificateur

```
{nom : remplissage alignement signe # 0 largeur . précision type}
```

- **Bases** : `{:b}` binaire, `{:o}` octal, `{:x}` / `{:X}` hexadécimal.
  `{:#x}` ajoute le préfixe `0x`.
- **Alignement** : `<` gauche, `>` droite, `^` **centré**.
- **Remplissage** (*fill*) : n'importe quel caractère placé **avant**
  l'alignement — `{:*>5}`, `{:->5}`, `{:.>5}`, `{:0>5}`.
- **Largeur dynamique** : `width$` (argument nommé ou variable capturée). Doit
  être de type **`usize`**.

### ⚠️ Piège : `{:0>5}` n'est pas `{:05}`

Deux syntaxes différentes qui coïncident sur les positifs et divergent sur les
négatifs. Vérifié expérimentalement :

| Valeur | `{:0>5}` (remplissage) | `{:05}` (flag zéro) |
|---|---|---|
| `42` | `00042` | `00042` |
| `-42` | `00-42` | `-0042` |

- `0>` traite le `0` comme un **caractère de remplissage quelconque** : le signe
  est poussé avec le reste.
- `{:05}` est un **flag dédié aux nombres**, conscient du signe : il garde le
  `-` devant et remplit après. Il respecte aussi le préfixe (`{:#08x}` →
  `0x0000ff`).
- 💡 Pour un nombre, préférer `{:05}` ; `0>` est correct pour du texte.
- ⚠️ **Le signe est compté dans la largeur** — personne ne le supprime :
  `{:0>3}` sur `-2` donne `0-2` (le `-` poussé *dans* le nombre), `{:03}` donne
  `-02` (le `-` gardé en tête, remplissage **après** le signe). Point mal compris
  en révision le 2026-09-20.

### Autres points

- La largeur est un **minimum** : elle ne tronque jamais (`{:>3}` sur
  `"abcdefgh"` rend la chaîne entière).
- Seuls les types qui implémentent `fmt::Display` passent dans `{}`. Une struct
  utilisateur ne l'implémente **pas** par défaut → `{:?}` via `#[derive(Debug)]`,
  ou `impl fmt::Display` à la main.

**Traductions** : *right-justified* → **aligné à droite** (ou *cadré à droite*
pour des colonnes de nombres). ⚠️ « justifié » seul, en typographie française,
désigne le **double** alignement (les deux bords nets), pas le droit.


## Tampons (*buffers*) — creusé le 2026-09-22

> ⚠️ **« Buffer » ne désigne pas une structure de données, mais un RÔLE.**
> Matériellement, c'est un tableau d'octets, rien d'autre. Chercher « c'est quoi
> un buffer » comme on chercherait « c'est quoi un tableau » ne marche pas —
> c'est comme demander « c'est quoi un stock ».

**Le motif, toujours le même** : une opération est chère **à déclencher** mais
pas beaucoup plus chère en gros volume → on accumule dans une zone d'attente et
on déclenche rarement, sur de gros paquets. *To buffer* = amortir, tamponner un
choc (comme le tampon d'un wagon).

### Famille A — amortir l'ALLOCATION (`String`, `Vec`)

```
   PILE (24 o)                    TAS (le tampon)
   ptr ──────────────────────►   [ h e l l o _ ]
   len = 5                        ◄── cap = 6 ──►
   cap = 6
```

Croissance mesurée, caractère par caractère :

```
ptr = 0x1          len=0  cap= 0   ← String::new() : AUCUNE allocation, ptr bidon
ptr = 0x105635cf0  len=1  cap= 8   ← 1er push : 8 octets d'un coup
ptr = 0x105635cf0  len=8  cap= 8   ← 7 push GRATUITS (place déjà réservée)
ptr = 0x105635cf0  len=9  cap=16   ← débordement : cap DOUBLE
```

- 💡 **`cap` est presque toujours > `len`.** La marge est délibérée.
- À la réallocation, Rust demande un nouveau tampon, **recopie** l'ancien et le
  libère. Ici l'adresse n'a pas bougé (l'allocateur a étendu sur place) mais
  **ce n'est pas garanti**.
- ⚠️ **C'est la raison profonde du borrow checker sur les slices** : si le
  tampon déménage, une slice pointerait vers de la mémoire **libérée**. D'où le
  refus de muter une `String` empruntée. Rust n'invente pas une contrainte, il
  interdit un pointeur qui peut se périmer.
- `String::with_capacity(1000)` → **0 réallocation** contre **8** avec
  `String::new()` (paliers 8, 16, 32… 1024), chacune impliquant une recopie.

### Famille B — amortir l'ENTRÉE/SORTIE (`BufWriter`, Node)

Écrire exige un **appel système** : bascule en mode noyau, des milliers de
cycles — quel que soit le volume. Écrire 1 octet coûte presque autant que 8 000.

Mesuré sur 200 000 lignes écrites dans un fichier :

| | Temps | Appels système |
|---|---|---|
| sans tampon | **823,72 ms** | 200 000 |
| avec `BufWriter` | **5,70 ms** | ~250 |
| | **144× plus rapide** | fichiers **identiques octet pour octet** |

- 💡 **`flush()`** (*vider*) = « pars maintenant, même si le chariot n'est pas
  plein ». Automatique quand le `BufWriter` sort de portée — mais **personne ne
  le fait si le programme se termine brutalement**, d'où les dernières lignes
  manquantes après un crash.
- `println!` et `console.log` écrivent tous deux dans un tampon, pas à l'écran.

### Modèle mental — trois questions suffisent

1. Quelle opération coûteuse amortit-on ? (allocation ? appel système ?)
2. Où est la réserve ?
3. Quand est-elle vidée ? (pleine, ou `flush` explicite)

### Créer un tampon à la main : Rust peut, il n'a juste pas de type `Buffer`

| Node/JS | Rust |
|---|---|
| `Buffer.alloc(1024)` | `vec![0u8; 1024]` (len 1024) |
| `Buffer.allocUnsafe(1024)` | `Vec::with_capacity(1024)` (len **0**, cap 1024) |
| `new Uint8Array(1024)` | `[0u8; 1024]` (**pile**, zéro allocation) |
| — | `String::with_capacity(n)`, `BufWriter::with_capacity(n, f)` |

- Rust nomme les types d'après **ce qu'ils sont**, pas d'après leur usage. Et il
  sépare trois rôles que Node fusionne : `Vec<u8>` **possède** les octets,
  `&[u8]` les **regarde**, `BufWriter` est le **comportement** de tamponnage
  (un emballage applicable à un fichier, une socket, `stdout`…).
- ⚠️ Pourquoi c'est flou en venant de JS : on n'y alloue **jamais** — V8 fait
  exactement la même chose (réservation, doublement, réallocation) sans exposer
  ni `.capacity()` ni `ptr`.

---

## Exercism — *Matching Brackets* (2026-09-24/25)

### La notion : une PILE (*stack*), principe **LIFO**

*Last in, first out* — dernier entré, premier sorti. En lisant `{[()]}` de
gauche à droite, chaque fermeture referme **toujours la dernière ouverture
encore en attente** :

```
{   en attente : {          )   ferme le (   reste : { [
[   en attente : { [        ]   ferme le [   reste : {
(   en attente : { [ (      }   ferme le {   reste : ∅
```

💡 **Même mot que la pile mémoire du ch. 4, et ce n'est pas un homonyme** : la
pile d'appels fonctionne ainsi pour la même raison — une fonction qui se termine
est toujours la dernière appelée encore en cours.

### L'algorithme

1. ouvrant → **empiler**
2. fermant → **dépiler** ; rien à dépiler **ou** mauvaise correspondance → `false`
3. autre caractère → ignorer
4. ⚠️ fin de chaîne → `true` **seulement si la pile est vide** (piège : `"{{"`
   atteint la fin sans jamais déclencher d'erreur)

### Fausses pistes explorées, et ce qu'elles ont appris

- ❌ **Couper la chaîne en deux moitiés et comparer.** Ne marche que pour les
  imbrications pures ; s'effondre sur `"()()"` et `"}{"`.
- ⚠️ Cette version mélangeait `chars().count()` (**caractères**) et `&s[..n]`
  (**indice d'octet**) — passait uniquement parce que les crochets sont ASCII.
- ✅ **Supprimer les paires collées jusqu'à épuisement** : algorithme **correct**
  (trouvé seul), mais O(n²) et pénible en Rust (suppression au milieu d'une
  `String` = indices d'octets + décalage). C'est la même règle que la pile, vue
  de l'intérieur au lieu de la gauche.
- 💡 **Beaucoup de bugs UTF-8 sont des bugs de conception déguisés** : la version
  finale gère `"{é(à)}"` sans effort, parce qu'elle itère sur `.chars()` et ne
  découpe jamais.

### ⚠️ Deux tableaux parallèles = une béquille JS

`opening_chars` / `closing_chars` indexés en parallèle simulent ce que JS n'a
pas : une expression qui transforme une valeur selon des cas. En Rust c'est
`match`, résolu **à la compilation** (saut direct, aucune itération).

- Risque éliminé : réordonner un seul des deux tableaux compile toujours et
  devient **faux silencieusement**. Avec `match`, la paire est sur une ligne.
- ⚠️ **Il n'existe aucun moyen de chercher dans un tableau non trié sans
  itérer.** `indexOf` en JS itère aussi — la boucle est juste cachée.
  `contains()` itère également. L'équivalent exact est
  `.iter().position(|&x| x == c)` (ch. 13).

### Détails d'écriture relevés

- `Some('(')` en motif teste **en même temps** qu'il y a quelque chose et que
  c'est le bon caractère — deux vérifications en un seul motif.
- `String::pop()` renvoie un `Option<char>` : le cas « pile vide » **ne peut pas
  échapper silencieusement**, contrairement à un `undefined`.
- ⚠️ **`continue` en dernière instruction d'une boucle est redondant** : il
  saute là où l'on serait arrivé de toute façon. La branche qui ne fait rien
  s'écrit `{}` (valeur `()`).
- `continue` et `return` ont pour type **`!`** (*never*) : ils s'unifient avec
  n'importe quel type d'arme. C'est pour ça que des `true` étaient nécessaires
  tant qu'une arme valait `bool`, et inutiles une fois toutes les armes à `()`.
- Clippy : `len() == 0` → **`is_empty()`** (sur certains types `len()` doit
  parcourir la structure).
- ⚠️ `char` comme nom de variable est **légal** (espaces de noms distincts) mais
  brouille la lecture — c'est un nom de type.

---

## Projets personnels en attente

### `vroom` — camion ASCII qui nettoie le terminal (idée du 2026-09-25)

Outil CLI : une commande efface la console en y faisant passer un gros camion en
ASCII. **Faisable intégralement avec les notions déjà vues.**

Briques :

| Besoin | Moyen |
|---|---|
| effacer l'écran | `\x1B[2J` puis `\x1B[H` (`\x1B[3J` pour l'historique) |
| cacher le curseur | `\x1B[?25l` / `\x1B[?25h` |
| animer | boucle + `std::thread::sleep(Duration::from_millis(~40))` |
| le camion | tableau de `&str`, une ligne par rangée ; décalage = espaces |
| largeur du terminal | **absent de `std`** → crate `terminal_size`/`crossterm`, ou 80 en dur |

- ⚠️ **Le piège garanti : `stdout` est tamponné.** Sans `flush()` après chaque
  image, soit rien ne s'affiche, soit tout apparaît d'un coup à la fin. Et
  `flush` vit sur le trait `std::io::Write`, absent du prélude → **`E0599`**
  (troisième occurrence du même symptôme).
- ⚠️ Si le dessin contient de l'Unicode (`─`, `█`), ni `.len()` ni
  `.chars().count()` ne donnent la **largeur d'affichage** — certains caractères
  occupent deux colonnes. Rester en ASCII d'abord.
- **Chapitre 12 du Book** (*An I/O Project: Building a Command Line Program*)
  donne exactement ce qui manque : arguments de la ligne de commande, structure
  d'un binaire, séparation de la logique et du `main`.
- Commencer en version dégradée **dès maintenant** (largeur en dur, aucun
  argument), enrichir au fil des chapitres.


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
| GC | **ownership** — libération déterministe, à la fin de portée |
| affectation = 2 références vers 1 objet | *move* : la source est **invalidée** |
| copier un objet est "gratuit" | `clone()` est explicite parce qu'il **coûte** |

---

## À venir

- **RBE ch. 1 (fin)** : `Display`/`Debug` à la main, `write!`, `{:?}` dérivé
- **Chapitre 5 (suite)** : 5.2 le programme d'exemple et `#[derive(Debug)]` /
  `{:#?}` / `dbg!`, puis 5.3 blocs `impl`, méthodes vs fonctions associées
- Chapitre 6 : enums, `Option`, `match`, `if let`
- Chapitre 9 : gestion d'erreurs, `Result`
- Chapitre 17 : « Rust est-il orienté objet ? », `dyn Trait`

## Questions ouvertes / défis en suspens

- Écrire une fonction `i32 -> &str` renvoyant `"pair"` / `"impair"`, en
  utilisant `if` **comme expression** (sans `return`)
- ✅ *Répondu au chapitre 5* : qu'apporte une struct `Point` qu'un tuple
  `(i32, i32)` n'apporte pas ? → un **type nommé**, donc nominal et
  incompatible avec un autre de même forme
- Pourquoi `#[repr(C)]` existe-t-il, si le réordonnancement des champs fait
  gagner 33 % ? Dans quel cas payer 24 octets au lieu de 16 ?
