fn main() {
    // Rust es fuertemente tipado

    // enteros
    // i8, i16, i32, i64, i128, int con signo +, -
    // u8, u16, u32, u64, u128, int sin signo solo +
    let numero_uno: i8 = -10;
    let numero_dos: u8 = 10;

    // cuando se declara y no se usa variables se puede usar el _ para evitar el warning

    //char UTF-8, acepta emojis
    let caracter = 'a';

    //numeros reales
    // f32, f64
    let real: f32 = 12.5;

    //bool
    let resultado: bool = true;

    println!("{}, {}, {}, {}, {}", numero_uno, numero_dos, caracter, real, resultado);

}
