use whoiam::{read_aws_information, AwsError};

#[tokio::main]
async fn main() -> Result<(), AwsError> {
    let info = read_aws_information().await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&info).expect("Failed to stringify user data")
    );

    Ok(())
}
