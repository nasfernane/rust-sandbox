fn main() {
    let string = String::from("Salut la famille");
    let first = first_word(&string); // ici &string est équivalent à un slice entier de string
    let first_ref = first_word(&string[0..8]);

    // string.clear(); // si on clear la String, len n'a plus de sens si on renvoie un index

    println!("first is {first}");
    println!("first_ref is {first_ref}");

    // string literals
    // on comprend mieux pourquoi les string literals sont immutables - le type est &str
    // les string literals sont stockés directement sur le binaire
    // la variable coucou est donc une référence qui pointe vers sur un point spécifique du binaire
    let coucou = "Coucou la famille";

    // pareil avec &str on peut passer la référence directement ou utiliser string slice
    let _first = first_word(&coucou);
    let _firstref = first_word(&coucou[0..8]);

    // les slices fonctionnent avec d'autres types de collections, ici un tableau
    let super_array = [0, 1, 2, 3, 4];
    let super_array_slice = &super_array[..3];
    assert_eq!(super_array_slice, &[0, 1, 2]);

    println!("super array slice !");
    for (el, index) in super_array_slice.iter().enumerate() {
        println!("el is {el} at index {index}");
    }
}

fn first_word(s: &str) -> &str {
    // utiliser &str permet d'utiliser la même fonction pour les &String et les &str
    let bytes = s.as_bytes();

    // pour les string slices il faut découper la phrase en fonction des octets, sinon en fonction des caractères à taille variable on peut tomber au milieu d'un caractère et provoquer une erreur
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[..i];
        }
    }

    &s[..]

    // le problème en renvoyant un index c'est que si la valeur initiale de s change ou est libérée, l'index porte une valeur qui n'a plus de sens
    // la solution est d'utiliser une string slice (une tranche) qui est une référence d'une partie de s
    // le compilateur s'assure que les références restent valides, donc plus de risque que la valeur soit libérée
    // avant l'utilisation de la référence

    // pour un autre contexte on pourrait boucler directement sur les caractères et leurs indexes
    // for (i, value) in s.char_indices() {
    //     println!("index {i} for value {value}");
    // }
}
