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

fn main() {
    // match control flow

    fn value_in_cents(coin: Coin) -> u8 {
        // mot clé match suivi d'une expression, ici la valeur passé en argument
        match coin {
            // chaque ligne est un bras (arm) avec deux parties:
            // un motif (pattern) et le code associé, séparé par =>
            // si le motif correspond à la valeur, la partie de droite est exécutée
            Coin::Penny => 1,
            Coin::Nickel => {
                println!("Wow that's a nickel");
                5
            }
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    let some_quarter = Coin::Quarter(UsState::Alabama);

    value_in_cents(some_quarter);

    // Le motif match avec Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(7) => Some(10),    // match une valeur précise
            Some(i) => Some(i + 1), // ici i récupère n'importe quelle valeur et la nomme
            // match is exhaustif et doit couvrir toutes les possibilités.
            // None => None, // si on supprime ce bras, erreur E0004
            // deux façons d'attraper n'importe quelle autre valeur
            // other => plus_one(other), // other récupère et lie la valeur
            _ => None, // _ ignore la valeur
        }
    }

    let five = Some(5);
    let six = dbg!(plus_one(five));
    let ten = dbg!(plus_one(Some(7)));

    let none = dbg!(plus_one(None));
}
