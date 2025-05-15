#[derive(Clone, Debug)]
pub enum Estado //Consejo del más allá Método Buscar Prestamo
{               //
    Devuelto,
    Prestado
}
#[derive(Clone, Debug)]
pub struct Biblioteca
{
    pub nombre : String,
    pub direccion : String,
    pub copias : Vec<(Libro,u32)>,
    pub prestamos : Vec<Prestamo>,
}
#[derive(Clone, Debug)]

pub enum Genero
{
    Novela,
    Infantil,
    Tecnicos,
    Otros
}
#[derive(Clone, Debug)]

pub struct Libro
{
    pub isbn : u32,
    pub titulo : String,
    pub autor : String,
    pub cant_paginas : u32,
    pub genero : Genero
}

use chrono::{Date, DateTime, Datelike, Duration, NaiveDate, Utc, Local};

use crate::tp03::Ejercicio3::Fecha;
#[derive(Clone, Debug)]

pub struct Prestamo
{
    pub libro : Libro,
    pub cliente : Cliente,
    pub fecha_devuelto : Fecha,
    pub fecha_vencimiento : Fecha,
    pub estado : Estado,
}
#[derive(Clone, Debug)]

pub struct Cliente
{
    pub nombre : String,
    pub telefono : u32,
    pub email : String
}

impl Prestamo
{
    pub fn comparar_cliente(&self, cliente : &Cliente) -> bool
    {
        (self.cliente.nombre == cliente.nombre)&&(self.cliente.telefono == cliente.telefono)&&(self.cliente.email == cliente.email)
    }

    pub fn est_str(&self) -> String
    {
        match self.estado
        {
            Estado::Devuelto => String::from("Devuelto"),
            Estado::Prestado => String::from("Prestado"),
        }
    }
}

impl Libro
{
    pub fn new(isbn : u32, titulo : String, autor : String, cant_paginas : u32, genero : Genero) -> Libro
    {
        Libro
        {
            isbn,
            titulo,
            autor,
            cant_paginas,
            genero
        }
    }
    
    pub fn gen_str(&self) -> String
    {
        match self.genero
        {
            Genero::Novela => String::from("Novela"),
            Genero::Infantil => String::from("Infantil"),
            Genero::Tecnicos => String::from("Tecnicos"),
            Genero::Otros => String::from("Otros"),
        }
    }

    pub fn comparar_libros(&self, lib : &Libro) ->bool
    {
        (self.isbn == lib.isbn)&&(self.titulo == lib.titulo)&&(self.autor == lib.autor)&&(self.cant_paginas == lib.cant_paginas)&&(self.gen_str() == lib.gen_str())
    }
}

impl Biblioteca
{
    pub fn new(nombre : String, direccion : String) -> Biblioteca
    {
        Biblioteca 
        {
            nombre,
            direccion,
            copias: Vec::new(),
            prestamos: Vec::new()
        }
    }
    
    pub fn copias_disponibles(&self, libro : &Libro) -> u32
    {
        for i in 0..self.copias.len()
        {
            if self.copias[i].0.comparar_libros(&libro)
            {
                return self.copias[i].1
            }
        }
        return 0
    }

    //➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1 la cantidad de copias de libros a disposición para prestar. 

    pub fn restar_una_copia(&mut self, lib : &Libro) -> bool
    {
        for i in 0..self.copias.len()
        {
            if self.copias[i].0.comparar_libros(&lib) //encontró el libro
            {
                if self.copias[i].1 > 0 
                {
                    self.copias[i].1 = self.copias[i].1 - 1;
                    return true
                }//else panic cantidad de libros = 0
            }
        }
        false
    }    

//➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1 la cantidad de copias del libro a disposición para ser prestado.

    pub fn sumar_una_copia(&mut self, lib : &Libro)
    {
        for i in 0..self.copias.len()
        {
            if self.copias[i].0.comparar_libros(&lib) //encontró el libro
            {
                self.copias[i].1 = self.copias[i].1 + 1;
                return
            }
        }
        //panic libro no encontrado
    }

//➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado “en préstamo” de un determinado cliente. 

    pub fn prestamos_a_devolver_cliente(&self, cliente : &Cliente) -> u32 
    {
        let mut contador :u32 = 0;
        for prestamos in &self.prestamos
        {
            if prestamos.comparar_cliente(&cliente)
            {
                if prestamos.est_str() == "Prestado"
                {
                    contador += 1;
                }
            }
        }
        contador
    }

/*➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro 
para un determinado cliente cumpliendo con lo siguiente
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo” 
◆  haya al menos una copia disponible en el registro de copias a 
disposición. 
De ser así descuenta 1 en el registro de “copias a disposición” y 
retorna true, si no cumple con alguna de las condiciones retorna false.    */

