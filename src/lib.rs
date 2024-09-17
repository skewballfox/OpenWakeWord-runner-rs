use mel_spec::prelude::*;
use mel_spec::vad::{duration_ms_for_n_frames, format_milliseconds};
use mel_spec_pipeline::prelude::*;

// struct WakewordDetector<Mel> {
//     // /// Preprocessor for audio data. Converts audio data to mel spectrogram, Optionally applies VAD.
//     // audio_filter: Mel,
//     model: T,
//     threshold: f32,
// }

// impl WakewordDetector<Mel> {
//     fn detect(&mut self, buffer: &[f32]) -> bool {
//         let mel = self.audio_filter.process(buffer);
//         let result = self.model.predict(&mel);
//         result > self.threshold
//     }
// }

mod test {
    // use super::*;
    fn get_mel() {}

    //     handles.push(handle);
    // }
    // #[test]
    // fn test_wakeword_detector() {
    //     let mel_filter = get_mel();
    //     let mut detector = WakewordDetector { model };
    //     let mut audio = Audio::new("resources/hey_snips_v4_with_core.wav").unwrap();
    //     let mut buffer = [0f32; 1280];
    //     let mut i = 0;
    //     while let Ok(_) = audio.read(&mut buffer) {
    //         let result = detector.detect(&buffer);
    //         if result {
    //             println!("Detected at {}", i);
    //         }
    //         i += 1;
    //     }
    // }
}
