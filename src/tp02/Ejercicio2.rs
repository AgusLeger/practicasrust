fn main()
{
    println!("es primo: {}",es_primo(50)); //2..50 probados
}
pub fn es_primo(num : u32) -> bool
{
    let mut bul = false;
    if (num > 3) && (num != 5)
    {
        //println!("ENTRE 1");
        if num % 2 != 0
        {
            //println!("ENTRE 2");
            if num % 3 != 0
            {
                //println!("ENTRE 3");
              'verificar: for i in 5..num-1 //quizás un while (num % i != 0) && (i != num-1)
              {
                //println!("LOOP {}",i);
                if num % i == 0
                {
                    //println!("ROMPÍ TODO");
                    bul = false;                //seguramente sea mejor dejarlo en true y ir cambiado a false
                    break 'verificar;
                }
                //println!("SALGO");
                bul = true;                    //un poco repetitivo, revisar mañana
              }
            }
        }
    }else 
    {
        bul = true;
    }
    bul
}
/*2- Definir la función llamada es_primo  que recibe un número entero positivo mayor a 1 y 
retorna true si es primo, false caso contrario. */