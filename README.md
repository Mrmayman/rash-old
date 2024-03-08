# Update
This project is abandoned, old and outdated. I am rewriting the whole thing with a cleaner design, and will publish it once ready.

# Rash (old version)
A fast Scratch interpreter written in rust. This is the old legacy version.

The new version will release soon.

# Setup and usage
Until this is stable for proper use, you have to compile it from source.

Just git clone this or download it as zip. Make sure you have rust installed, and navigate to the directory in terminal. Then do `cargo run -- path/to/your/file.sb3`

There are a few test sb3 files in the tests directory. For more information, go to tests/README.md

# Goals and progress
My goal is to be faster than TurboWarp, while maintaining decent compatibility. Currently it is on average 1.6x slower than TurboWarp as it is a simple bytecode interpreter, but I am planning to add things like advanced GPU acceleration, Just-In-Time compilation and much more.

This project is in the "Get it working" stage of development. The code is terrible and ugly. Once it works reasonably well, I can move on to making it cleaner and faster.

[Here is a document highlighting my progress on implementing features](https://docs.google.com/spreadsheets/d/1jYi5lsAyq6XeJPCKCpk4UkqF1YWVPX9C4d7-eTbXw9U/edit?usp=sharing)

# Credits
- Massive thanks to the creator of [svg2colored-png](https://github.com/MCorange99/svg2colored-png)! I couldn't figure out how to load SVG costumes but thanks to this, I can convert them to PNG.
- Further thanks to the authors of all the libraries I used.
- Also huge credit to the people in the Griffpatch Discord server for helping me understand how Scratch works.
