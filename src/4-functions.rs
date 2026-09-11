fn main() {
    println!("Hello, world!");

    print_number(5);
    print_labelled_number(45, 'h');

    // FONCTIONS

    // différence entre déclaration et expression
    // une déclaration ne retourne pas de valeur, on ne peut donc pas faire
    // let x = let y = 6; => error: expected expression, found `let` statement

    // une expression peut faire partie d'une déclaration mais retourne une valeur
    // 5 + 6 est une expression qui s'évalue à 11
    // appeler une fonction, une macro ou l'ouverture d'un bloc sont des expressions
    //
    // let y = {
    //     let x = 3;
    //     x + 1 // pas besoin d'accolade ici, c'est la valeur retournée par le bloc
    // };

    // println!("The value of y is {y}");

    // fonctions avec valeurs de retours
    // function_name(...parameters) -> return_type {}
    // fn amazing_function() -> i8 {
    //     6
    // }

    // let amazing_number = amazing_function();

    // println!("My amazing number is {amazing_number}");

    let incremental_number = add_one(5);
    println!("Incremental number is {incremental_number}");
}

fn print_number(number: i32) {
    println!("Wow {number} is a beautiful number");
}

fn print_labelled_number(value: i32, unit_label: char) {
    println!("Wow {value}{unit_label} is a a lot of {unit_label}");
}

fn add_one(x: i32) -> i32 {
    x + 1 // si on ajoute un semi-colon => ^^^ expected `i32`, found `()` car la fonction ne retourne plus rien
    // return ne doit être utilisé que pour des retours anticipés
}
