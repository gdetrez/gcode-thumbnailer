.PHONY: build
build:
	cargo build --bin=gcode-thumbnailer --bin=3mf-thumbnailer --release

.PHONY: install
install:
	install -d /usr/local/libexec /usr/local/share/thumbnailers/
	install -C target/release/3mf-thumbnailer /usr/local/libexec
	install -C target/release/gcode-thumbnailer /usr/local/libexec
	install -C target/release/bgcode-thumbnailer /usr/local/libexec
	install -C 3mf.thumbnailer /usr/share/thumbnailers/
	install -C gcode.thumbnailer /usr/share/thumbnailers/
	install -C bgcode.thumbnailer /usr/share/thumbnailers/
