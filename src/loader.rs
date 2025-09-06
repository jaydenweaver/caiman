use symphonia::core::{audio::AudioBufferRef, io::MediaSourceStream};

use crate::asset::*;
use std::path::Path;

pub trait AssetLoader: Send + Sync + 'static {
    fn load(path: &Path) -> Result<>;
}

pub struct TextureLoader;

impl AssetLoader for TextureLoader { // TextureLoader uses image-v0.25.6
    fn load(path: &Path) -> Result<Texture, AssetError> {
        let img = image::open(path).map_err(|_| AssetError::NotFound)?; // load file
        let rgba = img.to_rgba8(); // parse file to image
        Ok(Texture { width: rgba.width(), height: rgba.height(), data: rgba.into_raw() }) // create Texture object
    }
}

pub struct SoundLoader;

impl AssetLoader for SoundLoader {
    fn load(path: &Path) -> Result<Sound, AssetError> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let probed = get_probe()
                        .format(&Default::default(), mss, &Default::default(), &Default::default())
                        .map_err(|e| e.to_string())?;
        let mut format = probed.format;

        let track = format.default_track().ok_or_else(|| "no default audio track found".to_string())?;

        let mut decoder = get_codecs()
                            .make(&track.codec_params, &DecoderOptions::default())
                            .map_err(|e| e.to_string())?;

        let sample_rate = track.codec_params.sample_rate.ok_or("missing sample rate")?;
        let channels = track.codec_params.channels.ok_or("missing channels")?.count() as u8;
        let mut samples: Vec<i16> = Vec::new();

        loop {
            match format.next_packet() {
                Ok(packet) => {
                    let audio_buf = decoder.decode(&packet).map_err(|e| e.to_string())?;

                    match audio_buf {
                        AudioBufferRef::U8(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    let val = buf.chan(ch)[frame];
                                    samples.push((val as i16 - 128) << 8); // u8 -> i16
                                }
                            }
                        }
                        AudioBufferRef::U16(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    samples.push(buf.chan(ch)[frame] as i16);
                                }
                            }
                        }
                        AudioBufferRef::I16(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    samples.push(buf.chan(ch)[frame]);
                                }
                            }
                        }
                        AudioBufferRef::F32(buf) => {
                            for frame in 0..buf.frames() {
                                for ch in 0..buf.spec().channels.count() {
                                    let f = buf.chan(ch)[frame];
                                    samples.push((f * i16::MAX as f32) as i16);
                                }
                            }
                        }
                        _ => return Err("Unsupported sample format".into()),
                    }
                }
                Err(Error::IoError(e)) => return Err(e.to_string()),
                Err(Error::ResetRequired) => continue,
                Err(Error::DecodeError(e)) => return Err(e.to_string()),
                Err(Error::EndOfStream) => break,
            }
        }

        Ok(Sound {
            sample_rate,
            channels,
            samples,
        })
    }
}
