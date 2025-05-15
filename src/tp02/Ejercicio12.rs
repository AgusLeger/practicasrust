fn main()
{
    let mut ar : [i32;5] = [45,54,55,86,5];
    //                          ^     ^    = -1
    println!("{:?}",ar);
    reemplazar_pares(&mut ar);
    println!("PARES ELIMINADOS");
    println!("{:?}",ar);
}

fn reemplazar_pares(arreglo : &mut [i32;5])
{
    for numero in arreglo                            //FUNCIONA ME VUELVO LOCO TIENE QUE CERRAR LA TOTALIDAD DE LAS IDES DE PROGRAMACION
    {
        if *numero % 2 == 0
        {
            *numero = -1;
        }
    }
}

/*12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y 
reemplaza todos los números pares por -1. 


fn reemplazar_pares(arreglo : &mut [i32;5])
{
    for i in 0..arreglo.len()
    {
        if arreglo[i] % 2 == 0
        {
            arreglo[i] = -1;
        }
    }
}*/