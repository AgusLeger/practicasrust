use std::io::stdin;
fn main ()
{
    let testo = "ijustwantedtobeoneofthestrokes".to_string();
    let mut letra = String::new();
    let mut numero = 0;
    
    stdin().read_line(&mut letra).expect("Error al leer");
    let caracter = letra.trim().parse::<char>().expect("ERROR AL PARSEAR");

    for i in testo.chars()
    {
        //println!("{}",i);
        if caracter == i
        {
            numero = numero + 1;
        }
    }
    println!("CANTIDAD DE {} EN LA ORACION ES: {}",caracter,numero);
}
/*8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el 
número de veces que un caracter específico ingresado por el usuario aparece en la cadena. 
Se debe imprimir el resultado. */