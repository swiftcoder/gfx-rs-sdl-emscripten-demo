
# What?

A port of the gfx-rs triangle demo, with the necessary scafolding to cross-compile to the web via emscripten.

# Why?

Although I've been able to run simple rust programs using sdl2 under emscripten, I have yet to successfully run an OpenGL-based program. This repository is intended to capture ongoing efforts to debug the process.

# How?

To build the sample, you will need to install a recent rust, and the portable emscripten SDK. To display the sample in a web browser, install nodejs and then `npm install http-server -g`. 

To run the sample on desktop, a simple `cargo run` will suffice.

To run the sample in a browser, run `make web` to build, then run `make serve` to setup a web server, and finally load <http://localhost:8080> in your web browser of choice.

