// use std::env;
use std::fs;
// use std::io::prelude::*;
use std::process::Command;


#[test]
fn bin_sample_test(){
    let preds_control = std::fs::read_to_string("resources/PREDICATE_CONTROL.scm").expect("file not found");
    let semtypes_control = std::fs::read_to_string("resources/SEMTYPE_CONTROL.scm").expect("file not found");

    let bin_path = if cfg!(debug_assertions){
        "target/debug/semmed"
    } else {
        "target/release/semmed"
    };

    let output = Command::new(bin_path)
                     .arg("resources/sample_semmed.csv")
                     .output()
                     .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let control = format!("{}{}",  preds_control, semtypes_control);

    assert_eq!(control, stdout)
}