fn main() {
    // references and borrowing
    //
    // on peut passer la référence d'une variable sans transférer l'ownership avec &
    // let s1 = String::from("Hello");
    // let s1_len = calculate_length(&s1);

    // println!("S1 value is {s1}");
    // println!("S1 length is {s1_len}");

    let mut s2 = String::from("Hello");
    println!("S2 value is {s2}");

    // change(&mut s2);
    // println!("S2 vakue after change is {s2}");

    // let s3 = &mut s2;
    // let s4 = &mut s2; // error[E0499]: cannot borrow `s2` as mutable more than once at a time
    // on ne peut pas avoir deux références mutables simultanées à la même variable

    // si les références ne sont pas simultanées, pas de problème
    // {
    //   let _s3 = &mut s2;
    // }

    // let _s4 = &mut s2;
    // println!("{s3} {s4}");
    // 

    // on ne peut pas non plus combiner référence mutable et immutable
    // let _s3 = &s2;
    // let _s4 = &mut s2;
    // println!("{_s3} {_s4}"); // error[E0502]: cannot borrow `s2` as mutable because it is also borrowed as immutable
    // 
    // IMPORTANT: la portée d'une référence se termine la dernière fois que la référence est utilisée 
    let _s3 = &s2;
    let _s4 = &s2;
    println!("{_s3} {_s4}"); // le scope des deux références se termine ici

    let _s5 = &mut s2; // pas de problème à la compilation

}

// fn calculate_length(string_ref: &String) -> usize {
//     let len = string_ref.len();

//     // on ne peut pas modifier une référence
//     // string_ref.push_str("Coucou"); // error[E0596]: cannot borrow `*string_ref` as mutable, as it is behind a `&` reference

//     len
//     // comme string_ref est une référence, calculate_length ne récupère pas la propriété de la variable passée en argument
//     // la variable n'est donc pas libérée à la fin du scope
// }

// fn change(string_mutable_ref: &mut String) {
//     // une référence mutable peut modifier la valeur sans posséder l'ownership
//     string_mutable_ref.push_str(" world !")
// }
