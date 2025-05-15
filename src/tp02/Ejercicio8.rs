fn main()
{
    let ar1 :  [f32;5] = [1.23 , 47.8 , 564.61 , 4.68, 0.5];
    let ar2 :  [f32;5] = [32.11 , 89.8 , 1.61 , 401.68, 0.6];

    let ar_suma : [f32;5] = sumar_arreglos(ar1,ar2);

    println!("{:?}\n{:?} + \n{:?} ",ar1,ar2,ar_suma);
}

fn sumar_arreglos(arreglo1 : [f32;5], arreglo2 : [f32;5]) -> [f32;5]
{
    let mut arreglo_total : [f32;5] = [0.0;5];
    //let mut suma = 0.0;

    for i in 0..arreglo1.len() //esto sirve porque los 2 arreglos miden lo mismo
    {
        /*suma = arreglo1[i] + arreglo2[i]; //quizás más prolijo así
        arreglo_total[i] = suma; */
        arreglo_total[i] = arreglo1[i] + arreglo2[i];
    }
    arreglo_total //return el arreglo con la suma
}

/*8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de 
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los 
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los 
arreglos pasados por parámetro.  */