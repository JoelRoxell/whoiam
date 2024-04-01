use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_account as account;
use aws_sdk_iam as iam;
use aws_sdk_sts as sts;
use serde::{Deserialize, Serialize};

pub async fn collect_aws_information(is_sts: bool, is_verbose: bool) -> DisplayInformation {
    let shared_config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;

    if is_sts {
        let (arn, user_id, account) = retrieved_sts_data(&shared_config, &is_verbose).await;

        DisplayInformation::new(EMPTY_STR.to_string(), account, user_id, arn, vec![])
    } else {
        let ((arn, account, user_id), user_name, aliases) = tokio::join!(
            retrieved_sts_data(&shared_config, &is_verbose),
            retrieved_account_data(&shared_config, &is_verbose),
            retrieved_iam_data(&shared_config, &is_verbose),
        );

        DisplayInformation::new(user_name, account, user_id, arn, aliases)
    }
}

async fn retrieved_sts_data(c: &SdkConfig, verbose: &bool) -> (Arn, UserId, Account) {
    let client_sts = sts::Client::new(c);
    let sts_data = client_sts.get_caller_identity().send().await;

    match sts_data {
        Ok(d) => (
            d.arn().to_owned().unwrap_or(EMPTY_STR).to_string(),
            d.user_id().unwrap_or(EMPTY_STR).to_string(),
            d.account().unwrap_or(EMPTY_STR).to_string(),
        ),
        Err(e) => {
            if *verbose {
                eprintln!("STS request failed::{:?}", e);
            }

            (
                EMPTY_STR.to_string(),
                EMPTY_STR.to_string(),
                EMPTY_STR.to_string(),
            )
        }
    }
}

async fn retrieved_iam_data(c: &SdkConfig, verbose: &bool) -> Vec<String> {
    match iam::Client::new(c).list_account_aliases().send().await {
        Ok(a) => a.account_aliases().to_vec(),
        Err(e) => {
            if *verbose {
                eprintln!("Failed to list account aliases::{:?}", e);
            }

            vec![]
        }
    }
}

async fn retrieved_account_data(c: &SdkConfig, verbose: &bool) -> String {
    match account::Client::new(c)
        .get_contact_information()
        .send()
        .await
    {
        Ok(c) => match c.contact_information() {
            Some(c) => c.full_name().to_string(),
            None => EMPTY_STR.to_string(),
        },
        Err(e) => {
            if *verbose {
                eprintln!("Failed to read contact information::{:?}", e);
            }

            EMPTY_STR.to_string()
        }
    }
}

static EMPTY_STR: &str = "";

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayInformation {
    pub name: String,
    pub account: Account,
    pub user_id: UserId,
    pub arn: Arn,
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

type Arn = String;
type UserId = String;
type Account = String;
