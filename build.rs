use burn_import::onnx::ModelGen;
fn main() {
    // Generate Rust code from the ONNX model file
    ModelGen::new()
        //.input("src/model/linear_DC_CO.onnx")
        .input("../IAFGNN/model_ln/linear_DC_CO_9f.onnx")
        //.input("model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();
    ModelGen::new()
        //.input("src/model/linear_DC_CO.onnx")
        .input("../IAFGNN/model_ln/linear_DC_ST_9f.onnx")
        //.input("model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();
    ModelGen::new()
        //.input("src/model/linear_DC_CO.onnx")
        .input("../IAFGNN/model_ln/linear_DS_PR_9f.onnx")
        //.input("model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();
    ModelGen::new()
        //.input("src/model/linear_DC_CO.onnx")
        .input("../IAFGNN/model_ln/linear_DS_ST_9f.onnx")
        //.input("model/mnist.onnx")
        .out_dir("model/")
        .run_from_script();
}