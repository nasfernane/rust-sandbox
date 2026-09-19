fn main() {
    // formatted print
    //
    //
    println!("{} days", 31);

    // positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Furimi", "Cromequis");

    // named arguments
    println!(
        "{subject} {verb} {object}",
        subject = "Furimi",
        verb = "mange",
        object = "la pâtée"
    );

    // formater des nombres avec ":"
    let number = 69420;
    println!("Base 10: {}", number); // base 10 par défaut
    println!("Binary: {:b}", number);
    println!("Octal: {:o}", number);
    println!("Hexadecimal: {:x}", number);

    // cadrage à droite - ajout d'espaces avant le caractère
    println!("{number:>5}", number = 1);

    // même principe pour ajouter des caractères d'alignement à gauche d'un nombre
    println!("{number:0>5}", number = 1);
    // en inversant l'alignement, on comble à droite
    println!("{number:0<5}", number = 8);
    // centrer l'alignement
    println!("{number:0^5}", number = 8);

    // utiliser un argument nommé avec $
    println!("{number:0<width$}", number = 1, width = 12);

    // Note: seulement les types qui implémentent fmt::Display peuvent être formattés avec {}
    // Les types créés par des utilisateurs n'implémentent pas fmt::Display par défaut
    //
    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct {}", Structure(3)); // Structure doesn’t implement std::fmt::Display
    //

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");

    // std:fmt contient des traits pour définir l'affichage des textes, les deux plus importants
    // fmt::Display avec {} comme déjà vu avant
    println!("Coucou le meileur chat, {}", "Furimi");
    // fmt:Debug avec {:?}
    println!("Alerte envahisseur, {:?}", "Yuki");

    // formater le nombre de décimales
    let pi = 3.141592;

    println!("Pi is {:.3}", pi)
}
