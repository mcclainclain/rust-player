use clap::Parser;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use indicatif::ProgressBar;
use lofty::{read_from_path, AudioFile};
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{stdout, BufReader};
use std::time::{Duration, Instant};

mod args;
use args::PlayerArgs;

fn listen_for_events(duration: f32, stream: &mut Sink) -> crossterm::Result<()> {
    let mut stdout = stdout();

    crossterm::terminal::enable_raw_mode()?;
    loop {
        execute!(stdout, crossterm::cursor::MoveToColumn(0));

        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                ..
            }) => {
                if stream.is_paused() {
                    println!("Song resumed");
                    stream.play();
                } else {
                    println!("Song paused");
                    stream.pause();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                println!("Song stopped");
                stream.stop();
                break;
            }
            _ => (),
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn main() {
    let args = PlayerArgs::parse();

    match &args.entity_type {
        args::EntityType::Play(play) => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let mut sink = Sink::try_new(&stream_handle).unwrap();

            let file_result = File::open(&play.path);

            let file = match file_result {
                Ok(file) => file,
                Err(error) => {
                    println!("Error: {}", error);
                    return;
                }
            };

            let file_data = read_from_path(&play.path);
            let file_data = match file_data {
                Ok(file_data) => file_data,
                Err(error) => {
                    println!("Error: {}", error);
                    return;
                }
            };
            let duration = file_data.properties().duration();
            let duration = duration.as_secs_f32();

            let source = Decoder::new(BufReader::new(file)).unwrap();

            sink.append(source);
            println!("Playing audio at {}", play.path);
            println!("Press 'p' to pause/play, 'q' to quit");

            match listen_for_events(duration.round(), &mut sink) {
                Ok(_) => (),
                Err(error) => println!("Error: {}", error),
            }
            sink.sleep_until_end();
        }
        args::EntityType::Queue(queue) => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = File::open(&queue.path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();

            sink.append(source);
        }
    }
}
