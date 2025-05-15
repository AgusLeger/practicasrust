#[cfg(test)]
mod testtp03
{
    use crate::tp03::Ejercicio6::{Estudiante, Examen};

//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-1 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
    #[test]//✅
    fn test_new_persona()                          //crea una nueva persona
    {
        use crate::tp03::Ejercicio1::Persona;

        let sugus = Persona::new("Agus".to_string(), 90, Some("Maralrra".to_string()));

        assert_eq!(sugus.nombre, "Agus".to_string());
        assert_eq!(sugus.edad, 90);
        assert_eq!(sugus.direccion, Some("Maralrra".to_string()));
    }
    #[test]//✅
    fn test_persona_to_string()                                                         
    {
        use crate::tp03::Ejercicio1::Persona;
        let sugus = Persona::new("Agus".to_string(), 90, Some("Maralrra".to_string()));
        let cadena : String = format!("{:?}",sugus);

        assert_eq!(sugus.to_string(), cadena);
    }
    #[test]//✅
    fn test_obtener_edad_persona()
    {
        use crate::tp03::Ejercicio1::Persona;
        let sugus = Persona::new("Agus".to_string(), 90, Some("Maralrra".to_string()));

        assert_eq!(sugus.edad,sugus.obtener_edad());
        assert_eq!(90,sugus.obtener_edad());
    }
    #[test]//✅
    fn test_actualizar_direccion_persona()
    {
        use crate::tp03::Ejercicio1::Persona;
        let mut sugus = Persona::new("Agus".to_string(), 90, Some("Maralrra".to_string()));
        let direccion_anterior = sugus.direccion.clone();

        sugus.actualizar_direccion("Villa".to_string());
        assert_ne!(sugus.direccion, direccion_anterior);
        assert_eq!(sugus.direccion.unwrap(),"Villa".to_string());
    }
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-1 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-2 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
    #[test]//✅                                   new, calcular_area, calcular_perimetro, es_cuadrado
    fn test_new_rectangulo()
    {
        use crate::tp03::Ejercicio2::Rectangulo;
        let forma = Rectangulo::new(10,5);

        assert_eq!(forma.longitud,10);
        assert_eq!(forma.ancho, 5);
    }
    #[test]//✅
    fn test_calcular_area_rectangulo()
    {
        use crate::tp03::Ejercicio2::Rectangulo;
        let forma = Rectangulo::new(10,5);

        assert_eq!(forma.calcular_area(),50);
    }
    #[test]//✅
    fn test_calcular_perimetro_rectangulo()
    {
        use crate::tp03::Ejercicio2::Rectangulo;
        let forma = Rectangulo::new(10,5);

        assert_eq!(forma.calcular_perimetro(),30);
    }
    #[test]//✅
    fn test_es_cuadrado_rectangulo()
    {
        use crate::tp03::Ejercicio2::Rectangulo;

        let forma: Rectangulo = Rectangulo::new(10, 5);
        let cuadril = Rectangulo::new(10,10);

        assert!(!forma.es_cuadrado());
        assert!(cuadril.es_cuadrado());
    }
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-2 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-3 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
    #[test]//✅                          new,es_fecha_valida,es_bisiesto,sumar_dias,restar_dias,es_mayor
    fn test_new_fecha() 
    {
        use crate::tp03::Ejercicio3::Fecha;
        let cumple: Fecha = Fecha::new(23,05,2002);

        assert_eq!(cumple.dia, 23);
        assert_eq!(cumple.mes, 05);
        assert_eq!(cumple.anio, 2002);
    }
    #[test]//✅
    fn test_es_fecha_valida_fecha()
    {
        use crate::tp03::Ejercicio3::Fecha;
        let cumple: Fecha = Fecha::new(23,05,2002); //true
        let cumple1: Fecha = Fecha::new(30,02,2000); //false
        let cumple2: Fecha = Fecha::new(31,03,2002); //true
        let cumple3: Fecha = Fecha::new(31,05,2002); //true
        let cumple4: Fecha = Fecha::new(31,06,2002); //false
        let cumple5: Fecha = Fecha::new(31,07,2002); //true
        let cumple6: Fecha = Fecha::new(31,08,2002); //true
        let fallecio: Fecha = Fecha::new(31,04,2024); //false
        
        assert!(cumple.es_fecha_valida());
        assert!(!cumple1.es_fecha_valida());
        assert!(cumple2.es_fecha_valida());
        assert!(cumple3.es_fecha_valida());
        assert!(!cumple4.es_fecha_valida());
        assert!(cumple5.es_fecha_valida());
        assert!(cumple6.es_fecha_valida());
        assert!(!fallecio.es_fecha_valida());
    }
    #[test]//✅
    fn test_es_bisiesto_fecha()
    {
        use crate::tp03::Ejercicio3::Fecha;
        let cumple: Fecha = Fecha::new(23,05,2002);
        let fenecio: Fecha = Fecha::new(60,08, 2424);

        assert!(!cumple.es_bisiesto()); // no naci en bisiesto
        assert!(fenecio.es_bisiesto());
    }
    #[test]//✅
    fn sumar_dias_fecha()                       //mostrar cosas en pantalla: cargo test -- --show-output
    {
        use crate::tp03::Ejercicio3::Fecha;
        let mut cumple: Fecha = Fecha::new(23,05,2002);
        let mut fenecio: Fecha = Fecha::new(29,02, 2024);

        cumple.sumar_dias(40);

        //println!("{}/{}/{}",fenecio.dia,fenecio.mes,fenecio.anio);

        fenecio.sumar_dias(366);

        assert_eq!(cumple.dia, 02);
        assert_eq!(cumple.mes, 07);
        assert_eq!(cumple.anio, 2002);

        assert_eq!(fenecio.dia, 1);
        assert_eq!(fenecio.mes, 3);
        assert_eq!(fenecio.anio, 2025);
    }
    #[test]//✅
    fn restar_dias_fecha()
    {
        use crate::tp03::Ejercicio3::Fecha;
        let mut cumple: Fecha = Fecha::new(23,05,2002);
        let mut fenecio: Fecha = Fecha::new(1,03, 2024);

        cumple.restar_dias(873);

        assert_eq!(cumple.dia, 01);
        assert_eq!(cumple.mes, 01);
        assert_eq!(cumple.anio, 2000);

        fenecio.restar_dias(912);

        assert_eq!(fenecio.dia, 1);
        assert_eq!(fenecio.mes, 9);
        assert_eq!(fenecio.anio, 2021);
    }
    #[test]//✅
    fn es_mayor_fecha()
    {
        use crate::tp03::Ejercicio3::Fecha;
        let cumple: Fecha = Fecha::new(23,05,2002);
        let fenecio: Fecha = Fecha::new(1,03, 2024);

        assert!(!cumple.es_mayor(&fenecio));
        assert!(fenecio.es_mayor(&cumple));
    }
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-3 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-4 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
    #[test]//✅                                     new,determinar_tipo,calcular_area,calcular_perimetro
    fn new_triangulo()
    {
        use crate::tp03::Ejercicio4::*;

        let tresangulos : Triangulo = Triangulo::new(30.0, 40.0, 30.0);
        let tresangulos1 : Triangulo = Triangulo::new(30.0, 40.0, 35.0);
        let tri : Triangulo = Triangulo::new(16.0, 16.0, 16.0);

        assert_eq!(tresangulos.lados.0, 30.0);
        assert_eq!(tresangulos.lados.1, 40.0);
        assert_eq!(tresangulos.lados.2, 30.0);
        assert!(!matches!(tresangulos.tipo, Some(Tipo::Equilatero)));

        assert_eq!(tresangulos1.lados.0, 30.0);
        assert_eq!(tresangulos1.lados.1, 40.0);
        assert_eq!(tresangulos1.lados.2, 35.0);
        assert!(!matches!(tresangulos1.tipo, None));
        assert!(matches!(tresangulos1.tipo.unwrap(), Tipo::Escaleno));

        assert_eq!(tri.lados.0, 16.0);
        assert_eq!(tri.lados.1, 16.0);
        assert_eq!(tri.lados.2, 16.0);
        assert!(!matches!(tri.tipo, Some(Tipo::Isosceles)));
        assert!(matches!(tri.tipo.unwrap(), Tipo::Equilatero));
    }
    #[test]//✅
    fn determinar_tipo_triangulo()
    {
        use crate::tp03::Ejercicio4::*;
        let mut tresangulos : Triangulo = Triangulo::new(30.0, 40.0, 30.0);
        let mut tresangulos1 : Triangulo = Triangulo::new(30.0, 40.0, 50.0);
        let mut tri : Triangulo = Triangulo::new(16.0, 16.0, 16.0);

        tresangulos.determinar_tipo();
        tresangulos1.determinar_tipo();
        tri.determinar_tipo();
        /* 
        assert_eq!(tresangulos.tipo, Some(Tipo::Isosceles));
        assert_eq!(tresangulos1.tipo, Some(Tipo::Escaleno));
        assert_eq!(tri.tipo, Some(Tipo::Equilatero));

        assert_ne!(tresangulos.tipo, None);
        assert_ne!(tresangulos1.tipo, Some(Tipo::Isosceles));
        assert_ne!(tri.tipo, Some(Tipo::Escaleno));*/
    }
    #[test]//✅
    fn calcular_area_triangulo()
    {
        use crate::tp03::Ejercicio4::*;
        let tresangulos : Triangulo = Triangulo::new(30.0, 40.0, 30.0);   //isosceles
        let tresangulos1 : Triangulo = Triangulo::new(30.0, 40.0, 50.0);  //escaleno
        let tri : Triangulo = Triangulo::new(16.0, 16.0, 16.0);           //equilatero

        println!("{}", tresangulos.calcular_area());
                                                            //Si después del 5 ponés un 8 o 9 lo trunca igual a 2136 y si directamente pones 2136 assert_eq(true)
        assert_eq!(tresangulos.calcular_area(),447.21358); //447.21359_EL RESTO_549995793 precisión de 5 digitos después del 0
        assert_eq!(tresangulos1.calcular_area(),600.0); //600 clavado
        assert_eq!(tri.calcular_area(),110.85125);    //110.85125168440814

        assert_ne!(tresangulos.calcular_area(),0.0);
        assert_ne!(tresangulos1.calcular_area(),0.0);
        assert_ne!(tri.calcular_area(),0.0);
    }
    #[test]//✅
    fn calcular_perimetro_triangulo()
    {
        use crate::tp03::Ejercicio4::*;
        let tresangulos : Triangulo = Triangulo::new(30.0, 40.0, 30.0);   //isosceles
        let tresangulos1 : Triangulo = Triangulo::new(30.0, 40.0, 50.0);  //escaleno
        let tri : Triangulo = Triangulo::new(16.0, 16.0, 16.0);           //equilatero

        assert_eq!(tresangulos.calcular_perimetro(),100.0);
        assert_eq!(tresangulos1.calcular_perimetro(),120.0);
        assert_eq!(tri.calcular_perimetro(),48.0);

        assert_ne!(tresangulos.calcular_perimetro(),100.00001);
        assert_ne!(tresangulos1.calcular_perimetro(),120.00001);
        assert_ne!(tri.calcular_perimetro(),48.00001);
    }
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-4 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-5 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
    #[test]
    fn new_producto() 
    {
        use crate::tp03::Ejercicio5::*;

        let prod = Producto::new("cubo rubik".to_string(), 10000.0, 1);

        assert_eq!(prod.calcular_impuestos(50.0), 5000.0);
        assert_eq!(prod.aplicar_descuento(50.0), 5000.0);
        assert_eq!(prod.calcular_precio_total(Some(100.0), Some(50.0)), 15000.0);
    }

//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-5 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
//=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>=>EJERCICIO-6 TEST<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=<=    
    
