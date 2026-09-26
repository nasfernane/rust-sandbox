enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u32) -> bool {
        match self {
            UsState::Alabama => year >= 1818,
            UsState::Alaska => year >= 1930,
        }
    }
}

fn main() {
    // if let
    //
    let config_max = Some(3u8);

    // version initiale avec un match
    match config_max {
        Some(max) => println!("This maximum is configured to be {max}"),
        _ => (),
    }

    // if let permet une version plus concise
    // on évalue un seul cas et ignore tous les autres, let if n'est pas exhaustif
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // if let else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Penny;

    // match coin {
    //     // Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     Coin::Quarter(UsState::Alaska) => println!("State quarter from Alaska!"),
    //     _ => count += 1,
    // }

    // on peut utiliser if let else en alternative
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("count is {count}");

    // let else
    // imaginons une fonction dans laquelle on veut faire des traitements plus complexes en fonction de la valeur
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        // ici ça fonctionne, on utilise une méthode de state mais deux blocs if imbriqués
        // if let Coin::Quarter(state) = coin {
        //     if state.existed_in(1900) {
        //         Some(format!("{state:?} is pretty old, for America !"))
        //     } else {
        //         Some(format!("{state:?} is not that old"))
        //     }
        // } else {
        //     None
        // }
        //
        // on peut simplifier en renvoyant la valeur dans le if let
        // on vérifie d'abord si la pièce est un quarter
        // let state = if let Coin::Quarter(state) = coin {
        //     state
        // } else {
        //     return None;
        // };

        // if state.existed_in(1900) {
        //     Some(format!("{state:?} is pretty old, for America !"))
        // } else {
        //     Some(format!("{state:?} is not that old"))
        // }

        // C'est mieux construit mais ce n'est toujours pas le plus simple à lire
        // la première branche de if let Coin::Quarter(state) renvoie une valeur
        // mais la seconde sort complètement de la fonction
        // on peut utiliser let else pour simplifier encore
        // fonctione comme if let avec un motif à gauche et une expression à droite
        // mais comportement seulement un bras "else"
        // si le pattern matche la valeur sera lié depuis le motif, ici "state"
        let Coin::Quarter(state) = coin else {
            // ici ce bras doit absolument diverger car  le reste du code exige que state existe
            // donc le bras else doit forcément terminer par return, break, continue ou panic
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America !"))
        } else {
            Some(format!("{state:?} is not that old"))
        }
    }
}
