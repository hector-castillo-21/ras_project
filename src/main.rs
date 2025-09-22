mod math; // declaramos el módulo math

fn main() {
    let a = 8;
    let b = 3;

    println!("{} + {} = {}", a, b, math::suma(a, b));
    println!("{} - {} = {}", a, b, math::resta(a, b));
}
