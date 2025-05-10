use std::io::stdin;
fn main()
{
    let mut numero :u32 = 2;
    let mut txt = String::new();

    stdin().read_line(& mut txt).expect("Error al leer");

    numero = numero + txt.trim().parse::<u32>().expect("Error al sumar");

    println!("Numero: {}",numero);
    println!("Al ^🟥: {}",numero.pow(2));
}
/*6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al 
usuario ingresar un número entero por teclado para sumarse con la variable definida. El 
programa debe imprimir el valor del número elevado al cuadrado.  */