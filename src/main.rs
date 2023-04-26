use clap::{Arg, ArgAction, Command};
use whoiam::read_aws_information;

/// This is a simple cli to dump useful aws-account information to stdout
#[tokio::main]
async fn main() {
    let matches = Command::new("whoiam")
        .version("0.1.1")
        .about(
            r#"This CLI dumps some useful information about your AWS-account to stdout.

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
                .help("Force the cli to only retrieve info from the sts api"),
        )
        .get_matches();

    let info = read_aws_information(matches.get_flag("sts")).await;

    println!(
        "{}",
        serde_json::to_string_pretty(&info).expect("Failed to stringify user data")
    );
}
