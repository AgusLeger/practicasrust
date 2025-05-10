fn main()
{
    let array : [i32;5] = [30;5];

    let tupla = ("Marallrá ".to_string(), array);

    println!("texto = {}",tupla.0);
    println!("------ARREGLO-------");

    for num in tupla.1
    {
        println!("{num}");
    }
}
/*12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de 
enteros, y luego imprima la cadena y la suma de los valores en el arreglo.  */