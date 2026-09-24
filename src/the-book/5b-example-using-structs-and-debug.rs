fn main() {
    // version 1 basique
    // let width1 = 30;
    // let height1 = 50;
    //
    // // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }
    //

    // // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
    //

    // version 2 avec tuple => plus structuré mais pas de nom/sens donné aux valeurs
    // let dimensions = (30, 50);
    //
    // fn area(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(dimensions)
    // );

    // version 3 avec struct
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // // version 3 avec struct
    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }

    // println!("Rect 1 is {:#?}", rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
    //
    // utiliser dbg!
    // c'est une macro de print comme println! mais qui prend et redonne la propriété au lieu de prendre une référence
    // la macro output dans le stream console stderr au lieu de stdout
    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}
