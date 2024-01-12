use reqwest;
use std::{fs, fs::File};
use std::io::prelude::*;
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};

async fn downloader(driver: &WebDriver, manga_name: &str, chapter: u32) -> WebDriverResult<()> {
    if !fs::metadata(format!("./{}/{}", manga_name, chapter)).map(|metadata| metadata.is_dir()).unwrap_or(false) {
        fs::DirBuilder::new().create(format!("./{}/{}", manga_name, chapter))?;
    } else {
        return Ok(());
    }

    driver
        .goto(format!(
            "https://www.leercapitulo.com/leer/5pljav/one-piece/{}/#1",
            chapter
        ))
        .await?;

    let mut current_page: u8 = 1;

    loop {
        // Download page
        if let Some(img_url) = driver
            .find(By::ClassName("lazy"))
            .await?
            .attr("src")
            .await?
        {
            let img = reqwest::get(img_url).await.unwrap().bytes().await.unwrap();
            let mut img_file = File::create(format!("./{}/{}/{}.png", manga_name, chapter, current_page))?;
            img_file.write_all(&img)?;
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
            .await?;

        // Check if last page
        if let Some(url_current_page) = driver.current_url().await?.fragment() {
            if let Ok(url_current_page) = url_current_page.parse::<u8>() {
                if current_page == url_current_page {
                    return Ok(());
                } else {
                    current_page = url_current_page;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    let manga_name = "One Piece";

    if !fs::metadata(manga_name).map(|metadata| metadata.is_dir()).unwrap_or(false) {
        fs::DirBuilder::new().create(format!("./{}", manga_name))?;
    }

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    caps.set_headless()?;

    if let Ok(driver) = WebDriver::new("http://localhost:9515", caps.clone()).await {
        for i in 340..=500 {
            match downloader(&driver, &manga_name, i).await {
                Ok(()) => {
                    println!("Downloaded succesfully -> {}: {}", manga_name, {i});
                }
                Err(e) => {
                    println!("Error downloading {:?}", e);
                }
            }
        }
        driver.quit().await?;
    } else {
        println!("No driver (chromedriver) found. Make sure it is installed and running.");
    }

    Ok(())
}
