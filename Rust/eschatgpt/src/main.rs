use std::vec;

/* Hai il compito di creare un programma per gestire una libreria di oggetti di diversi tipi. Gli oggetti includono libri, DVD e CD. Dovrai implementare le seguenti classi:

Item: Questa è la classe di base che rappresenta un generico oggetto nella libreria. Deve avere un metodo display_details() che stampa le informazioni di base dell'oggetto.

Book: Questa classe rappresenta un libro e deriva dalla classe Item. Deve avere anche un campo author (autore) e un campo pages (pagine). Il metodo display_details() di Book deve stampare le informazioni di base dell'oggetto, insieme all'autore e al numero di pagine.

DVD: Questa classe rappresenta un DVD e deriva dalla classe Item. Deve avere anche un campo director (regista) e un campo duration (durata in minuti). Il metodo display_details() di DVD deve stampare le informazioni di base dell'oggetto, insieme al regista e alla durata.

CD: Questa classe rappresenta un CD e deriva dalla classe Item. Deve avere anche un campo artist (artista) e un campo tracks (numero di tracce). Il metodo display_details() di CD deve stampare le informazioni di base dell'oggetto, insieme all'artista e al numero di tracce.

Dopo aver implementato le classi, crea un array di oggetti Item che includa almeno un libro, un DVD e un CD. Successivamente, itera sull'array e chiama il metodo display_details() su ciascun oggetto per visualizzare le informazioni complete. */
fn main() {
    println!("Hello, world!");
    let items: Vec<Box<dyn Item>> = vec![
    Box::new(Book{
        title: String::from("The Lord of the Rings"),
        author: String::from("J.R.R. Tolkien"),
        pages: 1000
    }),
    Box::new(DVD{
        title: String::from("The Lord of the Rings"),
        director: String::from("Peter Jackson"),
        duration: 1000
    }),
    Box::new(CD{
        title: String::from("The Lord of the Rings"),
        artist: String::from("Howard Shore"),
        tracks: 1000
    })];
    for item in items.iter(){
        item.display_details();
    }
   
}

pub trait Item {
    fn display_details(&self);
}

pub struct Book {
    title: String,
    author: String,
    pages: u32,
}
impl Item for Book {
    fn display_details(&self) {
        println!("Book: {}, {}, {}", self.title,self.author, self.pages);
    }
}
pub struct DVD {
    title: String,
    director: String,
    duration: u32,
}
impl Item for DVD {
    fn display_details(&self) {
        println!("DVD: {}, {}, {}", self.title,self.director, self.duration);
    }
}
pub struct CD {
    title: String,
    artist: String,
    tracks: u32,
}
impl Item for CD {
    fn display_details(&self) {
        println!("CD:{}, {}, {}",self.title, self.artist, self.tracks);
    }
}