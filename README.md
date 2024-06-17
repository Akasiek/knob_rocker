<div style="text-align: center">
    <img src="icons/icon.ico"/>
</div>

# Knob Rocker

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=orange)
![Spotify](https://img.shields.io/badge/Spotify-1ED760?style=for-the-badge&logo=spotify&logoColor=white)

Windows Desktop application for controlling the volume of Spotify using Keyboard Knob. 

It uses the Spotify API to change internal Spotify volume, not the system volume. This way, you can control the
volume of Spotify independently of the system volume. Also works for other devices that are connected to the same
account.

## How to use

Control the volume of Spotify using F21 and F22 keys. Those were chosen because they're rarely used in other 
applications and can be easily bound to keyboard's knob.

### Configuration

First, you need to create a Spotify application in the Spotify Developer Dashboard. You can do that by following 
[this link](https://developer.spotify.com/dashboard/applications).

Choose 'Web Playback SDK' and 'Web API'. If you want to use Android or iOS devices, you can choose API keys for those
platforms as well, but they're not necessary.

After creating the application, you'll get a Client ID and a Client secret. You will be asked to provide them when
you run the application for the first time. Also, you'll need to add `http://localhost/` as a redirect URI in the 
Spotify Developer Dashboard.

After entering the Client ID and Secret and API authorization, it will create two files in 
`%APPDATA%\.config\knob_rocker` directory. Those files store the Client ID and Secret, as well as the cache API token. 

## How to build

1. Clone the repository
2. Change the current directory to the repository's root
3. Run `cargo build --release`
4. The executable will be in `target/release/knob_rocker.exe`

## Supported platforms

Only Windows is supported at the moment. Linux could be implemented if not for the Wayland way of handling global 
hotkeys. MacOS isn't really a priority for me, but I'm open to testing and merging PRs.
