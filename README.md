
<p align="center">
  <img width="500" alt="RustInfoInvest Logo" src="assets/icon/RustInfoInvest_LOGO_WITHOUT_BG.png">
</p>

# RustInfoInvest
> A Very Simple And Configurable Compound Interest Calculator With Online Selic Rate Values And Realtime Currency View, Writed In Rust. 

<br/>
<br/>
<br/>

## **Showcase**
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/40825d0d-c4be-461c-8f21-4e5168540b8c" />
<details> <summary>Others Pages</summary>

- Realtime Currency Page
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/cdc837dc-6566-4ad2-a81c-cc36c9c4bf3e" />

- Investment Wallet Page
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/426737c6-ef7a-45d8-a563-4b040589b169" />

- Selic Historic Page
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/e271d895-0ac6-41a2-8720-feb6eca3634f" />
</details>

<br/>
<br/>
<br/>

## **Known Issues**
- hard to read when window is small
- messy code

<br/>
<br/>
<br/>

## **Future Plans**
- make the code right
- make the code fast

<br/>
<br/>
<br/>

## **Dependencies**
**to use all features in the app you will need the followings packages: "sdl3_ttf (Only In AUR), ttf-jetbrains-mono, pkg-config, base-devel, cmake, geckodriver and firefox"**
  
| Arch Linux |
| --- |
| `sudo pacman -Sy ttf-jetbrains-mono pkg-config base-devel cmake geckodriver firefox` |
| --- |
| `yay -S sdl3_ttf` or `paru -S sdl3_ttf` |

**for basic features you will need to have only: "sdl3_ttf (Only In AUR), ttf-jetbrains-mono, pkg-config, base-devel and cmake"**
  
| Arch Linux |
| --- |
| `sudo pacman -Sy ttf-jetbrains-mono pkg-config base-devel cmake` |
| --- |
| `yay -S sdl3_ttf` or `paru -S sdl3_ttf` |

<br/>
<br/>
<br/>

## **Downloading And Running**
- Go to the [Github Release Page](https://github.com/HaruNashii/RustInfoInvest/releases)
- Download The App Packages "RustInfoInvest_x86_64_Linux.zip"
- Unzip In Your Desired Location
- Give It Permission To Run (Can be done with "chmod a+x 'name_of_the_app'")
- Run It And Enjoy!! :).

<br/>
<br/>
<br/>

## **Building And Running**
**after downloading the dependencies, you can build the app while on the ```root folder``` of the project with:**

| Bash |
| --- |
| `cargo build --release` |

**for running, you can direct run the app in the ```target/release``` folder, or if isn't builded yet, you can run the following command:**

| Bash |
| --- |
| `cargo run --release` |
