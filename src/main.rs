use std::fs;
extern crate dirs;
use std::process::Command;

#[tokio::main]
async fn main() {
    let ip = reqwest::get("http://inet-ip.info/ip")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut path = dirs::home_dir().unwrap();
    path.push(".global-ip");
    let pre_ip = fs::read_to_string(&path);
    let pre_ip = match pre_ip {
        Ok(ip) => ip,
        Err(_error) => "".to_string(),
    };

    if pre_ip != ip {
        fs::write(&path, ip).ok();
        println!("update ip");
        Command::new("/usr/local/bin/terminal-notifier")
            .args(&[
                "-title",
                "通知",
                "-message",
                "グローバルIPが更新されました。GAのフィルターを更新してください",
                "-open",
                "https://analytics.google.com/analytics/web/",
            ])
            .output()
            .expect("msg");
    }
}
