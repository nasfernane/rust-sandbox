#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 50,
    };

    println!("Area is {}", rect1.area());

    if rect1.width() {
        println!("rect1 width is {}", rect1.width)
    }

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    // fonctions associées
    // ce ne sont pas des méthodes mais des fonctions associées 
    // souvent utilisées comme constructeurs qui retournent une instance de la structure
    let _rect4 = Rectangle::square(50); // même syntaxe que pour les namespaces

    dbg!("rect 4 is {}", _rect4);
}
