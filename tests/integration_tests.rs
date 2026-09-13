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
        "program failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("program output was not valid UTF-8")
}

fn run_program_expect_error(input: &str) -> String {
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

    assert!(!output.status.success(), "program unexpectedly succeeded");

    String::from_utf8(output.stderr).expect("program error output was not valid UTF-8")
}

fn sort_csv_rows(csv: &str) -> String {
    let mut lines: Vec<&str> = csv.lines().collect();

    let header = lines.remove(0);
    lines.sort_unstable();

    std::iter::once(header)
        .chain(lines)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

#[test]
fn basic_deposit() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn multiple_deposits() {
    let input = "\
type,client,tx,amount
deposit,1,1,10
deposit,1,2,5.50
deposit,1,3,2.25
";

    let expected = "\
client,available,held,total,locked
1,17.75,0,17.75,false
";

    assert_eq!(sort_csv_rows(&run_program(input)), sort_csv_rows(expected));
}

#[test]
fn withdrawal_with_sufficient_funds() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
withdrawal,1,2,4.00
";

    let expected = "\
client,available,held,total,locked
1,6,0,6,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn withdrawal_with_insufficient_funds() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
withdrawal,1,2,15.00
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn multiple_clients() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
deposit,2,2,20.00
withdrawal,1,3,3.00
withdrawal,2,4,5.00
";

    let expected = "\
client,available,held,total,locked
1,7,0,7,false
2,15,0,15,false
";

    assert_eq!(sort_csv_rows(&run_program(input)), sort_csv_rows(expected));
}

#[test]
fn withdrawal_after_a_deposit() {
    let input = "\
type,client,tx,amount
deposit,1,1,100.00
withdrawal,1,2,25.00
deposit,1,3,10.00
withdrawal,1,4,15.00
";

    let expected = "\
client,available,held,total,locked
1,70,0,70,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn dispute_of_a_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
dispute,1,1,
";

    let expected = "\
client,available,held,total,locked
1,0,10,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn resolve_of_a_disputed_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
dispute,1,1,
resolve,1,1,
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn chargeback() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
dispute,1,1,
chargeback,1,1,
";

    let expected = "\
client,available,held,total,locked
1,0,0,0,true
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn attempt_to_operate_on_locked_client() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
dispute,1,1,
chargeback,1,1,
deposit,1,2,5.00
";

    let expected = "\
client,available,held,total,locked
1,0,0,0,true
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn dispute_referencing_nonexistent_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
dispute,1,999,
";

    // If your implementation ignores invalid disputes:
    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn dispute_referencing_another_clients_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
deposit,2,2,20.00
dispute,1,2,
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
2,20,0,20,false
";

    assert_eq!(sort_csv_rows(&run_program(input)), sort_csv_rows(expected));
}

#[test]
fn resolve_of_nonexistent_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
resolve,1,999,
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn chargeback_of_nonexistent_transaction() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.00
chargeback,1,999,
";

    let expected = "\
client,available,held,total,locked
1,10,0,10,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn transactions_with_unordered_ids() {
    let input = "\
type,client,tx,amount
deposit,1,42,10.00
withdrawal,1,7,5.00
deposit,1,103,20.00
";

    let expected = "\
client,available,held,total,locked
1,25,0,25,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn decimal_amounts_and_precision() {
    let input = "\
type,client,tx,amount
deposit,1,1,10.1234
deposit,1,2,0.0001
withdrawal,1,3,2.0002
";

    let expected = "\
client,available,held,total,locked
1,8.1233,0,8.1233,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn invalid_csv() {
    let input = "\
this,is,not,a,valid,transaction,file
hello,world
";

    let _error = run_program_expect_error(input);
}

#[test]
fn zero_amount_deposit() {
    let input = "\
type,client,tx,amount
deposit,1,1,0.00
";

    let expected = "\
client,available,held,total,locked
1,0,0,0,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn malformed_record_missing_amount() {
    let input = "\
type,client,tx,amount
deposit,1,1,
";

    let _error = run_program_expect_error(input);
}

#[test]
fn malformed_record_invalid_client() {
    let input = "\
type,client,tx,amount
deposit,not-a-client,1,10.00
";

    let _error = run_program_expect_error(input);
}

#[test]
fn malformed_record_invalid_transaction_id() {
    let input = "\
type,client,tx,amount
deposit,1,not-a-transaction,10.00
";

    let _error = run_program_expect_error(input);
}

#[test]
fn malformed_record_invalid_amount() {
    let input = "\
type,client,tx,amount
deposit,1,1,not-a-number
";

    let _error = run_program_expect_error(input);
}

#[test]
fn dispute_can_exceed_available_funds() {
    let input = "\
type,client,tx,amount
deposit,1,1,100.00
withdrawal,1,2,80.00
dispute,1,1,
";

    let expected = "\
client,available,held,total,locked
1,-80,100,20,false
";

    assert_eq!(run_program(input), expected);
}

#[test]
fn chargeback_can_result_in_negative_balance() {
    let input = "\
type,client,tx,amount
deposit,1,1,100.00
withdrawal,1,2,80.00
dispute,1,1,
chargeback,1,1,
";

    let expected = "\
client,available,held,total,locked
1,-80,0,-80,true
";

    assert_eq!(run_program(input), expected);
}
