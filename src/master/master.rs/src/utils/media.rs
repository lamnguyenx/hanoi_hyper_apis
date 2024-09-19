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

pub fn probe_audio(file: &std::path::PathBuf) -> SoundInfo {
    let file_obj = std::fs::File::open(file).unwrap();

    let _hint = symphonia::core::probe::Hint::new();
    let mss = symphonia::core::io::MediaSourceStream::new(Box::new(file_obj), Default::default());
    let format_opts = symphonia::core::formats::FormatOptions::default();
    let metadata_opts = symphonia::core::meta::MetadataOptions::default();

    let probed = symphonia::default::get_probe()
        .format(&_hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    let codec_params = probed.format.default_track().unwrap().codec_params.clone();
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

    return sound_info;
}
