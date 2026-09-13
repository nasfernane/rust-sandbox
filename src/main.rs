fn main() {
    // STACK VS HEAP
    // le stack (pile) est utilisé pour les données dont la taille est connue à la compilation, les dernières données entrées sont les premièes à sortir (pile LIFO)
    // le heap (tas) est un ensemble moins ordonné utilisé pour les données dont la taille n'est pas connu à la compilation. On parle d'allocation et les données ne sont pas récupérées dans un ordre précis, c'est donc moins performant de stocker et récupérer des données sur le heap mais plus souple.
    //
    // Règles de ownership
    // Toutes les valeurs en Rust ont un owner (propriétaire)
    // Il ne peut y avoir qu'un seul propriétaire à la fois
    // quand le owner sort du scope, la valeur est libérée
    // la libération est automatique pour toutes les valeurs possédantes (Drop d'exécuter du code personnalisé au moment de la libération)

    //  pour les valeurs simples, rust copie les deux valeurs dans le stack donc pas de problème
    // let _x = 5;
    // let _y = _x;

    // println!("x is {_x}, y is {_y}");

    // pour les valeurs complexes, comme un String, les deux variables pointent vers les mêmes valeurs dans le heap mais la première valeur est invalidée et la deuxième récupère l'ownership. Plus qu'un seul propriéataire;
    // ce mécanisme fait en sorte qu'il n'y ait qu'une seule libération, pour éviter une double tentative de drop si la variable sort du scope
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("s2 is {s2}");
    // println!("s1 is {s1}"); // error[E0382]: borrow of moved value: `s1`

    // on peut forcer la copie avec clone() mais il faut envisager la perte de performance
    // let s1 = String::from("Hello");
    // let s2 = s1.clone(); // ok tier
    // println!("s2 is {s2}");
    // println!("s1 is {s1}");

    // les valeurs dont le type implémentent le Trait "Copy" ne sont pas déplacées
    // Un même type ne peut pas impléenter à la fois Copy et Drop

    // Ownership et les fonctions
    // let s = String::from("Hello");
    // take_ownership(s);
    // println!("Initial string is {s}"); // error[E0382]: borrow of moved value: `s`
    // // le type String n'implémente pas le trait Copy, l'ownership de s est transféré à la fn take_ownership()

    // let n = 8;
    // makes_copy(n); // pas de problème, le type i32 implémente Copy, c'est une valeur scalaire simple qui ne coûte pas grand chose à copier donc la valeur est dupliquée automatiquement
    //

    // les return peuvent aussi redonner l'ownership
    let s = gives_ownership(); // récupère l'ownership de la variable retournée par gives_ownership
    let s2 = takes_and_gives_back(s); // s donne l'ownership à la fn, qui la retourne ensuite à s2

    // println!("{s}"); // s a été déplacée dans s2 => error[E0382]: borrow of moved value: `s
    println!("{s2}");
}

// fn take_ownership(string: String) {
//     println!("move string is {string}");
// } // la valeur string est libérée donc le contenu initial s n'existe plus

// fn makes_copy(number: i32) {
//     println!("copied number is {number}");
// }

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
