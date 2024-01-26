
# Rust Manga Scrapper

## Description
This program allows you to scrap manga chapters from [leercapitulo.com](https://www.leercapitulo.com/) in a simple and automated way. It uses Selenium to control a Chrome browser and Reqwest to download chapter images.

## Requirements
- [Chromedriver](https://sites.google.com/chromium.org/driver/) installed and in the PATH.
- [Google Chrome](https://www.google.com/chrome/) installed and compatible with previous Chromedriver.
- [Rust](https://www.rust-lang.org/tools/install) installed (to compile).

## Installation
1. Clone this repository:
   ```
   git clone https://github.com/Hakaary/rust_manga_scrapper.git
   cd MangaDownloader
   cargo build --release
   ```

## Usage

Run the program from the command line, providing the necessary arguments. For example, assuming the binary is this path:

```
./target/release/manga_downloader --manga "Fullmetal Alchemist" --from 1 --to 5
```

![Screenshot](read_imgs/example_1.png)

```
./target/release/manga_downloader -m death\ note -n 1
```

![Screenshot](read_imgs/example_2.png)


