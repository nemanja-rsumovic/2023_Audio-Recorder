# 2023_Audio-Recorder

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/1e6655af03f24245b32496f5dab9e933)](https://app.codacy.com/gh/matf-pp/2023_Audio-Recorder/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

## Opis teme
Projekat izrađen u okviru kursa [Programske paradigme](http://www.programskijezici.matf.bg.ac.rs/ProgramskeParadigmeI.html) napisan na programskom jeziku **Rust**.

Aplikacija Audio-Recorder namenjena je za snimanje zvuka i predviđena je za rad u terminalu.

## Neophodni alati
 [Rust i Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
 
 rusqlite:
 ```bash
sudo apt-get install libsqlite3-dev
 ```
 
 preuzimanje projekta:
 ```
 git clone https://github.com/matf-pp/2023_Audio-Recorder
 cd 2023_Audio-Recorder
 ```

## Komande za pokretanje i rad
*   cargo build - za kompilaciju lokalnih paketa i njihovih zavisnosti
*   cargo run **record** <**name**>
*   cargo run **play** <**name**>
*   cargo run **list**
*   cargo run **delete** <**name**>

Audio zapisi koji se snime se čuvaju u bazi pod nazivom  "audio_recordings.sqlite". Reprodukovanje snimaka iz baze se vrši po pozivu na osnovu zadatog imena. Takođe moguće je izlistavanje zabeleženih snimaka, ali i njihovo brisanje.

## Reference
[referenca](https://github.com/rustaudio/cpal)

## Autori
[Mitar Avramovic](https://github.com/MitarAvramovic1889)

[Nemanja Rsumovic](https://github.com/nemanja-rsumovic)
