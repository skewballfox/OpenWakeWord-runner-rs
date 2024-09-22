use burn::tensor::{Tensor, TensorPrimitive};
use burn_ndarray::{NdArray, NdArrayDevice, NdArrayTensor};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use mel_spec::{prelude::*, rb::RingBuffer as MelFrameBuffer};

use ndarray::{Array2, Axis};
use ringbuf::{
    traits::{Consumer, Observer, Producer, Split},
    HeapRb,
};
use soundkit::{audio_bytes::deinterleave_vecs_f32, wav::WavStreamProcessor};

use std::{
    process::exit,
    thread::{self, panicking},
};
mod alexa {
    include!(concat!(env!("OUT_DIR"), "/test_models/alexa_v0.rs"));
}

fn main() {
    let mel_path = "./mel_out";
    let n_mels = 80;
    let fft_size = 512;
    let hop_size = 160;
    let n_mels = 80;
    let sampling_rate = 16000.0;

    let mel_config = MelConfig::new(fft_size, hop_size, n_mels, sampling_rate);

    let mut frame_buffer = MelFrameBuffer::new(mel_config, 1024);

    let mut processor = WavStreamProcessor::new();

    let (stream, mut consumer) = create_input_stream::<f64>(sampling_rate as u32, 128);

    let mut frames: Vec<Array2<f64>> = Vec::new();

    let wakeword: alexa::Model<NdArray<f64>> = alexa::Model::new(&NdArrayDevice::Cpu);

    let mut buffer = [0u8; 128];
    stream.play().unwrap();
    loop {
        if consumer.occupied_len() < buffer.len() {
            continue;
        }
        let _ = consumer.pop_slice(&mut buffer);
        let samples = deinterleave_vecs_f32(&buffer, 2);
        frame_buffer.add_frame(&samples[0]);
        if let Some(mel_frame) = frame_buffer.maybe_mel() {
            let ndtensor: NdArrayTensor<f64, 3> = NdArrayTensor::<f64, 3>::new(
                mel_frame.insert_axis(Axis(0)).into_dyn().into_shared(),
            );
            let float_primitive = TensorPrimitive::Float(ndtensor);
            let burn_tensor = Tensor::from_primitive(float_primitive);

            let res = wakeword.forward(burn_tensor);
            println!("{:?}", res);
        }
    }
    //println!("Using output device: \"{}\"", output_device.name()?);
}

fn create_input_stream<T>(
    sample_rate: u32,
    buffer_size: u32,
) -> (
    cpal::Stream,
    ringbuf::wrap::caching::Caching<
        std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<u8>>>,
        false,
        true,
    >,
)
where
    T: cpal::SizedSample,
{
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("no input device available");
    let device_config = cpal::StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(sample_rate),
        //todo: change later
        buffer_size: cpal::BufferSize::Fixed(buffer_size * 2),
    };
    println!("Using input device: \"{}\"", input_device.name().unwrap());

    let ring = HeapRb::<u8>::new(buffer_size as usize * 2);

    let (mut producer, mut consumer) = ring.split();

    let input_fn = move |data: &[u8], _: &cpal::InputCallbackInfo| {
        for &sample in data {
            if producer.try_push(sample).is_err() {
                println!("Error pushing to ring buffer");
            }
        }
    };

    let stream = input_device
        .build_input_stream(&device_config, input_fn, err_fn, None)
        .unwrap();

    (stream, consumer)
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
