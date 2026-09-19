struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // une structure permet de regrouper plusieurs valeurs comme un tuple
    // mais toutes les valeurs doivent être nommées et on peut y accéder plus facilement

    let mut user1 = User {
        // new instance
        active: true,
        username: String::from("username123"),
        email: String::from("user@email.com"),
        sign_in_count: 67,
    };

    user1.email = String::from("another@email.com");

    let user2 = build_user(String::from("Furimi"), String::from("furimi@email.com"));
    let active_label = if user2.active { "Active" } else { "Inactive " };

    println!(
        "{} user {} email is {} - signed {} times",
        active_label, user2.username, user2.email, user2.sign_in_count
    );

    // on peut réutiliser des informations d'une autre instance du même type
    // let _user3 = User {
    //     active: user2.active,
    //     username: user2.username,
    //     email: String::from("another@email.fr"),
    //     sign_in_count: user2.sign_in_count,
    // };

    // ou en version plus consise
    let _user4 = User {
        email: String::from("another@email.fr"),
        ..user2
    };

    // TUPLE STRUCTS
    // une structure tuple permet d'avoir un nommage qui donne une signification à l'ensemble
    // sans avoir à nommer chaque champ individuellement
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // on peut accéder aux valeurs de la même façon qu'un tuple classique
    println!("{}, {}, {}", _black.0, _black.1, _black.2);

    // par contre pour déstructurer contrairement à un tuple il faut nommer le type
    let Point(x, y, z) = _origin;
    println!("x is {x}, y is {y}, z is {z}")
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}
