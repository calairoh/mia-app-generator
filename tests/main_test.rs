
use std::process::Command;
use serde_json::Value;

#[test]
fn test_main() {
    // Create a test input file
    let test_input_path = "./tests/files/console_config.json";
    let test_output_path = "./tests/files/output_app_definition.json";
    let test_expected_path = "./tests/files/app_definition.json";

    // Run the main function with test paths
    let output = Command::new("cargo")
        .args(&["run", "--", test_input_path, test_output_path])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Verify the output file content
    let output_content = std::fs::read_to_string(test_output_path).unwrap();
    let expected_content = std::fs::read_to_string(test_expected_path).unwrap();

    // Deserialize the JSON content
    let output_json: Value = serde_json::from_str(&output_content).expect("Failed to parse output JSON");
    let expected_json: Value = serde_json::from_str(&expected_content).expect("Failed to parse expected JSON");

    // Compare the JSON values
    assert_eq!(output_json, expected_json, "The JSON content does not match");

    // Clean up test files
    std::fs::remove_file(test_output_path).unwrap();
}