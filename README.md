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

It's not 100% accurate at the moment, it might skip villages it shouldn't.
But it's fairly rare.

## TODO:
- [ ] Add desktop notifications
- [ ] Add better logging, showing stats
- [ ] Pass images to OCR without saving them on the disk
- [ ] Add tests

## Purpose of this project:
Spending less time playing, instead watching the computer play for you🗿

&nbsp;

Made with ❤  in Rust 🦀 btw
