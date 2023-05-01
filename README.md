# 2023_Audio-Recorder

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/1e6655af03f24245b32496f5dab9e933)](https://app.codacy.com/gh/matf-pp/2023_Audio-Recorder/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

Projekat izrađen u okviru kursa [Programske paradigme](http://www.programskijezici.matf.bg.ac.rs/ProgramskeParadigmeI.html) napisan na programskom jeziku **Rust**.

Aplikacija Audio-Recorder namenjena je za snimanje zvuka i predviđena je za rad u terminalu.

## Komande
* cargo build - za kompilaciju lokalnih paketa i njihovih zavisnosti
* cargo run **record name**
* cargo run **play name**
* cargo run **list**
* cargo run **delete name**

Audio zapisi koji se snime se čuvaju u bazi pod nazivom  "audio_recordings.sqlite". Reprodukovanje snimaka iz baze se vrši po pozivu na osnovu zadatog imena. Takođe moguće je izlistavanje zabeleženih snimaka, ali i njihovo brisanje.

## Reference --dopuniti--
1. [referenca1]()
2. [referenca2]()
3. [referenca3]()
