use clap::{Arg, ArgAction, Command};
use whoiam::collect_aws_information;

/// This is a simple cli to dump useful aws-account information to stdout
#[tokio::main]
async fn main() {
    let matches = Command::new("whoiam")
        .version("0.1.1")
        .about(
            r#"This CLI dumps some useful information about your AWS-account to stdout.
Information is retrieved from STS, IAM, and AWS-SDK-Account by default.

Created by:

    $$$$$$$$\ $$\                      
    $$  _____|$$ |                     
    $$ |      $$ |$$\    $$\  $$$$$$\  
    $$$$$\    $$ |\$$\  $$  | \____$$\ 
    $$  __|   $$ | \$$\$$  /  $$$$$$$ |
    $$ |      $$ |  \$$$  /  $$  __$$ |
    $$$$$$$$\ $$ |   \$  /   \$$$$$$$ |
    \________|\__|    \_/     \_______|


> Work with us? 
> Send a mail to career@elva-group.com or contact someone in the team directly!
        "#,
        )
        .arg(
            Arg::new("sts")
                .short('s')
                .long("sts")
                .action(ArgAction::SetTrue)
                .help("Only retrieve info from the sts api"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Outputs err messages to stderr"),
        )
        .get_matches();

    let info = collect_aws_information(matches.get_flag("sts"), matches.get_flag("verbose")).await;

    println!(
        "{}",
        serde_json::to_string_pretty(&info).expect("Failed to stringify user data")
    );
}
