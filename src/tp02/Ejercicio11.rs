fn main()
{
    let mut array : [i32;5] = [13;5];

    println!("{:?}", array);
    
    multiplicar_valores(&mut array,3); //39

    println!("{:?}", array);
}

fn multiplicar_valores(ar : &mut [i32;5], factor : i32)
{
    for i in 0..ar.len()
    {
        ar[i] = ar[i] * factor;
    }
}
/*11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de 
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo 
por el parámetro factor modificándolo. */