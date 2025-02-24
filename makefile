deploy:
	git add .
	git commit -m "$(filter-out $@,$(MAKECMDGOALS))"
	cargo check
	cargo fix --lib
	cargo fmt
	cargo build-sbf
	cargo test
	solana program deploy ./target/deploy/pinocchio_study.so
	solana program show --programs

down:
	solana program close DKwwCKYxHE27QnJ7LLWSFdYXE6ZqGVV6hKrRZZtYhexm --bypass-warning