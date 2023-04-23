use aws_sdk_account as account;
use aws_sdk_iam as iam;
use aws_sdk_sts as sts;
use serde::Serialize;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let shared_config = aws_config::load_from_env().await;

    let client_sts = sts::Client::new(&shared_config);
    let client_iam = iam::Client::new(&shared_config);
    let client_account = account::Client::new(&shared_config);

    let (acc_res, sts_res, ali_res) = tokio::join!(
        client_account.get_contact_information().send(),
        client_sts.get_caller_identity().send(),
        client_iam.list_account_aliases().send()
    );

    let alias = ali_res.expect("Failed to read alias from AWS");
    let contact_info = acc_res.expect("Failed to read account information from AWS");
    let contact_info = contact_info
        .contact_information()
        .expect("Contact information is missing from the response");
    let sts_res = sts_res.expect("STS request failed");

    println!(
        "{}",
        serde_json::to_string_pretty(&DisplayInformation {
            name: contact_info.full_name().unwrap_or("n/a").to_string(),
            arn: sts_res.arn().unwrap_or("n/a").to_string(),
            user_id: sts_res.user_id().unwrap_or("n/a").to_string(),
            account: sts_res.account().unwrap_or("n/a").to_string(),
            aliases: match alias.account_aliases() {
                Some(a) => a.to_owned(),
                None => vec![],
            }
        })
        .expect("Failed to stringify user data")
    );

    Ok(())
}

#[derive(Serialize)]
struct DisplayInformation {
    name: String,
    account: String,
    user_id: String,
    arn: String,
    aliases: Vec<String>,
}
