use std::process::Command;

use whoiam::DisplayInformation;

#[test]
fn whoiam_integration_test() {
    let mut whoiam = Command::new("./target/release/whoiam");
    let result = String::from_utf8(whoiam.output().unwrap().stdout).unwrap();
    let account_information = serde_json::from_str::<DisplayInformation>(&result).unwrap();

    assert_eq!(account_information.name, "joel.roxell.sandbox");
    assert!(!account_information.arn.is_empty());
    assert!(!account_information.user_id.is_empty());
    assert_eq!(account_information.aliases.len(), 1);
}
