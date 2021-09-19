use tokio::time::{sleep, Duration};
use exitfailure::ExitFailure;
use sit_network_auth::{check_network, login};
use structopt::StructOpt;

/// 网络登录
async fn login_auth(username: &str, password: &str) -> Result<(), ExitFailure> {

    //如果有网络
    if check_network().await? {
        println!("Network has connected!!!");
        return Ok(());
    }

    loop {
        //如果验证失败了
        if !check_network().await? {
            println!("Network auth failure detected.");
            println!("Login by user: {}", username);
            login(username, password).await?;
        } else {    //登录成功
            println!("Network has connected!!!");
            return Ok(());
        }
        sleep(Duration::from_secs(2)).await;
    }
}

#[derive(Debug, StructOpt)]
struct Input {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input: Input = Input::from_args();

    println!("sit network login running!!!");
    login_auth(input.username.as_str(),
               input.password.as_str())
        .await
        .unwrap_or_else(|_| {
            println!("Network error!!!");
        });

    Ok(())
}
