
build:
	cargo build

test:
	cargo test --all-targets

check:
	CEL_CXX_FFI_SKIP_BUILD=1 cargo check

docs:
	@( \
		type cargo-docs-rs > /dev/null 2>&1 || \
			( \
				echo "cargo-docs-rs is not installed, Installing it..."; cargo install cargo-docs-rs; \
			) \
	) && \
	( \
		echo "Generating documentation for cel-build-utils..."; \
		cargo +nightly docs-rs -p cel-build-utils \
	) && \
	( \
		echo "Generating documentation for cel-cxx-macros..."; \
		cargo +nightly docs-rs -p cel-cxx-macros \
	) && \
	( \
		echo "Generating documentation for cel-cxx-ffi..."; \
		cargo +nightly docs-rs -p cel-cxx-ffi \
	) && \
	( \
		echo "Generating documentation for cel-cxx..."; \
		cargo +nightly docs-rs -p cel-cxx \
	) && \
	echo "Documentation generated successfully!"

publish:
	@( \
		echo "Publishing cel-build-utils..."; \
		cargo publish -p cel-build-utils \
	) && \
	( \
		echo "Publishing cel-cxx-macros..."; \
		cargo publish -p cel-cxx-macros \
	) && \
	( \
		echo "Publishing cel-cxx-ffi..."; \
		cargo publish -p cel-cxx-ffi \
	) && \
	( \
		echo "Publishing cel-cxx..."; \
		cargo publish -p cel-cxx \
	) && \
	echo "All crates published successfully!"


BEAR_CMD := bear --force-preload -- cargo build

compile_commands:
	@type bear > /dev/null 2>&1 \
		|| { echo "bear is not installed, please install it: https://github.com/rizsotto/Bear"; exit 1; } \
		&& { echo "$(BEAR_CMD)"; $(BEAR_CMD); }

clean:
	cargo clean
	rm -f compile_commands.json compile_commands.events.json


.PHONY: build test check docs publish compile_commands clean
