mod llm;
use llm::Model;
use std::path::{Path, PathBuf};

const MODEL_DIR: &str = "./candle-wasm-examples/phi_wasm/model";
const MODEL_NAME: &str = "Puffin-Phi-v2";

fn main() {
    let model_dir = PathBuf::from(MODEL_DIR).join(MODEL_NAME);
    let loaded_model = Model::load_from_file(model_dir, false);
    let mut model = match loaded_model {
        Ok(m) => m,
        Err(e) => panic!("failed to load the model: {}", e),
    };

    let prompt = "Tell me something about the moon.".to_string();
    let temp = 0.1;
    let top_p = 1.0;
    let repeat_penalty = 1.10;
    let repeat_last_n = 200; // Maximum context size
    let seed = 42;
    let init_result =
        model.init_with_prompt(prompt, temp, top_p, repeat_penalty, repeat_last_n, seed);
    let result = match init_result {
        Ok(r) => r,
        Err(e) => panic!("failed to init the model: {}", e),
    };
    println!("init_result: {}", result);
    for _ in 0..100 {
        let result = model.next_token().unwrap();
        print!("{}", result)
    }
}
