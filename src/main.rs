fn main() {
    let number = 10;

    // if number < 10 {
    //     println!("Small number");
    // } else {
    //     println!("big number !");
    // }

    // contrairement en js il faut que la condition soit un booléen
    // if number {
    //     // ==> error[E0308]: mismatched types
    //     println!("Small number");
    // } else {
    //     println!("big number !");
    // }

    // ici if est une expression qui renvoie une valeur, on s'en sert comme un ternaire
    // let conditional_number = if number < 10 { number } else { 50 };
    // println!("Conditional number is {conditional_number}");

    // le type de la variable dépend donc du type de valeur renvoyé par le if, il faut que les types soient homogènes
    // let conditional_number = if number < 10 { number } else { "coucou" };
    // println!("Conditional number is {conditional_number}");
    // error[E0308]: `if` and `else` have incompatible types

    // LOOPS
    //
    // infinite loop
    // loop {
    //     println!("To infinity and beyond...")
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    // break permet de sortir de la boucle, mais return sort toujours de la fonction
    //     if counter >= 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("Result is {result}");

    // break

    // on peut nommer une boucle avec 'name, ce qui permet de les cibler avec un break dans une autre boucle imbriquée ex:
    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("Count = {count}");

    //     let mut remaining = 10;

    //     loop {
    //         println!("Remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // println!("End count = {count}");

    // boucles conditionnelles
    // let mut number = 0;

    // while number < 3 {
    //     println!("Number is {number}");
    //     number += 1;
    // }

    // println!("Final number is {number}");

    // boucler sur une collection
    let arr = [10, 20, 30, 40, 50];

    // let mut index = 0;

    // while index < arr.len() {
    //     println!("Index value is {}", arr[index]);

    //     index += 1;
    // }

    // ou alors de manière plus simple
    // for value in arr {
    //     println!("Value is {value}");
    // }
    //
    // utiliser une boucle for avec une range:

    for value in (1..4).rev() {
        println!("Value is {value}");
    }
}
