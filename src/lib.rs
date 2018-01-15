extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_mpd;
extern crate rustic_core;
#[macro_use]
extern crate slog;

mod commands;
mod error;
mod song;

use rustic_core::logger::logger;
use rustic_core::Rustic;
use rustic_core::bus;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::sync::{Mutex, Arc};

use commands::MpdCommand;

#[derive(Deserialize, Clone)]
pub struct MpdConfig {
    pub ip: String,
    pub port: i32
}

fn open(config: MpdConfig, app: Arc<Rustic>) {
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).unwrap();
    info!(logger, "[MPD] Listening on Port {}", config.port);

    for stream in listener.incoming() {
        debug!(logger, "[MPD] Connection opened");

        let app = app.clone();

        thread::spawn(move|| handle_client(stream.unwrap(), app));
    }
}

pub fn start(config: Option<MpdConfig>, app: Arc<Rustic>) -> thread::JoinHandle<()> {
    let config = config.unwrap_or(MpdConfig {
        ip: "0.0.0.0".to_owned(),
        port: 6600
    });
    thread::spawn(move|| {
        open(config, app);
    })
}

fn handle_client(stream: TcpStream, app: Arc<Rustic>) {
    let mut reader = BufReader::new(stream);
    let header = "OK MPD 0.16.0\n";
    let result = reader.get_ref().write(header.as_bytes());
    match result {
        Ok(_) => trace!(logger, "< {:?}", &header),
        Err(e) => error!(logger, "[MPD] {:?}", &e)
    }



    let events: Arc<Mutex<Vec<bus::Message>>> = Arc::new(Mutex::new(vec![]));

    let mut bus = app.bus.lock().unwrap();

    {
        let events = events.clone();
        bus.subscribe(Box::new(move|msg| {
            events.lock().unwrap().push(msg);
        }));
    }

    loop {
        let line = reader.by_ref().lines().next();
        match line {
            Some(line) => {
                match line {
                    Ok(line) => {
                        trace!(logger, "> {:?}", &line);
                        let cmd: Result<MpdCommands, serde_mpd::Error> = if line == "command_list_ok_begin" {
                            let mut current = reader.by_ref().lines().next().expect("line").expect("line");
                            trace!(logger, "> {:?}", &current);
                            let mut cmds: Vec<MpdCommands> = vec![];
                            while current.as_str() != "command_list_end" {
                                match parse_single(current) {
                                    Ok(cmd) => cmds.push(cmd),
                                    Err(_) => {}
                                }
                                current = reader.by_ref().lines().next().expect("line").expect("line");
                                trace!(logger, "> {:?}", &current);
                            }
                            Ok(MpdCommands::CommandList(cmds))
                        }else {
                            parse_single(line)
                        };
                        match cmd {
                            Ok(MpdCommands::Idle) => {},
                            Ok(MpdCommands::Close) => {
                                break;
                            },
                            Ok(cmd) => {
                                let mut result = handle_mpd_command(cmd, &app).unwrap();
                                result += "OK\n";
                                trace!(logger, "< {:?}", &result);
                                reader.get_ref().write(result.as_bytes());
                            },
                            Err(err) => {
                                error!(logger, "[MPD] {:?}", err);
                            }
                        }
                    },
                    Err(err) => {
                        error!(logger, "[MPD] {:?}", &err);
                        break;
                    }
                }
            },
            None => break
        }
    }

    debug!(logger, "[MPD] Connection closed");
}

#[derive(Debug, Deserialize)]
enum MpdCommands {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "currentsong")]
    CurrentSong,
    #[serde(rename = "commandlist")]
    CommandList(Vec<MpdCommands>),
    #[serde(rename = "plchanges")]
    PlaylistChanges(String),
    #[serde(rename = "outputs")]
    Outputs,
    #[serde(rename = "decoders")]
    Decoders,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "noidle")]
    NoIdle,
    #[serde(rename = "listplaylists")]
    ListPlaylists,
    #[serde(rename = "listplaylist")]
    ListPlaylist(String),
    #[serde(rename = "listplaylistinfo")]
    ListPlaylistInfo(String),
    #[serde(rename = "load")]
    LoadPlaylist(String),
    #[serde(rename = "lsinfo")]
    ListInfo(String),
    #[serde(rename = "next")]
    Next,
    #[serde(rename = "pause")]
    Pause,
    // Pause(bool), Spec says bool argument exists, ncmpcpp doesn't send it
    #[serde(rename = "play")]
    Play(u64),
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "list")]
    List(String),
    #[serde(rename = "add")]
    Add(String),
    #[serde(rename = "addid")]
    AddId(String),
    #[serde(rename = "volume")]
    ChangeVolumeBy(i32),
    #[serde(rename = "setvol")]
    ChangeVolume(u32),
    #[serde(rename = "commands")]
    Commands,
    #[serde(rename = "tagtypes")]
    TagTypes,
    #[serde(rename = "close")]
    Close
}

fn parse_single(line: String) -> Result<MpdCommands, serde_mpd::Error> {
    serde_mpd::from_str(line.as_str())
}

fn handle_mpd_command(cmd: MpdCommands, app: &Arc<Rustic>) -> Result<String, error::MpdError> {
    debug!(logger, "[MPD] Command: {:?}", &cmd);
    match cmd {
        MpdCommands::Status => commands::StatusCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::CurrentSong => commands::CurrentSongCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        // MpdCommands::Pause(true) => commands::PauseCommand::new().handle(app)
        MpdCommands::Pause => commands::PauseCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Play(_) => commands::PlayCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Stop => commands::StopCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListInfo(path) => commands::ListInfoCommand::new(path).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylists => commands::ListPlaylistsCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylist(name) => commands::ListPlaylistCommand::new(name).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ListPlaylistInfo(name) => commands::ListPlaylistInfoCommand::new(name).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::LoadPlaylist(name) => commands::LoadPlaylistCommand::new(name).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Previous => commands::PreviousCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Next => commands::NextCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Outputs => commands::OutputsCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::List(ref t) if t == "Artist" => commands::ListArtistCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ChangeVolumeBy(volume) => commands::ChangeVolumeCommand::new(volume).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::ChangeVolume(volume) => commands::SetVolumeCommand::new(volume).handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::Commands => commands::CommandsCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::TagTypes => commands::TagTypesCommand::new().handle(app)
            .map(|res| serde_mpd::to_string(&res).unwrap()),
        MpdCommands::CommandList(commands) => {
            let mut result = String::new();
            for command in commands {
                result += handle_mpd_command(command, app).unwrap().as_str();
                result += "list_OK\n";
            }
            Ok(result)
        }
        _ => Ok(String::new())
    }
}