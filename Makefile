
build:
	cd src && cargo build --release

test:
	cd src && cargo test --release

render-scenes:
	cd scenes && ./render.sh release

clean:
	cd src && cargo clean --release


build-dev:
	cd src && cargo build

test-dev:
	cd src && cargo test

render-scenes-dev:
	cd scenes && ./render.sh debug

clean-dev:
	cd src && cargo clean
