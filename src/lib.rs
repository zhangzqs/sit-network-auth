use reqwest::Url;
use exitfailure::ExitFailure;
use serde::{Serialize, Deserialize};
use tokio::time::{Duration};

const DEFAULT_UA_STRING: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:75.0) Gecko/20100101 Firefox/75.0";

#[derive(Debug, Serialize, Deserialize)]
struct MyResult {
    result: u8,
}

/// 发送Get请求，返回请求结果字符串
async fn send_get_request(url: Url) -> Result<String, ExitFailure> {
    return Ok(
        reqwest::Client::new()
            .get(url)
            .header("User-Agent", DEFAULT_UA_STRING)
            .timeout(Duration::from_secs(5))
            .send()
            .await?
            .text_with_charset("utf-8")
            .await?);
}

/// 检查是否联网成功
pub async fn check_network() -> Result<bool, ExitFailure> {
    let status_result = send_get_request(
        Url::parse("http://172.16.8.70/drcom/chkstatus?callback=dr1002&jsVersion=4.1&v=7808&lang=zh")
            .unwrap())
        .await?;
    let status_result = status_result.trim();
    let status_result = &status_result[7..];
    let status_result = &status_result[0..(status_result.len() - 1)];

    // println!("{}",status_result);
    let json_page: MyResult = serde_json::from_str(status_result).unwrap();

    Ok(json_page.result == 1)
}

/// 登录上网认证
pub async fn login(user: &str, password: &str) -> Result<bool, ExitFailure> {
    let login_url = "http://172.16.8.70/drcom/login";
    let login_parameter = [
        ("callback", "dr1003"),
        ("DDDDD", user),
        ("upass", password),
        ("0MKKey", "123456"),
        ("R1'", "0"),
        ("R2", ""),
        ("R3", "0"),
        ("R6", "0"),
        ("para", "00"),
        ("terminal_type", "1"),
        ("lang", "zh-cn"),
        ("jsVersion", "4.1"),
        ("v", "857"),
    ];
    let login_result = send_get_request(Url::parse_with_params(login_url,
                                                               login_parameter).unwrap()).await?;
    let login_result = login_result.trim(); //切除首尾空格
    let login_result = &login_result[7..];   //切除头部7个字符
    let login_result = &login_result[0..(login_result.len() - 1)];  //切除尾部一个字符

    // println!("{}",login_result);
    let json_page: MyResult = serde_json::from_str(login_result).unwrap();

    Ok(json_page.result == 1)
}