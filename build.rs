use burn_import::onnx::ModelGen;

fn main() {
    // Generate Rust code from the ONNX model file
    ModelGen::new()
        .input("test/models/alexa_v0.1.onnx")
        .out_dir("test_models/")
        .run_from_script();
}
