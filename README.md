# mechsound

A Rust CLI tool that plays **mechanical keyboard sounds** from the terminal. It listens for global key presses and plays corresponding sound effects to simulate the tactile feedback of mechanical keyboards.

## Features

- **Global Key Listening**: Captures key presses system-wide using the `rdev` library.
- **Concurrent Sound Playback**: Supports up to 10 concurrent sounds to handle rapid typing.
- **OGG Audio Support**: Plays high-quality OGG sound files for each key.
- **Cross-Platform**: Works on macOS, Windows, and Linux (with appropriate permissions).
- **Lightweight**: Minimal dependencies and efficient audio handling with `rodio`.

## How It Works

The application consists of three main modules:

### 1. Main Module (`src/main.rs`)
- Initializes the `AudioEngine` with a maximum of 10 concurrent sounds.
- Sets up a global key listener using `rdev`.
- On each key press, maps the key to a numeric code and triggers the corresponding sound.

### 2. Audio Engine (`src/audio.rs`)
- Manages the audio output stream using `rodio_wav_fix`.
- Loads and decodes OGG files from the `sounds/` directory.
- Handles concurrent playback by maintaining a queue of audio sinks.
- Automatically removes old sinks when the limit is exceeded to prevent resource exhaustion.

### 3. Key Code Mapping (`src/key_code.rs`)
- Maps `rdev::Key` enums to integer codes (e.g., `Key::KeyA` → 30).
- Supports a wide range of keys including letters, numbers, modifiers, function keys, and special keys.
- Returns `None` for unmapped keys, which are ignored.

### Sound Files
- Sounds are stored in the `sounds/` directory as OGG files.
- Each file is named with the key code (e.g., `30.ogg` for the 'A' key).
- The `sounds/config.json` contains metadata about the sound pack, including key mappings, but is not currently used by the application.
- The root `config.json` defines settings like volume, mode, and max concurrent sounds, but these are not yet implemented in the code.

## Installation

1. Ensure you have Rust installed: [rustup.rs](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone <repository-url>
   cd mechsound
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Dependencies

- `rdev`: For global key event listening.
- `rodio_wav_fix`: For audio playback and OGG decoding.
- `serde` & `serde_json`: For potential configuration handling (not currently used).
- `clap`: For CLI argument parsing (not currently used).
- `anyhow`: For error handling.
- `flume`: For message passing (not currently used).
- `rand`: For randomization (not currently used).
- `once_cell`: For lazy initialization (not currently used).
- `libc`: For low-level system interactions.

## Usage

Currently, the application runs in a single mode: listening for key presses and playing sounds.

```bash
cargo run
```

- The program will start listening for key presses.
- Press any mapped key to play its corresponding sound.
- Use Ctrl+C to stop the program.

Note: The README mentions additional commands like `mechsound listen`, `mechsound keyword`, and `mechsound test`, but these are not implemented in the current code. The application directly enters listening mode.

## Configuration

The `config.json` file contains settings that are not yet loaded by the application:

- `volume`: Sound volume level (0.0 to 1.0).
- `mode`: Playback mode (e.g., "round_robin").
- `max_concurrent`: Maximum number of concurrent sounds (currently hardcoded to 10).

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/mechsound`.

## Permissions

On macOS, you may need to grant accessibility permissions for global key listening. Go to System Preferences > Security & Privacy > Privacy > Accessibility and add the terminal or application.

## Limitations

- Configuration files are not loaded; settings are hardcoded.
- No CLI arguments are processed; the application always runs in listening mode.
- Error handling is basic; missing sound files are logged but ignored.
- Volume control is not implemented.

## Contributing

Feel free to contribute by implementing the planned features, such as configuration loading, CLI modes, or additional sound packs.

## License

[Add license information here]
