fn main()
{
    let mult = 2;
    let mut array = [65,40,30,2,7,9];

    for i in 0..array.len()
    {
        println!("{} => {}",array[i], array[i] * mult);
        array[i] = array[i] * mult;
    }

    println!("--------------------------------------------------------");

    for i in array
    {
        println!("arreglo valor: {}",i);
    }

}
/* 7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números 
enteros, y luego multiplique cada valor del arreglo por un valor constante definido, 
modificando el contenido del arreglo.*/