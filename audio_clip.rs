use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;

use cpal::traits:: DeviceTrait;
use cpal::traits:: HostTrait;
use cpal::traits:: StreamTrait;

use std::sync::Arc;
use std::sync::Mutex;

use dasp::interpolate::linear::Linear;
use dasp::signal;
use dasp::Signal;

// Komponente audioklipa
#[derive(Clone)]        // zarad dubokog kopiranja pri resamplingu za usaglasavanje
                        // in/out frekvencija
pub struct AudioKlip {
    pub id: Option<usize>,
    pub name: String,
    pub samples: Vec<f32>,
    pub s_rates: u32,
}

impl AudioKlip {

    pub fn record(name: String) -> Result<AudioKlip> {
        
        // Dohvatanje host-a preko cpal-a
        let host = cpal::default_host();


        // trazenje drajvera od strane audio host-a
        let device = host.default_input_device()
                         .ok_or_else(|| eyre!("Nije pronadjen input"))?; // ako nije pronadjen
    


        let config = device.default_input_config()?;

        let clip = AudioKlip {
            id: None,
            name,  
            samples: Vec::new(),
            s_rates: config.sample_rate().0,
        };

        let clip = Arc::new(Mutex::new(Some(clip)));
        // "Bafer sa muteksom"
        let clip1 = clip.clone();

        //begin recording...
        println!("\nSnimanje zapoceto:\n.\n.\n.");

        let channels = config.channels();

        // Handler za mutex
        type ClipHandle = Arc<Mutex<Option<AudioKlip>>>;

        let streamerr = move |err| {
            eprintln!("Greska na stream-u: {}", err);
        };

        // referenca1

        // f-ja za hvatanje
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

        // opis stream-ovanja
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32>(data, channels, &clip1), // hvatanje-prebacivanje
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

        println!("Snimljeno je {} semplova.", clip.samples.len());

        Ok(clip)


        // STAVI POKRETANJE NA NEKO DUGME???
        // BOLJE POKRETANJE PO POZIVU

    }

    // resamplovanje-usaglasavanje frekvencija IO

    pub fn resample(&self, s_rates: u32) -> AudioKlip {
        
        if self.s_rates == s_rates {
            return self.clone();
        }

        // Interpolacija signala
        let mut signal = signal::from_iter(self.samples.iter().copied());
        let x = signal.next();
        let y = signal.next();

        let linear = Linear::new(x, y);

        AudioKlip {
            id: self.id,
            name: self.name.clone(),
            samples: signal 
                .from_hz_to_hz(linear, self.s_rates as f64, s_rates as f64)
                .take(self.samples.len() * (s_rates as usize) / (self.s_rates as usize))      //ref2
                .collect(),
            s_rates,
        }
    }


    pub fn play(&self) -> Result<()> {

        let host = cpal::default_host();
        
        let device = host.default_output_device()
                         .ok_or_else(|| eyre!("Nije pronadjen uredjaj"))?;
        
            
        let config = device.default_output_config()?;

        println!("\nReprodukcija snimka u toku:\n.\n.\n.");

        type StateHandle = Arc<Mutex<Option<(usize, Vec<f32>)>>>;
        
        let sample_rate = config.sample_rate().0;
        
        
        let state = (0, self.resample(sample_rate).samples);
        let state = Arc::new(Mutex::new(Some(state)));
        let channels = config.channels();

        let streamerr = move |err| {
            eprintln!("Greska na stream-u: {}", err);
        };

        // referenca na referencu1
        // input -> output
        fn write_output_data<T>(output: &mut [T], channels: u16, writer: &StateHandle)
        where
            T: cpal::Sample,
        {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some((i, clip_samples)) = guard.as_mut() {
                    for frame in output.chunks_mut(channels.into()) {
                        for sample in frame.iter_mut() {
                            *sample = cpal::Sample::from(clip_samples.get(*i).unwrap_or(&0f32));
                        }
                        *i += 1;
                    }
                   
                }
            }
        }

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<f32>(data, channels, &state),
                streamerr,
            )?,
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<i16>(data, channels, &state),
                streamerr,
            )?,
            cpal::SampleFormat::U16 => device.build_output_stream(
                &config.into(),
                move |data, _: &_| write_output_data::<u16>(data, channels, &state),
                streamerr,
            )?,
        };

        stream.play()?;
        std::thread::sleep(std::time::Duration::from_secs(5));

        println!("Reprodukcija snimka zavrsena.");

        Ok(())
        
    }
}