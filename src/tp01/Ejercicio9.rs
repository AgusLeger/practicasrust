fn main()
{
    let arreglo : [i32;5] = [20;5];
    let mut suma = 0;

    for num in arreglo
    {
        println!("{num}");
        suma = suma + num;
    }

    println!("------------------------------");
    println!("LA SUMA ES: {suma}");
}
/*9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la 
suma de los valores del  arreglo.  */