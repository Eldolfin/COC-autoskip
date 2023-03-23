# COC autoskip [![Build Status](https://jenkins.le-dauphin.tech/buildStatus/icon?subject=tests&?job=COC+autoskip+tests)](https://jenkins.le-dauphin.tech/job/COC%20autoskip%20tests/)

<!--toc:start-->
- [How to run it](#how-to-run-it)
- [Todo](#todo)
- [Purpose of this project](#purpose-of-this-project)
<!--toc:end-->

Using adb, this program
1. Asks the user the wanted gold and elixir (summed)
1. Starts searching
1. Skips until it sees a village fulfilling the requirements
1. Plays a sound effect once it finds one

It does it by taking a screenshot of the device, sending it to the computer
and processing it using the Tesseract OCR engine.

To use it, adb and Tesseract need to be installed, and the constants on top of the main file
need to be changed according to your screen resolution and aspect ratio.

This program is only made for Linux and has not been tested on other platforms.

Works just as well with wireless adb too!

## How to run it:
Install the rust toolchain first, then:
```
bash
sudo apt-get install libtesseract5 tesseract-ocr libtesseract-dev libasound2 libsdl2-dev clang build-essential pkg-config -y
git clone https://github.com/Eldolfin/COC-autoskip.git && cd COC-autoskip
cargo run --release
```

Refer to the [Tesseract doc](https://tesseract-ocr.github.io/tessdoc/Installation.html) and make sure to install version 5.
It might not work otherwise.

## TODO:
- [x] Add tests
- [x] Pass images to OCR without saving them on the disk
- [x] Add better logging, showing stats
- [x] Comment code parameters
- [x] Add desktop notifications
- [x] Add emojis and terminal colors
- [x] Allow user to skip village from android using volume buttons
- [x] Enhance initial number input by asking in units of 100k
- [ ] Add some sort of automatic adb connection

## Purpose of this project:
Spending less time playing, instead watching the computer play for you🗿

&nbsp;

Made with ❤  in Rust 🦀 btw
