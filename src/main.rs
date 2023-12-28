use args::Arguments;
use clap::Parser;
use inflector::{self, Inflector};
use notify_rust::Notification;
use std::{path::{PathBuf, Path}, process::Command, fs::{self}};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

mod args;
mod errors;

const ICON: &str = "image-jpeg";
const NAME: &str = env!("CARGO_PKG_NAME");

struct ImageObject {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
    let config_dir = get_config_dir()?;

    let args = Arguments::parse();
    let mode = mode(args.mode);

    let client = reqwest::Client::new();
    let response: serde_json::Value = match client.get(api_url).send().await {
            Ok(response) => response.json().await.map_err(|_| errors::Error::ImageRequest(api_url.to_owned()))?,
            Err(_) => {
                if let Ok(custom_command) = read_config(&config_dir).await {
                    Command::new("sh").arg("-c").arg(custom_command).spawn()?;
                }

                return Ok(());
            }
        };        

    let image_object = response["images"].as_array().unwrap().first().unwrap();
    let image = ImageObject {
        title: image_object["title"].as_str().unwrap().to_string(),
        url: image_object["url"].as_str().unwrap().to_string(),
    };

    let image_url = format!("https://bing.com/{}", image.url);
    let destination = save_image(&image_url, &config_dir).await?;

    if let Some(custom_command) = args.custom_command {
        let command_arg = custom_command.replace("%", &destination.to_string_lossy().into_owned());
        write_config(&config_dir, &command_arg).await?; // Saves custom command
        Command::new("sh").arg("-c").arg(command_arg).spawn()?
    } else {
        Command::new("feh").arg(mode).arg(&destination).spawn()?
    };

    if !args.silent {
        send_notification(
            &NAME.to_pascal_case(),
            &format!("Wallpaper successfully Set.\nTitle: {0}", image.title),
            ICON,
        )?;
    }

    Ok(())
}

// Config Functions
fn get_config_dir() -> anyhow::Result<PathBuf> {
    if let Some(base_dirs) = directories::ProjectDirs::from("com", "thejayduck", "bismuth") {
        let config_dir = base_dirs.data_dir().to_path_buf();
        let _ = fs::create_dir_all(&config_dir);
        Ok(config_dir)
    } else {
        Err(anyhow::Error::msg("Failed to get project folder"))
    }
}

async fn write_config(config_dir: &Path, custom_command: &str) -> anyhow::Result<()> {
   let config_path = config_dir.join("config.txt");

    let mut file = tokio::fs::File::create(&config_path).await?;
    let mut command_reader = custom_command.as_bytes();
    
    tokio::io::copy(&mut command_reader, &mut file).await?;

    Ok(())
}

async fn read_config(config_dir: &Path) -> anyhow::Result<String> {
    let config_path = config_dir.join("config.txt");
    let custom_command = tokio::fs::read_to_string(&config_path).await?;

    Ok(custom_command)
}

async fn save_image(image_url: &String, config_dir: &Path) -> anyhow::Result<PathBuf> {
        let mut dir = config_dir.to_path_buf();
        let file_name = PathBuf::from(".wallpaper.jpg");

        let response = reqwest::get(image_url).await?;
        dir.push(file_name);
        let mut file = tokio::fs::File::create(&dir).await?;

        let content = response.bytes_stream().map(|result| {
            result.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
        });

        tokio::io::copy(&mut StreamReader::new(content), &mut file).await?;

        Ok(dir)
}

// Notification Function
fn send_notification(summary: &str, body: &str, icon: &str) -> anyhow::Result<()> {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .show()?;

    Ok(())
}

fn mode(value: args::Modes) -> &'static str {
    match value {
        args::Modes::Center => "--bg-center",
        args::Modes::Fill => "--bg-fill",
        args::Modes::Max => "--bg-max",
        args::Modes::Scale => "--bg-scale",
        args::Modes::Tile => "--bg-tile",
    }
}
