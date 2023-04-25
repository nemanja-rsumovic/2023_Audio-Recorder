use clap::{AppSettings,Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "SejoKalac")]
#[clap(about = "Pozz")]

struct Cli{             // Cli za parsiranje komandi i formiranje strukture od njih
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]// Parsiranje podkomandi
enum Commands{              

    Record{                 // Snimanje, ime opciono
        name: Option<String>,
    },
    
    #[clap(setting(AppSettings::ArgRequiredElseHelp))] // Neophodan argument(koga pustamo)
    Play{

        name: String,
    },
    
    List{},
    
    #[clap(setting(AppSettings::ArgRequiredElseHelp))] // Neophodan argument(koga brisemo)
    Delete{
        name:String,
    },
       
}

fn main() {
    let args = Cli::parse();
    
    match args.command{                     // Switch na osnovu komande

        Commands:: Record{name} => {
            eprintln!("Record {:?}", name);
            todo!();                        // Makro za prazno telo, treba da se uvede obejakt audio_klip sa svojim metodama
        }

        Commands:: Play{name} => {
            eprintln!("Play {}", name);
        }

        Commands::List{} => {
            todo!();
        }
        Commands::Delete {name} => {
            eprintln!("Delete {}", name);
            todo!();
        }
    }
}
