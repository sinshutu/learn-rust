all: libembed.so
	python main.py

libembed.so:
	cargo build --release

clean:
	rm -rf target
