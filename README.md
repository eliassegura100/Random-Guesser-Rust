## What This Is

`rustguess` is an out-of-tree Linux kernel module that exposes a character device at `/dev/rustguess`. Users write guesses to the device and read hints back. The game state is mutex-protected across all opens of the device. Malformed input is handled with a friendly error message rather than a kernel panic.

## Build & Run

### Requirements
- Ubuntu 26.04 LTS (kernel 7.0.0 with `CONFIG_RUST=y`)
- `rustc 1.93` (kernel-blessed version)
- `linux-headers` and `linux-source` for your kernel version

### Setup
```bash
sudo apt install -y build-essential linux-headers-$(uname -r) kmod \
    rustc-1.93 rust-1.93-src bindgen clang llvm lld \
    flex bison libssl-dev libelf-dev libdw-dev linux-source-7.0.0
sudo update-alternatives --install /usr/bin/rustc rustc /usr/bin/rustc-1.93 100
```

Build the kernel Rust support files:
```bash
cd /usr/src
sudo tar -xjf linux-source-7.0.0.tar.bz2
sudo cp /boot/config-$(uname -r) /usr/src/linux-source-7.0.0/.config
sudo make -C /usr/src/linux-source-7.0.0 LLVM=1 olddefconfig
sudo make -C /usr/src/linux-source-7.0.0 LLVM=1 rust/core.o rust/compiler_builtins.o
sudo ln -s /usr/src/linux-source-7.0.0/rust /usr/src/linux-headers-7.0.0-14-generic/rust
```

### Build and Load
```bash
git clone https://github.com/eliassegura100/Random-Guesser-Rust.git
cd Random-Guesser-Rust
make
sudo insmod rustguess.ko
```

### Play
```bash
sudo cat /dev/rustguess          # read welcome message
echo 50 | sudo tee /dev/rustguess > /dev/null
sudo cat /dev/rustguess          # get a hint
```

### Unload
```bash
sudo rmmod rustguess
```

## Code Tour

Start at `impl kernel::InPlaceModule for RustGuess` — that is the module entry point where the device is registered and the welcome message is seeded into the global game state.

Then look at `write_iter()` — this is where user input is parsed, the guess is compared to the secret, and the response message is stored in the mutex-protected `GameState`.

Finally look at `read_iter()` — this is where the last response message is copied back to user space. The `served` flag on the per-open `RustGuessDevice` prevents `cat` from looping forever on the same message.

## Design Notes

**Why a global `Mutex<GameState>` instead of per-open state?** All opens of `/dev/rustguess` share the same in-progress game. This means two terminals playing simultaneously see the same secret and the same guess history — which makes the game feel like a shared experience rather than isolated sessions.

**Why a hardcoded secret?** Simplicity. Every load of the module picks the same secret (42). The first future-work item below addresses this.

**Why `build_message()` as a helper?** Every response the game produces goes through the same fallible `KVec<u8>` allocation. Centralizing it means allocation failures propagate via `?` consistently rather than being handled ad-hoc in each match arm.

## Future Work

- **Random secret** — use the kernel RNG (`kernel::random::getrandom`) to pick a fresh secret at module load, so each `insmod` starts a new game with a different answer
- **Per-open game state** — move `GameState` into the per-open `RustGuessDevice` so multiple users can play simultaneously without sharing a secret
- **Difficulty levels** — write `RANGE:1000\n` to expand the search space before guessing
- **Guess history** — track all guesses and return them on a `HISTORY\n` command
- **A `/proc/rustguess` view** — expose the secret to root for testing without affecting the game state

## License

Licensed GPL-2.0 to match the Linux kernel.
