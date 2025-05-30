use burn_import::onnx::ModelGen;
static EMBED_WEIGHT : bool = true;
fn main() {
    // Generate Rust code from the ONNX model file
    let supported_task = ["DC_CO", "DC_ST","DC_SST", "DC_ID", "DS_ST","DS_SST", "DS_PR"];
    for task in supported_task {
        let mut path = String::from("model_onnx/linear_");
        path.push_str(task);
        path.push_str("_9f3_d0.2.onnx");
        ModelGen::new()
            .input(&path)
            .out_dir("model/")
            .record_type(burn_import::onnx::RecordType::Bincode)
            .embed_states(EMBED_WEIGHT)
            .run_from_script();
    }
    /*ModelGen::new()
        .input("../af_research/IAFGNN/model_ln/linear_DC_ST_9f2_d0.2.onnx")
        .out_dir("model/")
        .record_type(burn_import::onnx::RecordType::Bincode)
        .embed_states(EMBED_WEIGHT)
        .run_from_script();
    ModelGen::new()
        .input("../af_research/IAFGNN/model_ln/linear_DS_PR_9f2_d0.2.onnx")
        .out_dir("model/")
        .record_type(burn_import::onnx::RecordType::Bincode)
        .embed_states(EMBED_WEIGHT)
        .run_from_script();
    ModelGen::new()
        .input("../af_research/IAFGNN/model_ln/linear_DS_ST_9f2_d0.2.onnx")
        .out_dir("model/")
        .record_type(burn_import::onnx::RecordType::Bincode)
        .embed_states(EMBED_WEIGHT)
        .run_from_script();*/
}