    pub fn realizar_prestamo(&mut self, cliente : &Cliente, libro: &Libro) ->bool
    {
        if (self.prestamos_a_devolver_cliente(&cliente) > 5)&&(self.copias_disponibles(&libro) > 0)
        {
            return self.restar_una_copia(&libro)
        }
        else
        {
            return false
        }
    }

/*➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a 
vencer en los próximos días, el valor de días es pasado por parámetro.  */

    pub fn prestamos_a_vencer(&self, dias : u32) -> Vec<Prestamo>
    {
        let mut prestamos  : Vec<Prestamo> = Vec::new();
        
        let now = Local::now().date_naive();
        let mut fecha_actual : Fecha = Fecha::new(now.day(), now.month(), now.year() as u32);
        
        fecha_actual.sumar_dias(dias);

        for prestado in &self.prestamos  
        {
            if  prestado.fecha_vencimiento.es_mayor(&fecha_actual) 
            {
                prestamos.push(prestado.clone());
            }
        }
        prestamos
    }
    
/*➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado“en 
préstamos” donde la fecha de vencimiento es menor a la fecha actual. */

    pub fn prestamos_vencidos(&self) -> Vec<Prestamo>
    {
        let mut vencido  : Vec<Prestamo> = Vec::new();
        
        let now = Local::now().date_naive();
        let mut fecha_actual : Fecha = Fecha::new(now.day(), now.month(), now.year() as u32);

        for prestado in &self.prestamos  
        {
            if  prestado.fecha_vencimiento.es_mayor(&fecha_actual) 
            {
                vencido.push(prestado.clone());
            }
        }
        vencido
    }

    /*
    ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si 
    existe. 
    */
    pub fn buscar_prestamito(&self, librito : &Libro, cliente : &Cliente)  -> Option<Prestamo>
    {
        for prestado in &self.prestamos
        {
            if librito.comparar_libros(&prestado.libro)&&prestado.comparar_cliente(&cliente)
            {
                return Some(prestado.clone());
            }
        }
        return None
    }

    /*
    ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al 
    estado “devuelto”, se registra la fecha de devolución y se incrementa la 
    cantidad de libros en 1 del libro devuelto en el registro de copias a 
    disposición. 
    */
    pub fn devolver_libro(&mut self, librito : &Libro, cliente : &Cliente)
    {
        let mut cambiado : bool = false;
        for prestado in &mut self.prestamos
        {
            if prestado.libro.comparar_libros(&librito)&&prestado.comparar_cliente(&cliente)&&(prestado.est_str() == "Prestado")
            {
                prestado.estado = Estado::Devuelto;
                let now: NaiveDate = Local::now().date_naive();
                let fecha_actual : Fecha = Fecha::new(now.day(), now.month(), now.year() as u32);
                prestado.fecha_devuelto = fecha_actual;

                cambiado = true;
            }
        }
        if cambiado
        {
            self.sumar_una_copia(&librito);
        }
    }
}

/*10-Para una biblioteca se desea implementar un sistema de préstamos de libros. 

De la biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para 
prestar y los préstamos efectuados.

Los libros a disposición es un registro donde se indica la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. 

De cada libro se conoce el isbn, el título, autor, número de páginas, género(novela, infantil, 
técnico, otros). 

Para registrar un préstamo se requiere el libro, el cliente, la fecha de 
vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en 
préstamo. 
Del cliente se conoce el nombre, teléfono y dirección de correo electrónico. 
Implemente los métodos necesarios para realizar las siguientes acciones: ✅

➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de 
copias a disposición que hay para prestar de dicho libro. ✅

➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1 
la cantidad de copias de libros a disposición para prestar. ✅

➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1 
la cantidad de copias del libro a disposición para ser prestado. ✅

➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado 
“en préstamo” de un determinado cliente. ✅

➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro 
para un determinado cliente cumpliendo con lo siguiente  
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo” 
◆  haya al menos una copia disponible en el registro de copias a 
disposición. 
De ser así descuenta 1 en el registro de “copias a disposición” y 
retorna true, si no cumple con alguna de las condiciones retorna false. ✅


➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a 
vencer el los próximos días, el valor de días es pasado por parámetro.  ✅

➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado“en 
préstamos” donde la fecha de vencimiento es menor a la fecha actual. ✅

➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si 
existe. ✅

➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al 
estado “devuelto”, se registra la fecha de devolución y se incrementa la 
cantidad de libros en 1 del libro devuelto en el registro de copias a 
disposición.  ✅*/ 