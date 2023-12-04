pub fn run(dir: &str, function: fn(&str) -> u32, expected_result: u32) {
    let test_result = function(&(dir.to_owned() + "/" + "input-test"));

    println!("Test result: {test_result}");

    assert!(test_result == expected_result);

    let result = function(&(dir.to_owned() + "/" + "input"));

    println!("Result: {result}");
}
