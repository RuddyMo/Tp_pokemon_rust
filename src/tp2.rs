// TP Évaluation

use std::fs::File;
use std::io::{Write, BufReader, Read};

fn main() -> std::io::Result<()> {
    let content = read("nom_fichier.txt")?;
    write("tp2.txt", &content)?;

    Ok(())
}

fn read(name_file: &str) -> std::io::Result<String> {
    let file = File::open(name_file)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn write(name_file: &str, content: &str) -> std::io::Result<()> {


    let mut file = File::create(name_file)?;
    file.write_all(content.as_bytes())?; 
    println!("Le fichier à été créé avec succès !");

    Ok(())

}