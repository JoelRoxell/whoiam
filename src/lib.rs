use account::error::ProvideErrorMetadata;
use aws_sdk_account as account;
use aws_sdk_iam as iam;
use aws_sdk_sts as sts;
use serde::{Deserialize, Serialize};

pub async fn read_aws_information() -> Result<DisplayInformation, AwsError> {
    let shared_config = aws_config::load_from_env().await;

    let client_sts = sts::Client::new(&shared_config);
    let client_account = account::Client::new(&shared_config);
    let client_iam = iam::Client::new(&shared_config);

    let (acc_res, sts_res, iam_res) = tokio::join!(
        client_account.get_contact_information().send(),
        client_sts.get_caller_identity().send(),
        client_iam.list_account_aliases().send()
    );

    let alias = iam_res.map_err(|e| {
        AwsError::IAMError(
            e.message()
                .unwrap_or("Failed to list account aliases")
                .to_owned(),
        )
    })?;
    let sts_res = sts_res
        .map_err(|e| AwsError::STSError(e.message().unwrap_or("STS request failed").to_owned()))?;
    let contact_info = acc_res.map_err(|e| {
        AwsError::AccoutError(
            e.message()
                .unwrap_or("Failed to read account information from AWS")
                .to_owned(),
        )
    })?;

    Ok(DisplayInformation {
        name: match contact_info.contact_information() {
            Some(contact) => contact.full_name().unwrap_or(""),
            None => "",
        }
        .to_string(),
        arn: sts_res.arn().to_owned().unwrap_or("").to_string(),
        user_id: sts_res.user_id().unwrap_or("").to_string(),
        account: sts_res.account().unwrap_or("").to_string(),
        aliases: match alias.account_aliases() {
            Some(a) => a.to_owned(),
            None => vec![],
        },
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayInformation {
    pub name: String,
    pub account: String,
    pub user_id: String,
    pub arn: String,
    pub aliases: Vec<String>,
}

#[derive(Debug)]
pub enum AwsError {
    IAMError(String),
    STSError(String),
    AccoutError(String),
}
