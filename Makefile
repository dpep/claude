.PHONY: build test fmt fmt-check lint check install uninstall clean

# Skills are markdown and need no build. Some of them drive a small CLI;
# those live in rust/ and land on PATH via `make install`.
#
# Binaries are invoked by name, never by plugin-relative path: a plugin's
# install directory is stamped with its version, so a path that works today
# breaks on the next update. A symlink on PATH survives.
CLAUDE_BIN_DIR ?= $(HOME)/.claude/bin

# **Symlink from a checkout we own; copy out of one Claude Code owns.**
# /statusbar-install runs this from the marketplace clone, and
# `claude plugin marketplace update` REPLACES that clone wholesale — taking
# rust/target and plugins/code/bin with it and leaving every symlink pointing
# at nothing, which is how an installed binary silently disappears after an
# upgrade. A copy survives the wipe; a symlink into a dev checkout keeps the
# property worth having, which is that `make build` alone puts a new binary
# live.
VOLATILE := $(findstring $(HOME)/.claude/plugins/marketplaces/,$(CURDIR))

# $(call install_bin,<path under CURDIR>,<installed name>)
define install_bin
@if [ -n "$(VOLATILE)" ]; then \
    cp -f "$(CURDIR)/$(1)" "$(CLAUDE_BIN_DIR)/$(2)"; \
    printf 'copied %-11s (from a clone Claude Code replaces)\n' "$(2)"; \
else \
    ln -sf "$(CURDIR)/$(1)" "$(CLAUDE_BIN_DIR)/$(2)"; \
    printf 'linked %-11s -> %s\n' "$(2)" "$(1)"; \
fi
endef

build:
	cd rust && cargo build --release --workspace

test:
	cd rust && cargo test --workspace
	bash script/find-gem-test.sh
	bash script/code-gc-test.sh

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
	$(call install_bin,rust/target/release/find-skill,find-skill)
	$(call install_bin,rust/target/release/statusbar,statusbar)
	$(call install_bin,plugins/code/bin/find-gem,find-gem)
	$(call install_bin,plugins/code/bin/code-gc,code-gc)
	@command -v find-skill >/dev/null 2>&1 || { \
	    echo ""; \
	    echo "  ⚠️  $(CLAUDE_BIN_DIR) is not on PATH — add to your shell rc:"; \
	    echo "      export PATH=\"$(CLAUDE_BIN_DIR):\$$PATH\""; \
	}

uninstall:
	@rm -f $(CLAUDE_BIN_DIR)/find-skill $(CLAUDE_BIN_DIR)/find-gem $(CLAUDE_BIN_DIR)/code-gc $(CLAUDE_BIN_DIR)/statusbar
	@echo "unlinked find-skill / find-gem / code-gc / statusbar"

clean: uninstall
	cd rust && cargo clean
