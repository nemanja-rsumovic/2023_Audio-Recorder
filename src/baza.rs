extern crate rusqlite;

use color_eyre::eyre::Result;
use rusqlite::Connection;		// Za upravljanje bazom
use rusqlite::params;
use rusqlite::types::Type;

use crate::audio_clip::AudioKlip;

pub struct Baza(Connection);

pub struct Podaci{
	pub id: usize,
	pub name: String,
}

//referenca3
fn encode(samples: &[f32]) -> Vec<u8> {
	let mut data = Vec::with_capacity(samples.len()*4);
	for sample in samples {
		data.extend_from_slice(&sample.to_be_bytes());
	}
	data
}

fn decode(bytes: &[u8]) -> Vec<f32> {
	let mut samples = Vec::with_capacity(bytes.len()/4);
	for chunk in bytes.chunks(4) {
		samples.push(f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
	}
	samples
}
//kraj ref3 

impl Baza{
	pub fn open() -> Result<Baza> {
		let connection = Connection::open("sejovaBaza.sqlite")?;

		connection.execute(
			"
			CREATE TABLE IF NOT EXISTS baza (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL UNIQUE,
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
			"INSERT OR REPLACE INTO baza (id, name, sRates, samples) VALUES (?1, ?2, ?3, ?4)",
			params![
			clip.id,
			clip.name,
			clip.sRates,
			encode(&clip.samples),
			],
			)?;
		if clip.id.is_none() {
			clip.id = Some(self.0.last_insert_rowid().try_into()?); // Dodavanje id-a na osnovu prethodno unetog reda, uz try_into za sigurnu konverziju tipova
		}

		Ok(())
	}

	pub fn load(&self, name: &str) -> Result<Option<AudioKlip>> {
		let mut stm = self.0.prepare("SELECT id, name, sRates, samples FROM baza WHERE name = ?1")?;
		let mut st_mach = stm.query_map([name], |row| {		// Kao state masina iz grafike, prvo je napravimo pa je podignemo
			let sampl: Vec<u8> = row.get(3)?;

			Ok(AudioKlip{
				id: Some(row.get(0)?),
				name: row.get(1)?,
				sRates: row.get(2)?,
				samples: decode(&sampl),
			})
		})?;												// bez ? - query_map, sa ? - (query_map, err) 
	

		Ok(if let Some(clip) = st_mach.next() {
				Some(clip?)
			}
			else {
				None
			})

	}

	pub fn list(&self) -> Result<Vec<Podaci>>{
		let mut stm = self.0.prepare("SELECT id, name FROM baza ORDER BY name")?;
		let st_mach = stm.query_map([], |row| {
			Ok(Podaci{
				id: row.get(0)?,
				name: row.get(1)?,
			})
		})?;
	
		Ok(st_mach.collect::<Result<_, rusqlite::Error>>()?)
	}

	pub fn delete(&self, name: &str) -> Result<()> {
		self.0.execute(
			"DELETE FROM baza WHERE name = ?1", [name]
			)?;

		Ok(())
	}
	
}