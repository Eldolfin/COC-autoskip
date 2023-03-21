# COC autoskip

<!--toc:start-->
- [COC autoskip](#coc-autoskip)
  - [Todo](#todo)
  - [Purpose of this project:](#purpose-of-this-project)
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

## TODO:
- [x] Add tests
- [x] Pass images to OCR without saving them on the disk
- [x] Add better logging, showing stats
- [x] Comment code parameters
- [x] Add desktop notifications
- [x] Add emojis and terminal colors

## Purpose of this project:
Spending less time playing, instead watching the computer play for you🗿

&nbsp;

Made with ❤  in Rust 🦀 btw
