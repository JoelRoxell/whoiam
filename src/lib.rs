use aws_config::SdkConfig;
use aws_sdk_account as account;
use aws_sdk_iam as iam;
use aws_sdk_sts as sts;
use serde::{Deserialize, Serialize};

static NA: &str = "n/a";

pub async fn read_aws_information(is_sts: bool) -> DisplayInformation {
    let shared_config = aws_config::load_from_env().await;

    if is_sts {
        let (arn, user_id, account) = read_sts_data(&shared_config).await;

        DisplayInformation::new(NA.to_string(), account, user_id, arn, vec![])
    } else {
        let ((arn, account, user_id), user_name, aliases) = tokio::join!(
            read_sts_data(&shared_config),
            read_account_data(&shared_config),
            read_iam_data(&shared_config),
        );

        DisplayInformation::new(user_name, account, user_id, arn, aliases)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayInformation {
    pub name: String,
    pub account: String,
    pub user_id: String,
    pub arn: String,
    pub aliases: Vec<String>,
}

impl DisplayInformation {
    fn new(
        name: String,
        account: String,
        user_id: String,
        arn: String,
        aliases: Vec<String>,
    ) -> Self {
        Self {
            arn,
            name,
            account,
            user_id,
            aliases,
        }
    }
}

async fn read_sts_data(c: &SdkConfig) -> (String, String, String) {
    let client_sts = sts::Client::new(c);
    let sts_data = client_sts.get_caller_identity().send().await;

    match sts_data {
        Ok(d) => (
            d.arn().to_owned().unwrap_or(NA).to_string(),
            d.user_id().unwrap_or(NA).to_string(),
            d.account().unwrap_or(NA).to_string(),
        ),
        Err(e) => {
            eprintln!("STS request failed::{:?}", e);

            (NA.to_string(), NA.to_string(), NA.to_string())
        }
    }
}

async fn read_iam_data(c: &SdkConfig) -> Vec<String> {
    match iam::Client::new(c).list_account_aliases().send().await {
        Ok(a) => {
            let a = a.account_aliases().unwrap_or_default();

            a.to_vec()
        }
        Err(e) => {
            eprintln!("Failed to list account aliases::{:?}", e);

            vec![]
        }
    }
}

async fn read_account_data(c: &SdkConfig) -> String {
    match account::Client::new(c)
        .get_contact_information()
        .send()
        .await
    {
        Ok(c) => match c.contact_information() {
            Some(c) => c.full_name().unwrap_or(NA).to_string(),
            None => NA.to_string(),
        },
        Err(e) => {
            eprintln!("Failed to read contact information::{:?}", e);

            NA.to_string()
        }
    }
}
