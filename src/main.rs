use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TypePokemon {
    Feu,
    Eau,
    Plante,
    Electrik,
    Normal,
}

#[derive(Debug, Clone, PartialEq)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u32,
    type_pokemon: TypePokemon,
    experience: u32,
    genre: Genre,
}

// Partie 2: Fonctions et comportements
impl Pokemon {
    fn new(nom: String, type_pokemon: TypePokemon, genre: Genre) -> Self {
        Pokemon {
            nom,
            niveau: 1,
            type_pokemon,
            experience: 0,
            genre,
        }
    }

    fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        while self.experience >= 100 {
            self.niveau += 1;
            self.experience -= 100;
            println!("{} passe au niveau {}!", self.nom, self.niveau);
        }
    }

    fn afficher(&self) {
        println!("Nom: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {:?}", self.type_pokemon);
        println!("XP: {}/100", self.experience);
        println!("Genre: {:?}", self.genre);
        println!("------------------------");
    }

    fn peut_se_reproduire(&self, autre: &Pokemon) -> bool {
        if self.genre == autre.genre {
            return false;
        }
        if self.type_pokemon != autre.type_pokemon {
            return false;
        }
        if self.niveau < 5 || autre.niveau < 5 {
            return false;
        }
        true
    }

    // Partie 3: Reproduction
    fn reproduire(&self, autre: &Pokemon) -> Option<Pokemon> {
        if !self.peut_se_reproduire(autre) {
            println!("Reproduction impossible entre ces Pokémon!");
            return None;
        }

        // Détermination aléatoire du genre
        let genre = if rand::random::<bool>() {
            Genre::Male
        } else {
            Genre::Femelle
        };

        let nouveau = Pokemon {
            nom: String::from("Mystere"),
            niveau: 1,
            type_pokemon: self.type_pokemon.clone(),
            experience: 0,
            genre,
        };

        println!("Un nouveau Pokémon est né!");
        Some(nouveau)
    }
}

// Partie 4: Gestion de l'élevage
struct Elevage {
    pokemons: Vec<Pokemon>,
}

impl Elevage {
    fn new() -> Self {
        Elevage {
            pokemons: Vec::new(),
        }
    }

    fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
        println!("{} a été ajouté à l'élevage!", self.pokemons.last().unwrap().nom);
    }

    fn afficher_tous(&self) {
        if self.pokemons.is_empty() {
            println!("L'élevage est vide.");
            return;
        }

        println!("\nListe des Pokémon dans l'élevage:");
        for (i, pokemon) in self.pokemons.iter().enumerate() {
            println!("Pokémon #{}", i + 1);
            pokemon.afficher();
        }
    }

    fn entrainer_tous(&mut self, xp: u32) {
        if self.pokemons.is_empty() {
            println!("Il n'y a aucun Pokémon à entraîner.");
            return;
        }

        println!("Entraînement de tous les Pokémon (+{} XP)...", xp);
        for pokemon in &mut self.pokemons {
            pokemon.gagner_experience(xp);
        }
    }

    fn tenter_reproduction(&mut self, index1: usize, index2: usize) -> bool {
        if index1 >= self.pokemons.len() || index2 >= self.pokemons.len() {
            println!("Index de Pokémon invalide!");
            return false;
        }

        if index1 == index2 {
            println!("Un Pokémon ne peut pas se reproduire avec lui-même!");
            return false;
        }

        // Cloner pour éviter les problèmes d'emprunt
        let pokemon1 = self.pokemons[index1].clone();
        let pokemon2 = self.pokemons[index2].clone();

        if let Some(bebe) = pokemon1.reproduire(&pokemon2) {
            self.ajouter_pokemon(bebe);
            return true;
        }

        false
    }

    // Bonus: Trier les Pokémon
    fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| b.niveau.cmp(&a.niveau));
        println!("Pokémon triés par niveau (décroissant).");
    }
}

// Menu principal
fn menu_principal() -> u32 {
    println!("\n===== SYSTÈME D'ÉLEVAGE POKÉMON =====");
    println!("1. Ajouter un nouveau Pokémon");
    println!("2. Afficher tous les Pokémon");
    println!("3. Entraîner tous les Pokémon");
    println!("4. Tenter une reproduction");
    println!("5. Trier les Pokémon par niveau");
    println!("0. Quitter");
    println!("Votre choix: ");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Échec de la lecture");

    match choix.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrée invalide, veuillez réessayer.");
            999 // Valeur invalide pour forcer une nouvelle saisie
        }
    }
}

fn main() {

    let mut elevage = Elevage::new();

    loop {
        match menu_principal() {
            0 => {
                println!("Au revoir!");
                break;
            },
            1 => {
                // Ajouter un Pokémon
                println!("Entrez le nom du Pokémon: ");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Échec de la lecture");
                let nom = nom.trim().to_string();

                println!("Choisissez le type du Pokémon:");
                println!("1. Feu\n2. Eau\n3. Plante\n4. Electrik\n5. Normal");
                let mut type_choix = String::new();
                io::stdin().read_line(&mut type_choix).expect("Échec de la lecture");
                let type_pokemon = match type_choix.trim().parse::<u32>() {
                    Ok(1) => TypePokemon::Feu,
                    Ok(2) => TypePokemon::Eau,
                    Ok(3) => TypePokemon::Plante,
                    Ok(4) => TypePokemon::Electrik,
                    _ => TypePokemon::Normal,
                };

                println!("Choisissez le genre du Pokémon:");
                println!("1. Mâle\n2. Femelle");
                let mut genre_choix = String::new();
                io::stdin().read_line(&mut genre_choix).expect("Échec de la lecture");
                let genre = match genre_choix.trim().parse::<u32>() {
                    Ok(1) => Genre::Male,
                    _ => Genre::Femelle,
                };

                let pokemon = Pokemon::new(nom, type_pokemon, genre);
                elevage.ajouter_pokemon(pokemon);
            },
            2 => {
                // Afficher tous les Pokémon
                elevage.afficher_tous();
            },
            3 => {
                // Entraîner tous les Pokémon
                println!("Combien d'XP voulez-vous attribuer à chaque Pokémon?");
                let mut xp = String::new();
                io::stdin().read_line(&mut xp).expect("Échec de la lecture");
                match xp.trim().parse::<u32>() {
                    Ok(val) => elevage.entrainer_tous(val),
                    Err(_) => println!("Valeur XP invalide!"),
                }
            },
            4 => {
                // Tenter une reproduction
                elevage.afficher_tous();

                println!("Sélectionnez le premier Pokémon (numéro): ");
                let mut index1 = String::new();
                io::stdin().read_line(&mut index1).expect("Échec de la lecture");

                println!("Sélectionnez le second Pokémon (numéro): ");
                let mut index2 = String::new();
                io::stdin().read_line(&mut index2).expect("Échec de la lecture");

                match (index1.trim().parse::<usize>(), index2.trim().parse::<usize>()) {
                    (Ok(i1), Ok(i2)) => {
                        elevage.tenter_reproduction(i1 - 1, i2 - 1);
                    },
                    _ => println!("Indices invalides!"),
                }
            },
            5 => {
                // Trier les Pokémon par niveau
                elevage.trier_par_niveau();
                elevage.afficher_tous();
            },
            _ => println!("Option invalide. Veuillez réessayer."),
        }
    }
}