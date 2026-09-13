use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn run_program(input: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let input_path = std::env::temp_dir().join(format!("payments_test_{timestamp}.csv"));

    fs::write(&input_path, input).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_payments-engine"))
        .arg(&input_path)
        .output()
        .expect("failed to execute program");

    fs::remove_file(&input_path).unwrap();

    assert!(
        output.status.success(),
        "program failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("program output was not valid UTF-8")
}

#[test]
fn deposit_and_withdrawal() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.5493
withdrawal,1,2,3.00
";

    let expected = "\
client,available,held,total,locked
1,7.5493,0,7.5493,false
";

    assert_eq!(run_program(input), expected);
}
