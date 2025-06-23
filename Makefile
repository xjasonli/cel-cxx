
build:
	cargo build

test:
	cargo test

check:
	cargo check

docs:
	scripts/docs.sh

compile_commands:
	scripts/update_compile_commands.sh

clean:
	cargo clean
	rm -f compile_commands.json compile_commands.events.json

