fn main() {
    // line comment
    /*
     * like comments
     * omg omg omg
     */
    println!("Hello world !");
    let x = 5 + /* block comment within expression */ 5;
    println!("Is x 10 or 100 ? x = {}", x);
}
