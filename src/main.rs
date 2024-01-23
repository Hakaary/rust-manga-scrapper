use clap::Parser;
use futures::future::try_join_all;
use reqwest;
use std::{
    ops::RangeInclusive,
    process::{exit, Command, Stdio},
};
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};
use tokio::{fs, io::AsyncWriteExt, task};

mod args;
mod folders;

use args::Args;
use folders::{create_chapter_folder, create_main_folder};

async fn gen_manga_chapters(
    driver: WebDriver,
    chapters: RangeInclusive<u32>,
    manga_name: String,
    manga_url: String,
) -> Result<Vec<tokio::task::JoinHandle<()>>, reqwest::Error> {
    let mut f_write_imgs = Vec::new();

    for chapter in chapters {
        // Create the folder for the chapter if it doesn't exist
        // If exists, continue. That chapter is assumed to be a chapter
        // that is already downloaded.
        if let Err(()) = create_chapter_folder(manga_name.clone().as_str(), chapter).await {
            continue;
        }

        driver
            .goto(format!("{}{}/#1", manga_url, chapter,))
            .await
            .unwrap();

        let mut current_page: u8 = 1;

        print!("Downloading chapter {}... ", chapter);

        loop {
            // Download page
            if let Some(img_url) = driver
                .find(By::ClassName("lazy"))
                .await
                .unwrap()
                .attr("src")
                .await
                .unwrap()
            {
                let mut img_file =
                    fs::File::create(format!("./{}/{}/{}.png", manga_name, chapter, current_page))
                        .await
                        .unwrap();
                let task = task::spawn(async move {
                    let img = reqwest::get(img_url);
                    let b_img = img.await.unwrap().bytes().await.unwrap();
                    img_file.write_all(&b_img).await.unwrap();
                });
                f_write_imgs.push(task);
            }

            // Click to go to next page
            driver
                .execute_async(
                    r#"
                        let done = arguments[0];
                        $("selector").ready(() => {
                            done(document.getElementsByClassName("img_land_next")[0].click());
                        });
                        "#,
                    Vec::new(),
                )
                .await
                .unwrap();

            // Check if last page
            if let Some(url_current_page) = driver.current_url().await.unwrap().fragment() {
                if let Ok(url_current_page) = url_current_page.parse::<u8>() {
                    if current_page >= url_current_page {
                        println!("{} pages", current_page);
                        break;
                    } else {
                        current_page = url_current_page;
                    }
                }
            }
        }
    }

    Ok(f_write_imgs)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Arguments
    let args = Args::parse();

    let chr_driver = Command::new("chromedriver").stdout(Stdio::null()).spawn();

    if let Err(e) = chr_driver {
        eprintln!("Chromedriver not found. Error: {}", e);
        exit(1);
    }

    // Download process
    let manga_url = format!("https://www.leercapitulo.com/leer/abcdef/{}/", {
        let manga_name = args.manga.to_lowercase();
        manga_name.replace(" ", "-")
    });

    let chapters: RangeInclusive<u32>;
    if args.number != 0 {
        chapters = args.number..=args.number;
        println!("Downloading '{}' chapter {}", args.manga, args.number);
    } else {
        chapters = args.from..=args.to;
        println!(
            "Downloading '{}' chapters from {} to {}",
            args.manga, args.from, args.to
        );
    }

    // Create the folder for the manga if it doesn't exist
    create_main_folder(args.manga.clone().as_str())
        .await
        .unwrap();

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    caps.set_headless()?;

    if let Ok(driver) = WebDriver::new("http://localhost:9515", caps.clone()).await {
        let f_write_imgs =
            gen_manga_chapters(driver.clone(), chapters, args.manga.clone(), manga_url)
                .await
                .unwrap();
        try_join_all(f_write_imgs.into_iter()).await.unwrap();
        driver.quit().await?;
        println!("Chapter/s downloaded!");
    } else {
        println!("Make sure chromedriver (port 9515) and Chrome is installed and running.");
    }

    Ok(())
}
