use crate::tp03::Ejercicio3::Fecha;
use std::collections::VecDeque;
#[derive(Clone, Debug)]
pub struct Duenio
{
    pub nombre : String,
    pub direccion : String,
    pub telefono : u32
}
#[derive(Clone, Debug)]
pub enum Animal
{
    Perro,
    Gato,
    Caballo,
    Otros
}
#[derive(Clone, Debug)]
pub struct Mascota
{
    pub nombre : String,
    pub edad : u8,
    pub animal : Animal,
    pub duenio : Duenio
}
#[derive(Clone, Debug)]
pub struct Veterinaria
{
    pub nombre : String,
    pub direccion : String,
    pub id : u32,
    pub fila : VecDeque<Mascota>,
    pub atendidos : Vec<Atencion>
}
#[derive(Clone, Debug)]
pub struct Atencion 
{
    pub mascota : Mascota,
    pub diagnostico : String,
    pub tratamiento : String,
    pub fecha : Fecha
}

impl Atencion
{
    pub fn new(mascota : Mascota, diagnostico : String, tratamiento : String, fecha:Fecha) -> Atencion
    {
        Atencion 
        {
            mascota,
            diagnostico,
            tratamiento,
            fecha,
        }
    }

    pub fn comparar_atencion(&self, ate : &Atencion) -> bool
    {
        ate.mascota.comparar_mascota(&self.mascota) && ate.tratamiento == self.tratamiento && ate.diagnostico == self.diagnostico && ate.fecha.comparar_fecha(&self.fecha)
    }

}

impl Duenio
{
    pub fn new(nombre : String, direccion : String, telefono : u32) -> Duenio
    {
        Duenio
        {
            nombre,
            direccion,
            telefono,
        }
    }
}

impl Mascota
{
    pub fn new(nombre : String, edad : u8, animal : Animal, duenio : &Duenio) -> Mascota
    {
        Mascota
        {
            nombre,
            edad,
            animal,
            duenio: duenio.clone()
        }   
    }
    
    pub fn animal_strng(&self) -> String  //(perro, gato, caballo, otros)
    {
        match self.animal
        {
            Animal::Perro => String::from("Perro"),
            Animal::Gato => String::from("Gato"),
            Animal::Caballo => String::from("Caballo"),
            Animal::Otros => String::from("Otros"),
        }
    }

    pub fn comparar_duenio(&self, masc : &Mascota) -> bool //nombre direccion telef
    {
        (self.duenio.nombre == masc.duenio.nombre)&&(self.duenio.direccion == masc.duenio.direccion)&&(self.duenio.telefono == masc.duenio.telefono)
    }

    pub fn comparar_mascota(&self, masc : &Mascota) -> bool
    {
        (self.nombre == masc.nombre)&&(self.edad == masc.edad)&&(self.animal_strng() == masc.animal_strng())&&(self.comparar_duenio(masc))
    }   
}

impl Veterinaria
{
    pub fn new(nombre : String, direccion : String, id : u32) -> Veterinaria 
    {
        Veterinaria
        {
            nombre,
            direccion,
            id,
            fila: VecDeque::new(),
            atendidos: Vec::new()
        }
    }

    pub fn add_mascota(&mut self, masc : &Mascota)
    {
        self.fila.push_back(masc.clone());
    }

    pub fn add_mascota_importante(&mut self, masc : &Mascota)
    {
        self.fila.push_front(masc.clone());
    }

    pub fn atender_next_mascota(&mut self)
    {
        self.fila.pop_front();
    }

    pub fn eliminar_mascota(&mut self, masc : &Mascota)
    {
        for i in 0..self.fila.len()
        {
            if self.fila[i].comparar_mascota(masc)
            {
                self.fila.remove(i);
                return
            }
        }
    }

    pub fn registrar_atencion(&mut self, ate: &Atencion)
    {
        self.atendidos.push(ate.clone());
    }

    //buscar una atención dado el nombre de la mascota, el nombre del dueño y el teléfono. 

    pub fn buscar_atencion(&self, nom_mascota : String, nom_dueño : String, telefono: u32) -> Option<Mascota>
    {
        for i in 0..self.fila.len() 
        {
            if (self.fila[i].nombre == nom_mascota) && (self.fila[i].duenio.nombre == nom_dueño) && (self.fila[i].duenio.telefono == telefono)
            {
                return Some(self.fila[i].clone())
            }
        }
        None
    }

    pub fn modificar_diagnostico_atencion(&mut self, diag : String, ate : &Atencion)
    {
        for a in &mut self.atendidos
        {
            if a.comparar_atencion(&ate)
            {
                a.diagnostico = diag;
                return
            }
        }  
    }
    
   pub fn modificar_fecha_prox_visita(&mut self, fecha : &Fecha, ate : &Atencion)
    {
        for a in &mut self.atendidos
        {
            if a.comparar_atencion(&ate)
            {
                a.fecha = fecha.clone();
                return
            }
        }
    } 

    pub fn eliminar_atencion(&mut self, ate : &Atencion)
    {
        for i in 0..self.atendidos.len()
        {
            if self.atendidos[i].comparar_atencion(ate)
            {
                self.atendidos.remove(i);
                return
            }
        }
    }
}

/* 9.-
Dada una cadena de veterinarias se desea implementar un sistema de atención de 
pacientes para cada veterinaria, 
de la veterinaria se conoce el nombre, la dirección y un id. 
Para la atención de mascotas se requiere administrar una cola de atención. 
De la mascota se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. 
Del  dueño se conoce el nombre, la dirección y un teléfono de contacto. 
Luego de la atención se desea tener un registro de las atenciones realizadas 
guardando los datos de la mascota, el diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere. 
Dado todo lo mencionado anteriormente implemente los métodos para realizar las 
siguientes acciones: 
➔ crear una veterinaria. 
➔ agregar una nueva mascota a la cola de atención de la veterinaria. 
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente 
en atender porque tiene la máxima prioridad. 
➔ atender la próxima mascota de la cola. 
➔ eliminar una mascota específica de la cola de atención dado que se retira. 
➔ registrar una atención. 
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el 
teléfono. 
➔ modificar el diagnóstico de una determinada atención. 
➔ modificar la fecha de la próxima visita de una determinada atención. 
➔ eliminar una determinada atención.  */