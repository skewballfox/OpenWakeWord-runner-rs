use burn_ndarray::NdArray;
use mel_spec::{prelude::*, vad::duration_ms_for_n_frames};
use mel_spec_pipeline::prelude::*;
use std::thread;
mod alexa {
    include!(concat!(env!("OUT_DIR"), "/test_models/alexa_v0.rs"));
}

fn main() {
    let sample_rate = 16000.0;
    let fft_size = 512;
    let n_mels = 80;
    let hop_size = 160;
    let bit_depth = 32;
    let min_y = 3;
    let min_x = 5;
    let mel_settings = MelConfig::new(fft_size, hop_size, n_mels, sample_rate as f64);
    let vad_settings = DetectionSettings::new(1.0, min_y, min_x, 0);
    let audio_config = AudioConfig::new(bit_depth, sample_rate);
    let pipeline_config = PipelineConfig::new(audio_config, mel_settings, vad_settings);

    let pipeline = Pipeline::new(pipeline_config);
    let mel_rx = pipeline.mel_rx();

    #[cfg(debug_assertions)]
    let mel_path = "debug_out";

    let n_mels = 80;

    let model: alexa::Model<NdArray<f64>> = alexa::Model::default();
    let handle = thread::spawn(move || {
        //let ctx = WhisperContext::new(&model_path).expect("failed to load model");
        //let mut state = ctx.create_state().expect("failed to create key");

        let mut buf = PipelineOutputBuffer::new();
        while let Ok(mel) = mel_rx.recv() {
            if let Some(frames) = buf.add(mel.idx(), mel.frame()) {
                #[cfg(debug_assertions)]
                {
                    let debug_image_path = format!("{}/frame_{}.tga", mel_path, mel.idx());
                    let _ = save_tga_8bit(&frames, n_mels, &debug_image_path);
                }
                let current_mel_spec = duration_ms_for_n_frames(hop_size, sample_rate, mel.idx());
                // let ms = duration_ms_for_n_frames(hop_size, sampling_rate, mel.idx());
                // let time = format_milliseconds(ms as u64);

                // let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
                // params.set_n_threads(6);
                // params.set_single_segment(true);
                // params.set_language(Some("en"));
                // params.set_print_special(false);
                // params.set_print_progress(false);
                // params.set_print_realtime(false);
                // params.set_print_timestamps(false);
                // state.set_mel(&frames).unwrap();

                model.forward(input1)

                let empty = vec![];
                //state.full(params, &empty[..]).unwrap();

                //let num_segments = state.full_n_segments().unwrap();
                if num_segments > 0 {
                    if let Ok(text) = state.full_get_segment_text(0) {
                        let msg = format!("{} [{}] {}", mel.idx(), time, text);
                        println!("{}", msg);
                    } else {
                        println!("Error retrieving text for segment.");
                    }
                }
            }
        }
    });
    print!("Hello, world!")
}

fn default_pipeline() -> Pipeline {}
