fn main() {
    // en rust les variables sont immutables par défaut. il faut ajouter mut pour les rendre mutable
    // let mut x = 5;
    // println!("X = {}", x);

    // x = 6;
    // println!("The value of x is {x}");

    // les constantes sont toujours immuables et doivent toujours avoir un type
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // on peut shadow une variable let immutable en la redéclarant. ça permet de changer son type qui est inféré
    // let spaces = "didadidadoum";

    // {
    //     let spaces = spaces.len();
    //     println!("spaces length in inner scope is {spaces}");
    // }

    // // le shadow disparait en dehors de son scope, la variable revient à sa valeur initiale
    // println!("spaces length is {spaces}");
    // 

    // par contre on ne peut pas changer le type d'un let mutable

    let mut spaces = "coucou";
    spaces = spaces.len(); // error[E0308]: mismatched types
}
