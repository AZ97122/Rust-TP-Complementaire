use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use chrono::{DateTime, Local};

pub struct Fichier {
    nom: String,
    contenu: String,
    date_creation: DateTime<Local>,
}

impl Fichier {
    /// Constructeur pour créer une nouvelle instance de Fichier
    pub fn new(nom: &str, contenu: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
            date_creation: Local::now(),
        }
    }

    /// Crée réellement le fichier et écrit le contenu présent dans la structure.
    pub fn creer_fichier(&self) -> io::Result<()> {
        let mut f = File::create(&self.nom)?;
        f.write_all(self.contenu.as_bytes())?;
        Ok(())
    }

    /// Méthode statique pour créer directement un fichier sans instance
    pub fn creer_avec_nom(nom: &str, contenu: &str) -> io::Result<()> {
        let mut f = File::create(nom)?;
        f.write_all(contenu.as_bytes())?;
        Ok(())
    }

    /// Modifier le contenu stocké dans la structure
    pub fn modifier_contenu(&mut self, nouveau_contenu: &str) {
        self.contenu = nouveau_contenu.to_string();
    }

    /// Ajouter du contenu à la fin du contenu existant
    pub fn ajouter_contenu(&mut self, contenu_a_ajouter: &str) {
        self.contenu.push_str(contenu_a_ajouter);
    }

    /// Vérifier si le fichier existe sur le disque
    pub fn existe(&self) -> bool {
        Path::new(&self.nom).exists()
    }

    // Getters

    /// Getter pour le nom
    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    /// Getter pour le contenu
    pub fn get_contenu(&self) -> &str {
        &self.contenu
    }

    /// Getter pour la date de création
    pub fn get_date_creation(&self) -> DateTime<Local> {
        self.date_creation
    }
}