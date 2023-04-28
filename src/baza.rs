extern crate rusqlite;

use color_eyre::eyre::Result;
use rusqlite::Connection;		// Za upravljanje bazom
use rusqlite::params;
use rusqlite::types::Type;

use crate::audio_clip::AudioKlip;

pub struct Baza(Connection);

//referenca3
fn encode(samples: &[f32]) -> Vec<u8> {
	let mut data = Vec::with_capacity(samples.len()*4);
	for sample in samples {
		data.extend_from_slice(&sample.to_be_bytes());
	}
	data
}
//decode

impl Baza{
	pub fn open() -> Result<Baza> {
		let connection = Connection::open("sejovaBaza.sqlite")?;

		connection.execute(
			"
			CREATE TABLE IF NOT EXISTS baza (
			id INTEGER PRIMARY KEY,
			sRates INTEGER NOT NULL,
			samples BLOB NOT NULL
			);
			",
			[],
			)?;

		Ok(Baza(connection))
	}

	pub fn save(&self, clip: &mut AudioKlip) -> Result<()> {
		self.0.execute(
			"INSERT OR REPLACE INTO baza (id, sRates, samples) VALUES (?1, ?2, ?3, ?4, ?5)",
			params![
			clip.id,
			clip.sRates,
			encode(&clip.samples),
			],
			)?;
		if clip.id.is_none() {
			clip.id = Some(self.0.last_insert_rowid().try_into()?); // Dodavanje id-a na osnovu prethodno unetog reda, uz try_into za sigurnu konverziju tipova
		}

		Ok(())
	}
}