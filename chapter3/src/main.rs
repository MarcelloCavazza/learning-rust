fn main() {
    let x = 5;
    println!("{x}");
    let x = x + 7;
    {
        let x = x * 2;
        println!("{}", x)
    }
    println!("{x}");

    const HELLO_WORLD: u32 = 60 * 60 * 3;
    println!("{}", HELLO_WORLD)
}
