use serde_json::{self, Value};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let models_dir = Path::new("budoux").join("budoux").join("models");
    let dest_dir = Path::new(&out_dir).join("models");

    fs::create_dir_all(&dest_dir)?;

    let features = vec![
        ("ja", env::var("CARGO_FEATURE_JA").is_ok()),
        ("zh-hans", env::var("CARGO_FEATURE_ZH_HANS").is_ok()),
        ("zh-hant", env::var("CARGO_FEATURE_ZH_HANT").is_ok()),
        ("th", env::var("CARGO_FEATURE_TH").is_ok()),
    ];

    for (lang, enabled) in features {
        if !enabled {
            continue;
        }

        let model_path = models_dir.join(format!("{}.json", lang));
        let contents = fs::read_to_string(&model_path)?;
        let model = serde_json::from_str::<Value>(&contents)?;

        let hashmap = generate_rust_hashmap(&model);
        let rust_code = format!(
            r#"
use std::collections::HashMap;
use std::sync::LazyLock;

pub static MODEL: LazyLock<HashMap<String, HashMap<String, i64>>> = LazyLock::new(|| {{
    {}
}});"#,
            hashmap
        );

        let model_name = lang.replace("-", "_");
        let dest_path = dest_dir.join(format!("{}.rs", model_name));
        let mut file = File::create(&dest_path)?;
        file.write_all(rust_code.as_bytes())?;

        println!("cargo:rerun-if-changed={}", model_path.display());
    }

    Ok(())
}

fn generate_rust_hashmap(json_data: &Value) -> String {
    let mut code = String::from("let mut model = HashMap::new();\n");

    if let Value::Object(obj) = json_data {
        for (key, value) in obj {
            if let Value::Object(inner_obj) = value {
                code.push_str(&format!(
                    "let mut {}_map = HashMap::new();\n",
                    sanitize_var_name(key)
                ));

                for (inner_key, inner_value) in inner_obj {
                    if let Value::Number(num) = inner_value {
                        if let Some(float_val) = num.as_i64() {
                            code.push_str(&format!(
                                "{}_map.insert(\"{}\".to_string(), {});\n",
                                sanitize_var_name(key),
                                escape_string(inner_key),
                                float_val
                            ));
                        }
                    }
                }

                code.push_str(&format!(
                    "model.insert(\"{}\".to_string(), {}_map);\n",
                    escape_string(key),
                    sanitize_var_name(key)
                ));
            }
        }
    }

    code.push_str("    model");
    code
}

fn sanitize_var_name(name: &str) -> String {
    name.replace("-", "_")
        .replace(".", "_")
        .replace(" ", "_")
        .to_ascii_lowercase()
}

fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"")
}
