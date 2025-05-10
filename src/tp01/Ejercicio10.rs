fn main()
{
    let arreglo : [i32;5] = [10;5];
    let gloarre : [i32;5] = [1;5];
    let mut suma : [i32;5] = [0;5];

    for i in 0..arreglo.len()
    {
        suma[i] = arreglo[i] + gloarre[i];
    }

    for total in suma
    {
        println!("{}",total);
    }
}
/*10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego 
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos 
originales. */