
build:
	cargo build

test:
	cargo test

check:
	CEL_CXX_FFI_SKIP_BUILD=1 cargo check

docs:
	scripts/docs.sh

BEAR_CMD := bear --force-preload -- cargo build

compile_commands:
	@type bear > /dev/null 2>&1 \
		|| { echo "bear is not installed, please install it: https://github.com/rizsotto/Bear"; exit 1; } \
		&& { echo "$(BEAR_CMD)"; $(BEAR_CMD); }

clean:
	cargo clean
	rm -f compile_commands.json compile_commands.events.json

