use clap::{Parser, Subcommand};

use crate::scanner::{detect_project_type, ProjectType};

#[derive(Parser)]
#[command(name = "docgen")]
#[command(about = "Générateur automatique de documentation")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Génère ou met à jour le README
    Readme {
        /// Chemin du projet
        path: String,
    },

    /// Installe un hook Git pour mise à jour automatique
    InstallHook,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Readme { path } => {
                println!("📁 Analyse du projet : {}", path);

                // Détection du type de projet
                let project_type = detect_project_type(path);

                match project_type {
                    ProjectType::Rust => println!("🦀 Projet Rust détecté"),
                    ProjectType::Python => println!("🐍 Projet Python détecté"),
                    ProjectType::JavaScript => println!("📦 Projet JavaScript détecté"),
                    ProjectType::Generic => println!("📄 Projet générique détecté"),
                }

                // TODO : Appeler le générateur de README ici
                // generate_readme(path, project_type);
            }

            Commands::InstallHook => {
                println!("🔧 Installation du hook Git…");

                // TODO : Appeler la fonction d'installation du hook
                // install_git_hook();
            }
        }
    }
}