    #[test]
    fn estudiante_test() 
    {
        use crate::tp03::Ejercicio6::Estudiante;
        
        let mut alu : Estudiante = Estudiante::new("Carlos".to_string(), 40002);
        let parcial1 : Examen = Examen::new("Matemáticas".to_string(), 6.7);
        let parcial2 : Examen = Examen::new("Geografia".to_string(), 4.5);
        let parcial3 : Examen = Examen::new("Historia".to_string(), 2.7);

        alu.calif.push(parcial1);
        alu.calif.push(parcial2);
        alu.calif.push(parcial3);

        let prom : f32 = (6.7+4.5+2.7) / 3.0;
        assert_eq!(alu.obtener_calificacion_mas_alta(),6.7);
        assert_eq!(alu.obtener_calificacion_mas_baja(),2.7);
        assert_eq!(alu.obtener_promedio(),prom);
        assert_eq!(alu.calif[0].materia,"Matemáticas".to_string());
        assert_eq!(alu.calif[0].nota,6.7);
    }

    // el 7
    #[test]
    fn test_concesionario_auto() 
    {
        use crate::tp03::Ejercicio7::{Auto, Color, ConcesionarioAuto};
        // Crear un concesionario
        let mut concesionario: ConcesionarioAuto = ConcesionarioAuto::new(
            "AutoMax".to_string(),
            "Av. Libertad 123".to_string(),
            3,
        );
        assert_eq!(concesionario.nombre, "AutoMax", "El nombre del concesionario no coincide");
        assert_eq!(concesionario.autos.len(), 0, "El concesionario debería estar vacío al inicio");

        // Crear autos
        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::ROJO,
        );
        let auto2 = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            1998,
            30000.0,
            Color::AZUL,
        );
        let auto3 = Auto::new(
            "Ford".to_string(),
            "Focus".to_string(),
            2020,
            25000.0,
            Color::BLANCO,
        );
        let auto4 = Auto::new(
            "Honda".to_string(),
            "Civic".to_string(),
            2018,
            22000.0,
            Color::NEGRO,
        );

        // Probar agregar autos
        assert!(concesionario.agregar_auto(&auto1), "No se pudo agregar el primer auto");
        assert_eq!(concesionario.autos.len(), 1, "Debería haber un auto en el concesionario");
        assert!(concesionario.agregar_auto(&auto2), "No se pudo agregar el segundo auto");
        assert_eq!(concesionario.autos.len(), 2, "Debería haber dos autos en el concesionario");
        assert!(concesionario.agregar_auto(&auto3), "No se pudo agregar el tercer auto");
        assert_eq!(concesionario.autos.len(), 3, "Debería haber tres autos en el concesionario");
        assert!(!concesionario.agregar_auto(&auto4), "No debería permitir agregar más autos por capacidad máxima");
        assert_eq!(concesionario.autos.len(), 3, "El número de autos no debería cambiar");

        // Probar buscar auto
        let encontrado = concesionario.buscar_auto(&auto1);
        assert!(encontrado.is_some(), "Debería encontrar el auto1");
        assert!(
            encontrado.unwrap().comparar_autos(&auto1),
            "El auto encontrado no coincide con auto1"
        );
        let no_encontrado = concesionario.buscar_auto(&auto4);
        assert!(no_encontrado.is_none(), "No debería encontrar el auto4");

        // Probar eliminar auto
        concesionario.eliminar_auto(&auto2);
        assert_eq!(concesionario.autos.len(), 2, "Debería haber dos autos después de eliminar");
        let no_encontrado = concesionario.buscar_auto(&auto2);
        assert!(no_encontrado.is_none(), "El auto2 no debería estar en el concesionario");

        // Probar comparación de autos
        let auto1_clon = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::ROJO,
        );
        assert!(
            auto1.comparar_autos(&auto1_clon),
            "Los autos idénticos deberían ser considerados iguales"
        );
        assert!(
            !auto1.comparar_autos(&auto3),
            "Autos diferentes no deberían ser considerados iguales"
        );

        // Probar color_string
        assert_eq!(auto1.color_string(), "ROJO", "El color ROJO no se convirtió correctamente");
        assert_eq!(auto3.color_string(), "BLANCO", "El color BLANCO no se convirtió correctamente");

        // Probar calcular_precio
        // Auto1: Toyota, 2015, ROJO -> +25% = 20000 * 1.25 = 25000
        assert_eq!(
            Auto::calcular_precio(&auto1),
            25000.0,
            "El precio de auto1 no se calculó correctamente"
        );
        // Auto2: BMW, 1998, AZUL -> +25% (color) + 15% (BMW}
    }
    // el 8
    #[test]
    fn playlists_test()
    {
        use crate::tp03::Ejercicio8::*;

        // Crear una playlist
        let mut playlist = Playlist::new("Mis Favoritas".to_string());
        assert_eq!(playlist.nombre, "Mis Favoritas", "El nombre de la playlist no coincide");
        assert_eq!(playlist.lista.len(), 0, "La playlist debería estar vacía al crearse");
    
        // Crear canciones
        let cancion1 = Cancion {
            titulo: "Bohemian Rhapsody".to_string(),
            artista: "Queen".to_string(),
            genero: Genero::ROCK,
        };
        let cancion2 = Cancion {
            titulo: "Billie Jean".to_string(),
            artista: "Michael Jackson".to_string(),
            genero: Genero::POP,
        };
        let cancion3 = Cancion {
            titulo: "Lose Yourself".to_string(),
            artista: "Eminem".to_string(),
            genero: Genero::RAP,
        };
    
        // Agregar canciones
        playlist.agregar_cancion(cancion1.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3.clone());
        assert_eq!(playlist.lista.len(), 3, "La playlist debería tener tres canciones");
        assert!(playlist.lista[0].comparar_cancion(&cancion1), "La primera canción no coincide");
        assert!(playlist.lista[1].comparar_cancion(&cancion2), "La segunda canción no coincide");
        assert!(playlist.lista[2].comparar_cancion(&cancion3), "La tercera canción no coincide");
    
        // Mover canción
        playlist.mover_cancion(0, 2);
        assert!(playlist.lista[2].comparar_cancion(&cancion1), "La canción no se movió a la posición 2");
        assert!(playlist.lista[0].comparar_cancion(&cancion3), "La canción no se movió a la posición 0");
    
        // Buscar canción por nombre
        let resultado_busqueda = playlist.buscar_cancion_nombre("Billie Jean".to_string());
        assert!(resultado_busqueda.is_some(), "Debería encontrar la canción");
        assert!(resultado_busqueda.unwrap().comparar_cancion(&cancion2), "La canción encontrada no coincide");
    
        // Obtener canciones por género
        let canciones_rock = playlist.playlist_genero(&Genero::ROCK);
        assert_eq!(canciones_rock.len(), 1, "Debería haber una canción de género ROCK");
        assert!(canciones_rock[0].comparar_cancion(&cancion1), "La canción de ROCK no coincide");
    
        // Obtener canciones por artista
        let canciones_eminem = playlist.playlist_artista("Eminem".to_string());
        assert_eq!(canciones_eminem.len(), 1, "Debería haber una canción de Eminem");
        assert!(canciones_eminem[0].comparar_cancion(&cancion3), "La canción de Eminem no coincide");
    
        // Eliminar una canción
        playlist.eliminar_cancion(cancion2.clone());
        assert_eq!(playlist.lista.len(), 2, "La playlist debería tener dos canciones después de eliminar");
        assert!(playlist.buscar_cancion_nombre("Billie Jean".to_string()).is_none(), "La canción eliminada aún está en la playlist");
    
        // Modificar título de la playlist (asumiendo un método simple)
        playlist.nombre = "Nuevas Vibras".to_string();
        assert_eq!(playlist.nombre, "Nuevas Vibras", "El título de la playlist no se modificó correctamente");
    
        // Eliminar todas las canciones
        playlist.borrar_lista();
        assert_eq!(playlist.lista.len(), 0, "La playlist no se vació correctamente");
    }

    
    // el 9
    #[test]
    fn veterinaria()
    {
        use crate::tp03::Ejercicio9::*;
        use crate::tp03::Ejercicio3::*;

        let mut vete = Veterinaria::new("una vete".to_string(), "coso".to_string(), 1);
        let duenio = Duenio::new("jorge".to_string(), "coso1".to_string(), 123456);
        let mascotita = Mascota::new("mascotita".to_string(), 2, Animal::Perro, &duenio);
        vete.add_mascota(&mascotita);  


        // Verificar que la mascota se agregó correctamente
        assert_eq!(vete.fila.len(), 1, "La cola debería tener una mascota");
        assert!(vete.fila[0].comparar_mascota(&mascotita), "La mascota en la cola no coincide");

        // Agregar una mascota con máxima prioridad
        let mascotita2 = Mascota::new("luna".to_string(), 3, Animal::Gato, &duenio);
        vete.add_mascota_importante(&mascotita2);
        assert_eq!(vete.fila.len(), 2, "La cola debería tener dos mascotas");
        assert!(vete.fila[0].comparar_mascota(&mascotita2), "La mascota prioritaria debería estar al frente");

        // Atender la próxima mascota
        vete.atender_next_mascota();
        assert_eq!(vete.fila.len(), 1, "La cola debería tener una mascota después de atender");
        assert!(vete.fila[0].comparar_mascota(&mascotita), "La mascota restante debería ser mascotita");

        // Registrar una atención
        let fecha = Fecha::new(14, 5, 2025);
        let atencion = Atencion::new(
            mascotita.clone(),
            "Fiebre".to_string(),
            "Antibióticos".to_string(),
            fecha.clone(),
        );
        vete.registrar_atencion(&atencion);
        assert_eq!(vete.atendidos.len(), 1, "Debería haber una atención registrada");
        assert!(vete.atendidos[0].comparar_atencion(&atencion), "La atención registrada no coincide");

        // Buscar una atención
        let resultado_busqueda = vete.buscar_atencion(
            "mascotita".to_string(),
            "jorge".to_string(),
            123456,
        );
        assert!(resultado_busqueda.is_some(), "Debería encontrar la mascota");
        assert!(resultado_busqueda.unwrap().comparar_mascota(&mascotita), "La mascota encontrada no coincide");

        // Modificar el diagnóstico de una atención
        vete.modificar_diagnostico_atencion("Infección".to_string(), &atencion);
        assert_eq!(vete.atendidos[0].diagnostico, "Infección", "El diagnóstico no se modificó correctamente");


        let atencion = Atencion::new(
            mascotita.clone(),
            "Infección".to_string(),
            "Antibióticos".to_string(),
            fecha.clone(),
        );
        // Modificar la fecha de la próxima visita
        let nueva_fecha = Fecha::new(1, 6, 2025);
        vete.modificar_fecha_prox_visita(&nueva_fecha, &atencion);
        assert!(vete.atendidos[0].fecha.comparar_fecha(&nueva_fecha), "La fecha no se modificó correctamente");

        let fecha = Fecha::new(1, 6, 2025);
        let atencion = Atencion::new(
            mascotita.clone(),
            "Infección".to_string(),
            "Antibióticos".to_string(),
            fecha.clone(),
        );
        // Eliminar una atención
        vete.eliminar_atencion(&atencion);
        assert_eq!(vete.atendidos.len(), 0, "La atención no se eliminó correctamente");

        // Eliminar una mascota específica de la cola
        vete.eliminar_mascota(&mascotita);
        assert_eq!(vete.fila.len(), 0, "La mascota no se eliminó de la cola");      
    }

    // el 10
    #[test]
    fn test_biblioteca() {
        use crate::tp03::Ejercicio10::*;
        use crate::tp03::Ejercicio3::Fecha;
        // Crear una biblioteca
        let mut biblioteca = Biblioteca::new(
            "Biblioteca Central".to_string(),
            "Calle del Saber 123".to_string(),
        );
        assert_eq!(biblioteca.nombre, "Biblioteca Central", "El nombre de la biblioteca no coincide");
        assert_eq!(biblioteca.copias.len(), 0, "La lista de copias debería estar vacía");
        assert_eq!(biblioteca.prestamos.len(), 0, "La lista de préstamos debería estar vacía");

        // Crear un cliente
        let cliente = Cliente {
            nombre: "Juan Pérez".to_string(),
            telefono: 123456789,    
            email: "juan@example.com".to_string(),
        };

        // Crear un libro
        let libro = Libro::new(
            123456,
            "Cien Años de Soledad".to_string(),
            "Gabriel García Márquez".to_string(),
            400,
            Genero::Novela,
        );

        // Agregar copias del libro
        biblioteca.copias.push((libro.clone(), 3));
        assert_eq!(
            biblioteca.copias_disponibles(&libro),
            3,
            "Debería haber 3 copias disponibles"
        );

        // Probar restar una copia
        assert!(biblioteca.restar_una_copia(&libro), "No se pudo restar una copia");
        assert_eq!(
            biblioteca.copias_disponibles(&libro),
            2,
            "Debería haber 2 copias disponibles después de restar"
        );
        assert!(!biblioteca.restar_una_copia(&Libro::new(
            999999,
            "Libro Inexistente".to_string(),
            "Autor".to_string(),
            100,
            Genero::Otros
        )), "No debería restar copia de un libro inexistente");

        // Probar sumar una copia
        biblioteca.sumar_una_copia(&libro);
        assert_eq!(
            biblioteca.copias_disponibles(&libro),
            3,
            "Debería haber 3 copias disponibles después de sumar"
        );

        // Crear un préstamo
        let fecha_vencimiento = Fecha::new(20, 5, 2025);
        let fecha_devuelto = Fecha::new(1, 1, 1970); // Fecha inicial irrelevante
        let prestamo = Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            fecha_devuelto,
            fecha_vencimiento,
            estado: Estado::Prestado,
        };
        biblioteca.prestamos.push(prestamo.clone());

        // Probar contar préstamos de un cliente
        assert_eq!(
            biblioteca.prestamos_a_devolver_cliente(&cliente),
            1,
            "El cliente debería tener 1 préstamo activo"
        );

        // Probar realizar un préstamo
        assert!(biblioteca.restar_una_copia(&libro), "No se pudo realizar el préstamo inicial");
        assert_eq!(
            biblioteca.copias_disponibles(&libro),
            2,
            "Debería haber 2 copias disponibles después del préstamo"
        );

        // Probar buscar préstamo
        let encontrado = biblioteca.buscar_prestamito(&libro, &cliente);
        assert!(encontrado.is_some(), "Debería encontrar el préstamo");
        assert!(
            encontrado.unwrap().libro.comparar_libros(&libro),
            "El libro del préstamo encontrado no coincide"
        );
        let encontrado = biblioteca.buscar_prestamito(&libro, &cliente);
        assert!(
            encontrado.unwrap().comparar_cliente(&cliente),
            "El cliente del préstamo encontrado no coincide"
        );
        let no_encontrado = biblioteca.buscar_prestamito(
            &Libro::new(999999, "Otro".to_string(), "Autor".to_string(), 100, Genero::Otros),
            &cliente,
        );
        assert!(no_encontrado.is_none(), "No debería encontrar un préstamo inexistente");

        // Probar devolver libro
        biblioteca.devolver_libro(&libro, &cliente);
        let prestamo_devuelto = biblioteca.buscar_prestamito(&libro, &cliente).unwrap();
        assert_eq!(
            prestamo_devuelto.est_str(),
            "Devuelto",
            "El estado del préstamo debería ser Devuelto"
        );
        assert_eq!(
            biblioteca.copias_disponibles(&libro),
            3,
            "Debería haber 3 copias disponibles después de la devolución"
        );
        assert_eq!(
            biblioteca.prestamos_a_devolver_cliente(&cliente),
            0,
            "El cliente no debería tener préstamos activos después de la devolución"
        );

        // Probar préstamos a vencer
        let prestamos_a_vencer = biblioteca.prestamos_a_vencer(4);
        assert_eq!(
            prestamos_a_vencer.len(),
            1,
            "Debería haber 1 préstamo a vencer en los próximos 4 días"
        );

        // Probar préstamos vencidos
        let prestamos_vencidos = biblioteca.prestamos_vencidos();
        assert_eq!(
            prestamos_vencidos.len(),
            1,
            "Debería haber 1 préstamo vencido"
        );

        // Probar comparación de libros
        let libro_clon = Libro::new(
            123456,
            "Cien Años de Soledad".to_string(),
            "Gabriel García Márquez".to_string(),
            400,
            Genero::Novela,
        );
        assert!(
            libro.comparar_libros(&libro_clon),
            "Los libros idénticos deberían ser considerados iguales"
        );
        assert!(
            !libro.comparar_libros(&Libro::new(
                999999,
                "Otro".to_string(),
                "Autor".to_string(),
                100,
                Genero::Otros
            )),
            "Libros diferentes no deberían ser considerados iguales"
        );

        // Probar gen_str
        assert_eq!(libro.gen_str(), "Novela", "El género Novela no se convirtió correctamente");

        // Probar est_str
        assert_eq!(prestamo.est_str(), "Prestado", "El estado Prestado no se convirtió correctamente");
    }
}