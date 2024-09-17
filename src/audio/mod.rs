pub mod mel {
    //include!(concat!(env!("OUT_DIR"), "/audio/mel_onnx.rs"));
}
use mel::Mel;
pub struct Mel {
    pub mel: mel::Mel,
}
