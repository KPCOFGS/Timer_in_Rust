# Timer_in_Rust

A simple Rust program that waits for a specified amount of time and then plays an alarm sound. This can be used as a reminder or notification tool.

## Features
* Waits for a specified duration (in seconds, minutes, or hours).
* Repeats the alarm sound a specified number of times.
* Plays an MP3 file as the alarm sound.
## Requirements
* Rust (Install from [rust-lang.org](https://www.rust-lang.org/))
* MP3 file named `alarm_sound.mp3` in the root directory
* `libasound2-dev` installed on Linux devices
## Installation
1. Clone the repository:
```bash
git clone https://github.com/KPCOFGS/Timer_in_Rust.git
cd Timer_in_Rust/Timer
```
2. Build the project:
```bash
cargo build --release
```

## Usage
The program accepts the following command-line arguments:
`--seconds` (default: 0): Number of seconds to wait.
`--minutes` (default: 0): Number of minutes to wait.
`--hours` (default: 0): Number of hours to wait.
`--repeat_times` (default: 1): Number of times to repeat the alarm sound.
## Example
To wait for 1 minute and 30 seconds and repeat the alarm sound 3 times, use:
```bash
./target/release/new_project --seconds 30 --minutes 1 --repeat_times 3
```
## How It Works
1. The program parses the command-line arguments to determine the total sleep time.
2. It waits for the specified amount of time.
3. After the wait, it plays the `alarm_sound.mp3` file the specified number of times.
## License
This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.
