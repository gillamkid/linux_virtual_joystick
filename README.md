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

## Build from Source
```
# dependencies
sudo apt install git curl build-essential -y

# this installs rust/cargo as instructed on https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# restart terminal so cargo is added to PATH

# get source code
git clone https://github.com/gillamkid/linux_virtual_joystick.git
cd linux_virtual_joystick

# build and run
cargo build
sudo -E ~/.cargo/bin/cargo run
```

#### Build an Appimage
```
# dependencies
cargo install cargo-appimage
wget https://github.com/AppImage/appimagetool/releases/download/1.9.0/appimagetool-x86_64.AppImage
sudo mv appimagetool-x86_64.AppImage /usr/local/bin/appimagetool
sudo chmod +x /usr/local/bin/appimagetool

# build
sudo -E ~/.cargo/bin/cargo appimage

# the appimage can be found at target/appimage/linux_virtual_joystick.AppImage
```

## Notes
This is a fork of https://github.com/abezukor/linux_virtual_joystick
