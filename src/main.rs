use clap::Parser;
use reqwest;
use std::{ops::RangeInclusive, collections::HashMap};
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};
use tokio::{fs, io::AsyncWriteExt, task};

mod args;
mod folders;

use args::Args;
use folders::{create_chapter_folder, create_main_folder};

async fn gen_manga_chapters(
    driver: WebDriver,
    chapters: RangeInclusive<u32>,
    manga_name: &str,
    manga_url: &str,
) -> Result<HashMap<u32, tokio::task::JoinHandle<()>>, reqwest::Error> {
    let mut f_write_imgs = HashMap::new();

    for chapter in chapters {
        driver
            .goto(format!("{}{}/#1", manga_url, chapter,))
            .await
            .unwrap();

        let mut current_page: u8 = 1;

        // Create the folder for the chapter
        if let Err(()) = create_chapter_folder(manga_name, chapter).await {
            continue;
        }

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
                f_write_imgs.insert(chapter, task);
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
                    if current_page == url_current_page {
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

    // Download process
    let manga_name = "One Piece";
    let manga_url = "https://www.leercapitulo.com/leer/5pljav/one-piece/";

    let chapters: RangeInclusive<u32>;
    if args.number != 0 {
        chapters = args.number..=args.number;
        println!("Downloading chapter {}", args.number);
    } else {
        chapters = args.from..=args.to;
        println!("Downloading chapter from {} to {}", args.from, args.to);
    }
    println!();

    // Create the folder for the manga
    create_main_folder(manga_name).await.unwrap();

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    caps.set_headless()?;

    if let Ok(driver) = WebDriver::new("http://localhost:9515", caps.clone()).await {
        let f_write_imgs = gen_manga_chapters(driver.clone(), chapters, manga_name, manga_url)
            .await
            .unwrap();
        for (cha, f_write_img) in f_write_imgs {
            f_write_img.await.unwrap();
            println!("Chapter {} downloaded", cha);
        }
        driver.quit().await?;
        println!();
        println!("Manga '{}' downloaded!", manga_name);
    } else {
        println!("No driver (chromedriver) found. Make sure it is installed and running.");
    }

    Ok(())
}
