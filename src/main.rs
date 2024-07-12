use std::fs;
use std::process::{Command, exit};
use dialoguer::{Select, theme::ColorfulTheme};
use clap::{Arg, Command as ClapCommand};

fn main() {
    // 使用 clap 解析命令行参数
    let matches = ClapCommand::new("gopen")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("Open Git repository remote URL in your browser")
        .arg(Arg::new("directory")
            .default_value(".")
            .index(1)
            .help("The directory of the Git repository"))
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Interactive mode to select remote URL")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // 获取目录参数
    let directory = matches.get_one::<String>("directory").unwrap();

    // 检查目录是否存在
    if !fs::metadata(directory).is_ok() {
        eprintln!("Directory does not exist: {}", directory);
        exit(1);
    }

    // 获取所有 remote URLs
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .arg("remote")
        .arg("-v")
        .output()
        .expect("failed to execute git command");

    if !output.status.success() {
        eprintln!("Failed to get remote URL, please make sure you are running this script in a Git repository.");
        exit(1);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let remote_urls: Vec<&str> = output_str
        .lines()
        .filter(|line| line.contains("(fetch)"))
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();

    // 检查是否成功获取 remote URL
    if remote_urls.is_empty() {
        eprintln!("No remote URL found, please make sure you are running this script in a Git repository.");
        exit(1);
    }

    // 检查是否输入了 -i 参数
    let interactive = matches.get_flag("interactive");

    // 如果是交互模式，显示选择界面
    let remote_url = if interactive {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select a remote URL")
            .default(0)
            .items(&remote_urls)
            .interact()
            .expect("failed to read input");

        remote_urls[selection]
    } else {
        // 默认选择第一个 remote URL
        remote_urls[0]
    };

    // 如果是 SSH URL，转换为 HTTPS URL
    let remote_url = if remote_url.starts_with("git@") {
        remote_url.replacen("git@", "https://", 1).replace(':', "/")
    } else if remote_url.starts_with("ssh://git@") {
        remote_url.replacen("ssh://git@", "https://", 1)
    } else {
        remote_url.to_string()
    };

    // 确保 remote_url 以 "https://" 开头
    let remote_url = if !remote_url.starts_with("https://") {
        format!("https://{}", remote_url)
    } else {
        remote_url
    };

    // 在浏览器中打开 remote URL
    Command::new("open")
        .arg(remote_url)
        .status()
        .expect("failed to open URL");
}
