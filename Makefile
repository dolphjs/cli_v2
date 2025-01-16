linux_build:
	@echo "Building cli_v2 for linux distributions..."
	cargo build --release --target x86_64-unknown-linux-gnu

windows_build:
	@echo "Building cli_v2 for windows distributions..."
	cargo build --release --target x86_64-pc-windows-gnu 

mac_build:
	@echo "Building cli_v2 for mac distributions..."
	cargo build --release --target x86_64-apple-darwin