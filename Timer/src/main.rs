use clap::Parser;
use std::thread::sleep;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0)]
    seconds: u64,

    #[arg(long, default_value_t = 0)]
    minutes: u64,

    #[arg(long, default_value_t = 0)]
    hours: u64,

    #[arg(long, default_value_t = 1)]
    repeat_times: u8,
}

fn main() {
    let args = Args::parse();
    let sleep_time = args.seconds + args.minutes * 60 + args.hours * 3600;
    std::thread::sleep(Duration::from_secs(sleep_time));
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    for _ in 0..args.repeat_times {
        play_sound(&stream_handle);
    }
}

fn play_sound(stream_handle: &rodio::OutputStreamHandle) {
    let file = File::open("alarm_sound.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::try_new(stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}