use std::io;
use std::fs::File;
use std::io::{Write, Read};

struct Livre {
    titre: String,
    auteur: String,
    isbn: String,
    annee: u32,
}

fn main() {
    // J'initialise la bibliotheque
    let mut bibliotheque: Vec<Livre> = Vec::new();

    match charger_bibliotheque(&mut bibliotheque) {
        Ok(_) => println!("Bibliothèque chargée avec succès."),
        Err(e) => println!("Erreur lors du chargement: {}", e),
    }

    // Boucle principale du menu
    loop {
        println!("\n==Menu=== GESTION DE BIBLIOTHÈQUE =====");
        println!("1. Ajouter un livre");
        println!("2. Rechercher un livre par titre");
        println!("3. Afficher tous les livres");
        println!("4. Retirer un livre");
        println!("5. Quitter");
        println!("Votre choix: ");

        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let choix = choix.trim();

        // On traite le choix
        match choix {
            "1" => ajouter_livre(&mut bibliotheque),
            "2" => rechercher_livre(&bibliotheque),
            "3" => afficher_livres(&bibliotheque),
            "4" => retirer_livre(&mut bibliotheque),
            "5" => {
                match sauvegarder_bibliotheque(&bibliotheque) {
                    Ok(_) => println!("Bibliothèque sauvegardée avec succès."),
                    Err(e) => println!("Erreur lors de la sauvegarde: {}", e),
                }
                println!("Au revoir !");
                break;
            },
            _ => println!("Choix invalide. Veuillez réessayer."),
        }
    }
}

// Ajout d'un livre
fn ajouter_livre(bibliotheque: &mut Vec<Livre>) {
    println!("\n== Ajouter un livre ==");

    // Collecte des informations du livre
    println!("Titre: ");
    let mut titre = String::new();
    io::stdin()
        .read_line(&mut titre)
        .expect("Erreur de lecture");
    let titre = titre.trim().to_string();

    println!("Auteur: ");
    let mut auteur = String::new();
    io::stdin()
        .read_line(&mut auteur)
        .expect("Erreur de lecture");
    let auteur = auteur.trim().to_string();

    println!("ISBN: ");
    let mut isbn = String::new();
    io::stdin()
        .read_line(&mut isbn)
        .expect("Erreur de lecture");
    let isbn = isbn.trim().to_string();

    println!("Année de publication: ");
    let mut annee_str = String::new();
    io::stdin()
        .read_line(&mut annee_str)
        .expect("Erreur de lecture");

    // On convertie l'annee
    let annee: u32 = match annee_str.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Année invalide, utilisation de 0");
            0
        }
    };

    let nouveau_livre = Livre {
        titre,
        auteur,
        isbn,
        annee,
    };

    bibliotheque.push(nouveau_livre);
    println!("Livre ajouté avec succès !");

    // Je sauvegarde la bibliotheque
    match sauvegarder_bibliotheque(&bibliotheque) {
        Ok(_) => {},
        Err(e) => println!("Erreur lors de la sauvegarde: {}", e),
    }
}

// Recherche par livre
fn rechercher_livre(bibliotheque: &Vec<Livre>) {
    println!("\n== Rechercher un livre ==");

    println!("Entrez un titre (ou une partie): ");
    let mut recherche = String::new();
    io::stdin()
        .read_line(&mut recherche)
        .expect("Erreur de lecture");
    let recherche = recherche.trim().to_lowercase();

    let mut trouves = 0;

    // On parcours les livres de la bibliothèque
    for (index, livre) in bibliotheque.iter().enumerate() {
        if livre.titre.to_lowercase().contains(&recherche) {
            println!("Livre #{}", index + 1);
            println!("  Titre: {}", livre.titre);
            println!("  Auteur: {}", livre.auteur);
            println!("  ISBN: {}", livre.isbn);
            println!("  Année: {}", livre.annee);
            println!();
            trouves += 1;
        }
    }

    if trouves == 0 {
        println!("Aucun livre trouvé avec ce titre.");
    }
}

// Affichage des livres
fn afficher_livres(bibliotheque: &Vec<Livre>) {
    println!("\n== Liste de tous les livres ==");

    if bibliotheque.is_empty() {
        println!("La bibliothèque est vide.");
        return;
    }

    for (index, livre) in bibliotheque.iter().enumerate() {
        println!("Livre #{}", index + 1);
        println!("  Titre: {}", livre.titre);
        println!("  Auteur: {}", livre.auteur);
        println!("  ISBN: {}", livre.isbn);
        println!("  Année: {}", livre.annee);
        println!();
    }
}

// Suppression d'un livre
fn retirer_livre(bibliotheque: &mut Vec<Livre>) {
    println!("\n== Retirer un livre ==");

    if bibliotheque.is_empty() {
        println!("La bibliothèque est vide.");
        return;
    }

    afficher_livres(bibliotheque);

    println!("Entrez le numéro du livre à retirer: ");
    let mut index_str = String::new();
    io::stdin()
        .read_line(&mut index_str)
        .expect("Erreur de lecture");

    // Conversion de l'index
    let index: usize = match index_str.trim().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("Numéro invalide.");
            return;
        }
    };

    if index < bibliotheque.len() {
        let titre = bibliotheque[index].titre.clone();
        bibliotheque.remove(index);
        println!("Livre '{}' retiré avec succès.", titre);

        match sauvegarder_bibliotheque(&bibliotheque) {
            Ok(_) => {},
            Err(e) => println!("Erreur lors de la sauvegarde: {}", e),
        }
    } else {
        println!("Numéro de livre invalide.");
    }
}

// Sauvegarde de la bibliothèque dans un fichier
fn sauvegarder_bibliotheque(bibliotheque: &Vec<Livre>) -> Result<(), String> {
    let mut fichier = match File::create("bibliotheque.txt") {
        Ok(f) => f,
        Err(_) => return Err("Impossible de créer le fichier".to_string()),
    };

    // Écriture de chaque livre au format délimité
    for livre in bibliotheque {
        let ligne = format!("{}|{}|{}|{}\n", livre.titre, livre.auteur, livre.isbn, livre.annee);
        if let Err(_) = fichier.write_all(ligne.as_bytes()) {
            return Err("Erreur lors de l'écriture dans le fichier".to_string());
        }
    }

    Ok(())
}

// Chargement de la bibliothèque depuis un fichier
fn charger_bibliotheque(bibliotheque: &mut Vec<Livre>) -> Result<(), String> {
    let mut contenu = String::new();

    let mut fichier = match File::open("bibliotheque.txt") {
        Ok(f) => f,
        Err(_) => return Ok(()),
    };

    if let Err(_) = fichier.read_to_string(&mut contenu) {
        return Err("Impossible de lire le contenu du fichier".to_string());
    }

    // Traitement des lignes du fichier
    for ligne in contenu.lines() {
        let parties: Vec<&str> = ligne.split('|').collect();

        if parties.len() == 4 {
            let annee = match parties[3].parse::<u32>() {
                Ok(num) => num,
                Err(_) => continue,  // Ignorer les années invalides
            };

            let livre = Livre {
                titre: parties[0].to_string(),
                auteur: parties[1].to_string(),
                isbn: parties[2].to_string(),
                annee,
            };

            bibliotheque.push(livre);
        }
    }

    Ok(())
}