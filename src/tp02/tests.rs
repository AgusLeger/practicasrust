#[cfg(test)]
mod testtp02
{
    #[test]
    fn test_ejercicio1()
    {
        use crate::tp02::Ejercicio1::es_par;
        assert!(es_par(200));
        assert!(!es_par(201));
    }
    #[test]
    fn test_ejercicio2()
    {
        use crate::tp02::Ejercicio2::es_primo;
        assert!(es_primo(47));
        assert!(!es_primo(50));
    }
    #[test]
    fn test_ejercicio3()
    {
        use crate::tp02::Ejercicio3::suma_pares;

        assert_eq!(suma_pares([16,4,17]),20);
        assert_ne!(suma_pares([3,2,3]),4);
    }
        #[test]
    fn test_ejercicio4()
    {
        use crate::tp02::Ejercicio4::cantidad_impares;  //devuelve un i32

        assert_eq!(cantidad_impares([2;5]),0);
        assert_ne!(cantidad_impares([3;5]),4);
    }
        #[test]
    fn test_ejercicio5()
    {
        use crate::tp02::Ejercicio5::duplicar_valores;  //devuelve un arreglo de f32

        assert_eq!(duplicar_valores([0.5, 0.5, 0.5, 0.5, 0.5]),[1.0, 1.0, 1.0, 1.0, 1.0]);
        assert_ne!(duplicar_valores([0.5, 0.5, 0.5, 0.5, 0.5]),[0.5, 0.5, 0.5, 0.5, 0.5]);
    }
        #[test]
    fn test_ejercicio13()
    {
        use crate::tp02::Ejercicio13::ordenar_nombres;  //devuelve un arreglo de strings ordenado
        let mut chain = ["andar".to_string(),"caramelo".to_string(),"merluza".to_string(),"plomero".to_string(),"falopa".to_string()];
        ordenar_nombres(&mut chain);
        assert_eq!(chain,["andar".to_string(),"caramelo".to_string(),"falopa".to_string(),"merluza".to_string(),"plomero".to_string()]);
        assert_ne!(chain,["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]);
    }
    #[test]
    fn test_ejercicio14()
    {
        use crate::tp02::Ejercicio14::incrementar; //recibe un f32 y le suma 1.0

        let mut num: f32 = 4.918;
        incrementar(&mut num);
        assert_eq!(num, 5.918);
    }
}