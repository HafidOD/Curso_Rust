fn main() {
    let valor: i32 = 10;
    println!("El valor de la variable es: {}", valor);

    // al detectar que se usa una variable con un mismo nombre se elimina la variable anterior y se crea una nueva
    // esto se le conoce como shadowing
    let valor = 20;
    println!("El valor de la variable es: {}", valor);

    let valor = false;
    println!("El valor de la variable es: {}", valor);
}
