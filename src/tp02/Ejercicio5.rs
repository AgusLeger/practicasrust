fn main()
{
    let array : [f32;5] = [4.12,68.9,5.1,36.01,12.04];

    println!("{:?}",array);

    let vector : [f32;5] = duplicar_valores(array);

    println!("----------------------------------");

    println!("{:?}",vector);
}
pub fn duplicar_valores(array : [f32;5]) -> [f32;5]
{
    let mut arreglin : [f32; 5] = [0.0;5];

    for i in  0..array.len() //array.iter()
    {
        arreglin[i] = array[i] * 2.0; //Si uso el iter toma el elemento
    }
    arreglin
}
/*5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y 
retorna un arreglo nuevo con los valores duplicados del parámetro.  */