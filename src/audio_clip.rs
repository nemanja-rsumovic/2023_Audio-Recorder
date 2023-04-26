use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;

use cpal::traits:: DeviceTrait;
use cpal::traits:: HostTrait;
use cpal::traits:: StreamTrait;

use std::sync::Arc;
use std::sync::Mutex;

// Komponente audioklipa
pub struct AudioKlip {
    pub samples: Vec<f32>,
    pub sRates: u32,
}

impl AudioKlip {

    pub fn record(name: String) -> Result<AudioKlip> {
        
        // Dohvatanje host-a preko cpal-a
        let host = cpal::default_host();


        // trazenje drajvera od strane audio host-a
        let device = host.default_input_device()
                         .ok_or_else(|| eyre!("Not found input"))?; // ako nije pronadjen
    


        let config = device.default_input_config()?;

        let clip = AudioKlip {
            samples: Vec::new(),
            sRates: config.sample_rate().0,
        };

        let clip = Arc::new(Mutex::new(Some(clip)));
        // "Bafer sa muteksom"
        let clip1 = clip.clone();

        //begin recording...
        println!("Snimanje zapoceto...");

        let channels = config.channels();

        // Handler za mutex
        type ClipHandle = Arc<Mutex<Option<AudioKlip>>>;

        let streamerr = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        // referenca1
        fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some(clip) = guard.as_mut() {
                    for frame in input.chunks(channels.into()) {
                        clip.samples.push(frame[0].to_f32());
                    }
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32>(data, channels, &clip1),
                streamerr,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16>(data, channels, &clip1),
                streamerr,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<u16>(data, channels, &clip1),
                streamerr,
            )?,
        };
        // kraj reference1

        // zapoceto snimanje
        stream.play()?;

        std::thread::sleep(std::time::Duration::from_secs(5));
        //zatvaranje
        drop(stream);
        println!("Snimanje uspesno zavrseno!");

        let clip = clip.lock().unwrap().take().unwrap();

        println!("Snimljeno: {}", clip.samples.len());

        Ok(clip)


        // STAVI POKRETANJE NA NEKO DUGME???
        // BOLJE POKRETANJE PO POZIVU

    }

}