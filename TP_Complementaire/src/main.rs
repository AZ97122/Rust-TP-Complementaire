mod fonctions; // import des modules nécessaires et de fonctions.rs qui contient nos fonctions
use fonctions::Fichier;
use std::io::{self, Write, stdin};

fn main() -> io::Result<()> {
    let mut fichier = Fichier::new("exemple.txt", "Texte initial\n");


    // Boucle infinie qui affiche le menu et traite les choix de l'utilisateur
    loop {
        println!("\n=== MENU FICHIER ===");
        println!("1. Créer/écrire le fichier");
        println!("2. Modifier le contenu");
        println!("3. Ajouter du contenu");
        println!("4. Vérifier l'existence du fichier");
        println!("5. Afficher les informations du fichier");
        println!("6. Créer un fichier directement avec méthode statique");
        println!("0. Quitter");

        // L'utilisateur chosit le numéro de l'option
        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        stdin().read_line(&mut choix)?;
        match choix.trim() {
            // Creer fichier
            "1" => {
                fichier.creer_fichier()?;
                println!("Fichier '{}' écrit sur le disque.", fichier.get_nom());
            },
            // Modifierle contenu en mémoire
            "2" => {
                println!("Entrez le nouveau contenu :");
                let mut nouveau = String::new();
                stdin().read_line(&mut nouveau)?;
                fichier.modifier_contenu(nouveau.trim());
                println!("Contenu modifié (en mémoire).");
            },
            // Ajouter du disque au fichier
            "3" => {
                println!("Texte à ajouter :");
                let mut ajout = String::new();
                stdin().read_line(&mut ajout)?;
                fichier.ajouter_contenu(ajout.trim());
                println!("Ajout fait (en mémoire).");
            },
            // Vérifier si le fichier existe
            "4" => {
                println!(
                    "Le fichier '{}' existe ? {}",
                    fichier.get_nom(),
                    fichier.existe()
                );
            },
            // Afficher nom contenu et la date de création
            "5" => {
                println!("Nom : {}", fichier.get_nom());
                println!("Contenu (en mémoire) : {}", fichier.get_contenu());
                println!("Date de création : {}", fichier.get_date_creation());
            },
            // Création rapide sans l'instance
            "6" => {
                println!("Nom du nouveau fichier :");
                let mut nom = String::new();
                stdin().read_line(&mut nom)?;
                println!("Contenu du nouveau fichier :");
                let mut contenu = String::new();
                stdin().read_line(&mut contenu)?;
                Fichier::creer_avec_nom(nom.trim(), contenu.trim())?;
                println!("Fichier '{}' créé via la méthode statique.", nom.trim());
            },
            // Quitter le programme
            "0" => break,
            _ => println!("Choix non reconnu, veuillez réessayer."),
        }
    }

    Ok(())
}
