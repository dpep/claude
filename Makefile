.PHONY: build test fmt fmt-check lint check install uninstall clean

# Skills are markdown and need no build. Some of them drive a small CLI;
# those live in rust/ and land on PATH via `make install`.
#
# Binaries are invoked by name, never by plugin-relative path: a plugin's
# install directory is stamped with its version, so a path that works today
# breaks on the next update. A symlink on PATH survives.
CLAUDE_BIN_DIR ?= $(HOME)/.claude/bin

build:
	cd rust && cargo build --release --workspace

test:
	cd rust && cargo test --workspace
	bash script/find-gem-test.sh

fmt:
	cd rust && cargo fmt --all

fmt-check:
	cd rust && cargo fmt --all --check

lint:
	cd rust && cargo clippy --all-targets --workspace -- -D warnings

# The gate. Run before pushing.
check: fmt-check lint test

install: build
	@mkdir -p $(CLAUDE_BIN_DIR)
	@ln -sf "$(CURDIR)/rust/target/release/find-skill" $(CLAUDE_BIN_DIR)/find-skill
	@echo "linked $(CLAUDE_BIN_DIR)/find-skill -> rust/target/release/find-skill"
	@ln -sf "$(CURDIR)/plugins/code/bin/find-gem"      $(CLAUDE_BIN_DIR)/find-gem
	@echo "linked $(CLAUDE_BIN_DIR)/find-gem   -> plugins/code/bin/find-gem"
	@command -v find-skill >/dev/null 2>&1 || { \
	    echo ""; \
	    echo "  ⚠️  $(CLAUDE_BIN_DIR) is not on PATH — add to your shell rc:"; \
	    echo "      export PATH=\"$(CLAUDE_BIN_DIR):\$$PATH\""; \
	}

uninstall:
	@rm -f $(CLAUDE_BIN_DIR)/find-skill $(CLAUDE_BIN_DIR)/find-gem
	@echo "unlinked find-skill / find-gem"

clean: uninstall
	cd rust && cargo clean
