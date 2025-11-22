use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::error::Error;
use std::time::Duration;
use std::thread;

pub struct AudioPlayer {
    _stream: OutputStream,
    music_sink: Arc<Mutex<Sink>>, // Música de fondo (loop)
    sfx_sink: Arc<Mutex<Sink>>,   // Efectos de sonido (una vez)
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Crear un nuevo stream de audio
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let music_sink = Sink::try_new(&stream_handle)?;
        let sfx_sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            _stream,
            music_sink: Arc::new(Mutex::new(music_sink)),
            sfx_sink: Arc::new(Mutex::new(sfx_sink)),
        })
    }

    pub fn play_background_music<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
        // Detener la música actual si hay alguna reproduciéndose
        self.stop_music();

        // Cargar el archivo de audio
        let file = BufReader::new(File::open(file_path)?);
        let source = Decoder::new(file)?;
        
        // Repetir la música infinitamente
        let source = source.repeat_infinite();
        
        // Reproducir la música
        if let Ok(sink) = self.music_sink.lock() {
            sink.append(source);
            sink.play();
        }
        
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.set_volume(volume);
        }
    }

    pub fn pause_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.pause();
        }
    }

    pub fn play_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.play();
        }
    }

    pub fn stop_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.stop();
        }
    }

    pub fn is_music_playing(&self) -> bool {
        if let Ok(sink) = self.music_sink.lock() {
            !sink.empty() && !sink.is_paused()
        } else {
            false
        }
    }

    // --- SFX ---
    pub fn play_sfx_once<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
        // Opcional: detener cualquier sfx anterior para evitar solapamientos
        if let Ok(sink) = self.sfx_sink.lock() {
            sink.stop();
        }

        let file = BufReader::new(File::open(file_path)?);
        let source = Decoder::new(file)?; // No repetir

        if let Ok(sink) = self.sfx_sink.lock() {
            sink.append(source);
            sink.play();
        }

        Ok(())
    }

    // Reproduce un SFX y "silencia" (pausa) la música de fondo por la duración del SFX
    // Si no puede obtener la duración, usa un fallback dado por parámetro.
    pub fn play_sfx_duck_music<P: AsRef<Path>>(
        &self,
        file_path: P,
        fallback_duration: Duration,
    ) -> Result<(), Box<dyn Error>> {
        // Detener SFX anterior para evitar solapamientos
        if let Ok(sink) = self.sfx_sink.lock() {
            sink.stop();
        }

        // Cargar el archivo de SFX y obtener duración
        let file = BufReader::new(File::open(&file_path)?);
        let decoder = Decoder::new(file)?; // No repetir
        let sfx_duration = decoder.total_duration().unwrap_or(fallback_duration);

        // Pausar la música
        self.pause_music();

        // Reproducir el SFX
        if let Ok(sink) = self.sfx_sink.lock() {
            sink.append(decoder);
            sink.play();
        }

        // Programar reanudación de la música después de la duración del SFX
        let music_sink = Arc::clone(&self.music_sink);
        thread::spawn(move || {
            thread::sleep(sfx_duration);
            if let Ok(sink) = music_sink.lock() {
                sink.play();
            }
        });

        Ok(())
    }
}

// Implementación de Default para facilitar la creación de instancias
impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().expect("No se pudo crear el reproductor de audio")
    }
}