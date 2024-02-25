build:
	bash build.sh

clean:
	rm -rf output

# run-debug:
rd:
	RUST_BACKTRACE=1 cargo run

# run-release:
rr:
	cd output && bash run.sh

fmt:
	rustfmt ./src/main.rs -v
