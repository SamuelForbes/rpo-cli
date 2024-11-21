mod args;
mod file_move;

use clap::Parser;
use args::Args;
use std::collections::HashSet;
use std::{fs, io};
use std::fs::DirEntry;
use std::path::Path;
use file_move::*;
use rayon::prelude::*;

pub fn run() {
    if let Err(error) = move_photos(Args::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn move_photos(args: Args) -> Result<(), io::Error> {
    println!("{:?}", args);

    let file_moves = build_file_moves(&args)?;

    create_new_folders(&file_moves, &args.target_directory)?;
    process_moves(&file_moves, &*args.target_directory);

    Ok(())
}

fn build_file_moves(args: &Args) -> Result<Vec<FileMove>, io::Error> {
    let mut moves = vec!();

    for result in fs::read_dir(args.source_directory.clone())? {
        let entry = result?;
        if entry_is_valid_file_type(&args, &entry) {
            moves.push(FileMove::from(entry))
        }
    }

    Ok(moves)
}

fn entry_is_valid_file_type(args: &Args, entry: &DirEntry) -> bool {
    args.valid_file_formats.contains(
        &entry.file_name().into_string()
            .expect(format!("{:?} is a bad file name", entry.file_name()).as_str())
            .split('.')
            .last()
            .expect(format!("{:?} file does not have a suffix", entry.file_name()).as_str())
            .to_string()
    )
}

fn process_moves(file_moves: &Vec<FileMove>, target_path: &str) {
    file_moves
        .par_iter()
        .for_each(|file_move| {
            let file_name = file_move.source.file_name().into_string().unwrap().replace("\"", "");

            let target = if file_move.target_path.is_some() {
                format!("{}{}/{}", target_path, file_move.target_path.as_ref().unwrap(), file_name)
            } else {
                format!("{}/{}", target_path.to_string(), file_name)
            };
            match fs::rename(file_move.source.path(), target) {
                Err(e) => eprintln!("{e}"),
                _ => ()
            }
        });
}

fn create_new_folders(file_moves: &Vec<FileMove>, target_path: &str) -> Result<(), io::Error> {
    let mut required_folders = HashSet::new();

    create_path_if_not_exists(String::from(target_path))?;

    for file_move in file_moves {
        if file_move.target_path.as_ref().is_some() {
            required_folders.insert(file_move.target_path.as_ref().unwrap());
        }
    }

    for entry in required_folders {
        let path_pieces = entry[1..].split("/").collect::<Vec<&str>>();
        let year_folder = format!("{target_path}/{}", path_pieces[0]);
        let month_folder = format!("{target_path}{entry}");

        create_path_if_not_exists(year_folder)?;
        create_path_if_not_exists(month_folder)?;
    }

    Ok(())
}

fn create_path_if_not_exists(path_string: String) -> Result<(), io::Error> {
    if !Path::new(&path_string).exists() {
        fs::create_dir(path_string)?;
    }

    Ok(())
}

#[test]
fn valid_file_formats(){
    let args = Args {
        source_directory: "test".to_string(),
        target_directory: "test".to_string(),
        valid_file_formats: vec!("jpg".to_string())
    };
    
    assert!(entry_is_valid_file_type(&args, &fs::read_dir("./Test").unwrap().map(|entry| entry.unwrap()).collect::<Vec<DirEntry>>()[0]));
}

#[test]
fn invalid_file_formats(){
    let args = Args {
        source_directory: "test".to_string(),
        target_directory: "test".to_string(),
        valid_file_formats: vec!("png".to_string())
    };

    assert!(!entry_is_valid_file_type(&args, &fs::read_dir("./Test").unwrap().map(|entry| entry.unwrap()).collect::<Vec<DirEntry>>()[0]));
}