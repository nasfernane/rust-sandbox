fn main() {
    // STACK VS HEAP
    // le stack (pile) est utilisé pour les données immutables, les dernières données entrées sont les premièes à sortir (pile LIFO)
    // le heap (tas) est un ensemble moins ordonné utilisé pour les données dont la taille peut changer. On parle d'allocation et les données ne sont pas récupérées dans un ordre précis, c'est donc moins performant de stocker et récupérer des données sur le heap mais plus souple.
    //
    // Règles de ownership
    // Toutes les valeurs en Rust ont un owner (propriétaire)
    // Il ne peut y avoir qu'un seul propriétaire à la fois
    // quand le owner sort du scope, la valeur est libérée (via le trait Drop)

    //  pour les valeurs simples, rust copie les deux valeurs dans le heap donc pas de problème
    let _x = 5;
    let _y = _x;

    println!("x is {_x}, y is {_y}");

    // pour les valeurs complexes, comme un String, les deux variables pointent vers les mêmes valeurs dans le heap mais la seconde variable récupère l'ownership;
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("s2 is {s2}");
    // println!("s1 is {s1}"); // error[E0382]: borrow of moved value: `s1`

    // on peut forcer la copie avec clone() mais il faut envisager la perte de performance
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // ok tier
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    // les valeurs dont le type implémentent le Trait "Copy" ne sont pas déplacées
    // Un même type ne peut pas impléenter à la fois Copy et Drop
    // 
    
}
