
use std::fs;
use std::process::Command;

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

    assert_eq!(output_content.trim(), expected_content.trim());

    // Clean up test files
    std::fs::remove_file(test_output_path).unwrap();
}