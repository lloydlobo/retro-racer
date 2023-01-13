# List all just  commands
default:
    just --list

# Build binary in debug mode
b:
    cargo b -F bevy/dynamic
# Build binary in release mode
br:
    cargo b -r -F bevy/dynamic
# Check binary in debug mode
c:
    cargo c
# Check binary in relase mode
cr:
    cargo c -r
# Fix clippy lint problems in debug mode
cl:
    cargo clippy
# Fix clippy lint problems in release mode
clf:
    cargo clippy --fix
# Fix clippy lint problems --allow-dirty --allow-staged
clf-all:
    cargo clippy --fix --allow-dirty --allow-staged
# Build and serve rust docs in debug mode
d:
    cargo d --no-deps --open
# Build and serve rust docs in release mode
dr:
    cargo d -r --no-deps --open
# Format with rustfmt in debug mode
f:
    cargo fmt
# Run binary in debug mode
r:
    cargo r -F bevy/dynamic
# Run binary in release mode
rr:
    cargo r -r -F bevy/dynamic
# Update cargo dependencies
u:
    cargo update
