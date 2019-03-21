PLOT_NUM = 500000000
JOBS = 0
WINDOW_SIZE = 10000000
RELEASE_EXE = ./target/release/montecarlo-pi

ifeq ($(OS),Windows_NT)
	RELEASE_EXE = ./target/release/montecarlo-pi.exe
endif

.PHONY: debug
debug:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: bench
bench: release
	time $(RELEASE_EXE) serial -n $(PLOT_NUM) && time $(RELEASE_EXE) parallel -n $(PLOT_NUM) -j $(JOBS) -w $(WINDOW_SIZE)
