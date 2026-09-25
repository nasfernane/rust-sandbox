// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
//
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // on peut utiliser n'importe quel, tuple, structure, même un autre enum
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// exemple d'enum plus complexe
// on pourrait faire l'équivalent avec des structurées dédiées mais ça sera moins simple
// de définir une fonction qui accepte n'importe lequel de ces types de messages en paramètre
#[derive(Debug)]
enum Message {
    Quit,                    // pas de données associées
    Move { x: i32, y: i32 }, // champs nommé comme une structure
    Write(String),
    ChangeColor(i32, i32, i32), // inclut 3 valeurs i32 comme un tuple
}

// on peut implémenter des fonctions associées et des méthodes comme pour une structure
// #[derive(Debug)]
impl Message {
    fn call(&self) {
        println!("Who you gonna call ?");
    }
}

fn main() {
    // création d'instances de chaque variante de IpAddrKind
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(four);
    // route(six);

    // on pourrait être tenté d'utiliser une structure pour organiser les adresses IP
    // let _home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // en fait on peut utiliser une méthode plus concise directement avec l'enum
    let home_ip = IpAddrKind::V4(127, 0, 0, 1);
    let work_ip = IpAddrKind::V6(String::from("::1"));

    route(home_ip);
    route(work_ip);

    let msg = Message::Write(String::from("Coucou la famille"));

    msg.call();

    // Option
    // Le type Option est un enum standard
    // il permet de définir le concept d'une valeur qui peut être présente ou absente
    // enum Option<T> { 
    //  None,
    //  Some(T),
    // }
    // 
    let some_number = Some(5); // Option<i32>
    let some_char = Some('e'); // Option<char>

    // annotation nécessaire car pas de valeur à l'initialisation
    let absent_number: Option<i32> = None; // Option<i32> mais déclaré sans valeur
}

fn route(ip_kind: IpAddrKind) {
    dbg!("ip kind is {}", ip_kind);
}
