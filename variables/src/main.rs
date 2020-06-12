fn main() {
    // println!("Hola mundo desde cargo");
    let numero_uno: i32 = 10; // define tipo de dato let <identificador>: <tipo> = <valor>
    let mut numero_dos = 15; // o se puede hacer de forma dinamica

    // definir constantes se tiene que definir el tipo de dato, de preferencia las constantes en mayusculas
    const VALOR: i32 = 10;

    numero_dos = 20;
    let resultado = numero_uno + numero_dos + VALOR;
    println!("El resultado de ({} + {} + {}) es: {}", numero_uno, numero_dos, VALOR, resultado);
}
