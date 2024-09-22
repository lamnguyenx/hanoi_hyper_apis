// use std::fs::File;
// use symphonia::core::audio::AudioBufferRef;
// use symphonia::core::audio::Signal;
// use symphonia::core::codecs::DecoderOptions;
// use symphonia::core::formats::FormatOptions;
// use symphonia::core::formats::Packet;
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::core::probe::Hint;
// use webrtc_vad::Vad;

#[rustfmt::skip]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct SoundInfo {
    // Basics
    pub duration        : Option<f32>,
    pub sample_rate     : Option<u32>,
    pub channels        : Option<usize>,  // e.g: 1
    pub format          : Option<String>, // e.g: 'WAV'
    pub subtype         : Option<String>, // e.g: 'PCM_16'
    pub sample_width    : Option<u8>,     // e.g: 1, 2, 3, or 4
    pub frames          : Option<u64>,
    pub is_standard_wav : Option<bool>,
}

// Enums for sample width
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum SampleWidth {
    One,
    Two,
    Three,
    Four,
}

pub fn probe_audio<R>(file: R) -> SoundInfo
where
    R: symphonia::core::io::MediaSource + 'static,
{
    // read
    let mss = symphonia::core::io::MediaSourceStream::new(Box::new(file), Default::default());
    let _hint = symphonia::core::probe::Hint::new();
    let format_opts = symphonia::core::formats::FormatOptions::default();
    let metadata_opts = symphonia::core::meta::MetadataOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&_hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    let track = probed.format.default_track().unwrap();

    // sound info
    let codec_params = track.codec_params.clone();
    let mut sound_info = SoundInfo::default();
    sound_info.frames = codec_params.n_frames;
    sound_info.sample_rate = codec_params.sample_rate;

    if let (Some(frames), Some(sample_rate)) = (sound_info.frames, sound_info.sample_rate) {
        sound_info.duration = Some(crate::utils::common::round_dur(
            frames as f32 / sample_rate as f32,
            None,
        ));
    }
    sound_info.channels = Some(codec_params.channels.unwrap().count());

    // vad
    // let mut vad = webrtc_vad::Vad::new_with_rate_and_mode(
    //     webrtc_vad::SampleRate::Rate16kHz,
    //     webrtc_vad::VadMode::Quality,
    // );

    // let mut decoder = symphonia::default::get_codecs()
    //     .make(
    //         &codec_params,
    //         &symphonia::core::codecs::DecoderOptions::default(),
    //     )
    //     .unwrap();

    // while let Ok(packet) = probed.format.next_packet() {
    //     let decoded = decoder.decode(&packet).unwrap();
    //     process_audio_buffer(&decoded, &mut vad).unwrap();
    // }

    return sound_info;
}

// fn process_audio_buffer(
//     decoded: &AudioBufferRef,
//     vad: &mut Vad,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     match decoded {
//         AudioBufferRef::S16(buffer) => {
//             let samples = buffer.chan(0);
//             process_vad(vad, samples)?;
//             Ok(())
//         }
//         _ => Err("unsupported audio format".into()),
//     }
// }

// fn process_vad(vad: &mut Vad, samples: &[i16]) -> Result<(), Box<dyn std::error::Error>> {
//     for frame in samples.chunks(160) {
//         // Assuming 16kHz sample rate, 10ms frames
//         if frame.len() == 160 {
//             let is_speech = vad.is_voice_segment(frame).unwrap();
//             println!("{}", if is_speech { "Speech" } else { "Silence" });
//         }
//     }
//     Ok(())
// }
