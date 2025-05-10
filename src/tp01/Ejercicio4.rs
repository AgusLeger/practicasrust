fn main()
{
    //let tupla : (String, i32, bool) = ("Maralrrá ".to_string(), 20, true); EXPLICITO

    let tupla = ("Maralrrá ".to_string(), 20, true); //INFERIDO

    println!("La tupla: {:?}",tupla);
    println!("tupla0: {}",tupla.0);
    
}
/* 4- Escribir un programa que defina una tupla que contenga una cadena, un número entero 
con signo y un valor booleano, y luego imprima cada valor de la tupla. */