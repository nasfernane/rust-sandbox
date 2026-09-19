fn main() {
    // Dangling references / Références pendantes
    // Une référence pendante est une référence qui pointe vers une adresse qui a été libérée: le propriétaire de la valeur a disparu.
    // La règle: Une référence ne doit jamais vivre plus longtemps que la valeur qu'elle emprunte. C'est vérifié par le borrow checker.
    let ref_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     // ici on ressaie de retourner une référence qui pointe vers une valeur libérée à la fin du scope de la fonction
//     // la référence pointe vers une adresse libérée, s n'existe plus
//     &s // error[E0106]: missing lifetime specifier, this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// }

fn dangle() -> String {
    let s = String::from("Hello");

    s // solution: on retourne la valeur réelle et on transfère l'ownership de s
}

// Règles des références
// On peut avoir soit une seule référence mutable, soit plusieurs références immutables
// les références doivent toujours être valides (durée de vie qui ne dépasse pas la durée de vie de la variable)
