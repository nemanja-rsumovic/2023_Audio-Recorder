mod audio_clip;
mod baza;

use audio_clip::AudioKlip;
use baza::Baza;

// Za rad sa komandnom linijom
use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;
// Za obradu gresaka
use color_eyre::eyre::Result;


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

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let baza = Baza::open()?;
    
    match args.command{                     // Switch na osnovu komande

        Commands::Record{name} => {
            eprintln!("Snimanje {:?}", name);
            
            let name = name.unwrap_or_else(|| "sladja".to_string());
            let mut clip = AudioKlip::record(name)?.resample(44100);
            baza.save(&mut clip)?;
            todo!();
        }

        Commands::Play{name} => {
            if let Some(clip) = baza.load(&name)? {
                eprintln!("Reprodukcija {:?}", name);
                clip.play()?;
            }
            else {
                eprintln!("Audio zapis sa tim imenom ne postoji!");
            }
            todo!();
            
        }

        Commands::List{} => {
            todo!();
        }
        Commands::Delete {name} => {
            eprintln!("Brisanje {}", name);
            todo!();
        }
    }
}
