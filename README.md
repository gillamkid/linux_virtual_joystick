# Linux Virtual Joystick
Run a virtual joystick on your computer! 

![Alt text](docs/app-screenshot.png)

Models a UXV SRoC device.

I use this when working with QGroundControl, which requires a joystick to be connected for certain actions. 

## Run prebuilt appimage
Download latest appimage from github Release page (https://github.com/gillamkid/linux_virtual_joystick/releases/)
```
sudo chmod +x linux_virtual_joystick.AppImage
sudo ./linux_virtual_joystick.AppImage
```

## Doing a Local Build
To compile from source you will need [cargo and rust](https://www.rust-lang.org/tools/install).To install from source clone this repository and `cargo build`.
To start the program `sudo -E ~/.cargo/bin/cargo run`.

## Build an Appimage
```
cargo install cargo-appimage
wget https://github.com/AppImage/appimagetool/releases/download/1.9.0/appimagetool-x86_64.AppImage
sudo mv appimagetool-x86_64.AppImage /usr/local/bin/appimagetool
sudo -E ~/.cargo/bin/cargo appimage
```

## Notes
This is a fork of https://github.com/abezukor/linux_virtual_joystick
