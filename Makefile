all: target/release/flip-text
.PHONY: all

target/release/flip-text:
	cargo build --release
.PHONY: target/release/flip-text

install: target/release/flip-text
	./alfred-install-workflow/install-workflow.sh target/release/flip-text
.PHONY: install

update-plist:
	./alfred-install-workflow/install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
.PHONY: clean

