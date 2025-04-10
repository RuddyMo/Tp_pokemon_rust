use std::io::{self, Write};

fn main() {
    let mut solde: f64 = 0.0;

    loop {
        println!("\n--- Gestion du compte bancaire ---");
        println!("1. Consulter le solde");
        println!("2. Effectuer un dépôt");
        println!("3. Effectuer un retrait");
        println!("4. Quitter");
        print!("Choisissez une option : ");
        io::stdout().flush().unwrap(); 

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Échec de la lecture du choix");
        let choix = match choix.trim().parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Option invalide !");
                continue;
            }
        };

        match choix {
            1 => {
                // 1. Consulter le solde
                println!("Votre solde est de {:.2} €", solde);
            },
            2 => {
                // 2. Effectuer un dépôt
                println!("Entrez le montant du dépôt :");
                let mut saisie = String::new();
                io::stdin().read_line(&mut saisie).expect("Échec de la lecture du montant");
                let montant = match saisie.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                solde += montant;
                println!("Dépôt de {:.2} € effectué. Nouveau solde : {:.2} €", montant, solde);
            },
            3 => {
                // 3. Effectuer un retrait
                println!("Entrez le montant du retrait :");
                let mut saisie = String::new();
                io::stdin().read_line(&mut saisie).expect("Échec de la lecture du montant");
                let montant = match saisie.trim().parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Montant invalide !");
                        continue;
                    }
                };
                if montant > solde {
                    println!("Solde insuffisant pour retirer {:.2} € !", montant);
                } else {
                    solde -= montant;
                    println!("Retrait de {:.2} € effectué. Nouveau solde : {:.2} €", montant, solde);
                }
            },
            4 => {
                // 4. Quitter
                println!("Au revoir !");
                break;
            },
            _ => {
                println!("Option invalide !");
            }
        }
    }
}
