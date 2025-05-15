#[derive(Clone, Debug)]
pub enum Genero
{
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS
}
#[derive(Clone, Debug)]

pub struct Cancion
{
    pub titulo : String,
    pub artista: String,
    pub genero: Genero
}
#[derive(Clone, Debug)]
pub struct Playlist
{
    pub nombre : String,
    pub lista : Vec<Cancion>
}
impl Genero
{
    pub fn g_str(&self) -> String
    {
        match self
        {
            Genero::ROCK => String::from("ROCK"),
            Genero::POP => String::from("POP"),
            Genero::RAP => String::from("RAP"),
            Genero::JAZZ => String::from("JAZZ"),
            Genero::OTROS => String::from("OTROS")
        }
    }
}
impl Cancion
{
    pub fn comparar_cancion(&self, song : &Cancion) -> bool
    {
        (self.titulo == song.titulo) && (self.artista == song.artista) && (self.genero.g_str() == song.genero.g_str())
    }
}
impl Playlist               //sin news
{
    pub fn new(nombre : String) -> Playlist
    {
        Playlist 
        { 
            nombre,
            lista : Vec::new()
        }
    }
    pub fn agregar_cancion(&mut self, song : Cancion)
    {
        self.lista.push(song);
    }
    pub fn eliminar_cancion(&mut self, song : Cancion)
    {
        for i in 0..self.lista.len()
        {
            if self.lista[i].comparar_cancion(&song)
            {
                self.lista.swap_remove(i);
                return
            }
        }
    }
    pub fn mover_cancion(&mut self, pos1 : u32, pos2 : u32)
    {
        self.lista.swap(pos1 as usize,pos2 as usize);
    }
    pub fn buscar_cancion_nombre(&self, titulo : String) -> Option<Cancion>
    {
        for cancion in &self.lista
        {
            if cancion.titulo == titulo
            {
                return Some(cancion.clone());
            }
        }
        None
    }
    pub fn playlist_genero(&self, gene : &Genero) -> Vec<Cancion>
    {
        let mut gen_play : Vec<Cancion> = Vec::new();
        for cancion in &self.lista
        {
            if cancion.genero.g_str() == gene.g_str()
            {
                gen_play.push(cancion.clone());
            }
        }
        gen_play
    }
    pub fn playlist_artista(&self, art : String) -> Vec<Cancion>
    {
        let mut art_play : Vec<Cancion> = Vec::new();
        for cancion in &self.lista
        {
            if cancion.artista == art
            {
                art_play.push(cancion.clone());
            }
        }
        art_play
    }
    pub fn borrar_lista(&mut self)
    {
        self.lista.clear();
    }
}
/*8- Defina la estructura Cancion con campos para el título, el artista y el género. El género 
puede ser rock, pop, rap,  jazz, otros. Luego modele una playlist. La playlist está compuesta 
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre 
ella: 
➔ agregar canción. 
➔ eliminar canción. 
➔ mover canción // mueve la canción a una determinada posición de la playlist. 
➔ buscar canción por nombre. 
➔ obtener las canciones de un determinado género. 
➔ obtener las canciones de un determinado artista. 
➔ modificar título de la playlist. 
➔ eliminar todas las canciones  */