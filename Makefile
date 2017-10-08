
PROJECT := "gfx-rs-sdl-emscripten-demo"
EMMAKEN_CFLAGS := "-s USE_SDL=2"

all:

build-debug:
	EMMAKEN_CFLAGS=$(EMMAKEN_CFLAGS) cargo build --target asmjs-unknown-emscripten
	cp target/asmjs-unknown-emscripten/debug/$(PROJECT).js html/$(PROJECT).js

build-release:
	EMMAKEN_CFLAGS=$(EMMAKEN_CFLAGS) cargo build --target asmjs-unknown-emscripten --release
	cp target/asmjs-unknown-emscripten/release/$(PROJECT).js html/$(PROJECT).js

web: build-debug
	gzip -f html/$(PROJECT).js

web-release: build-release
	gzip -f html/$(PROJECT).js

serve:
	http-server html/ -g -c-1 -p 8080

clean:
	rm -f html/$(PROJECT).js.gz

