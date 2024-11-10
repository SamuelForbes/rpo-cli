mod args;
mod file_move;

use clap::Parser;
use args::Args;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use file_move::*;
use rayon::prelude::*;

pub fn run(){
    if let Err(error) = move_photos(Args::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn move_photos(args: Args) -> Result<(), String>{
    println!("{:?}", args);

    let file_moves = build_file_moves(&*args.source_directory);

    create_new_folders(&file_moves, &args.target_directory)?;
    process_moves(&file_moves, &*args.target_directory);

    Ok(())
}

fn build_file_moves(source_path: &str) -> Vec<FileMove> {
    let result = fs::read_dir(source_path);

    match result {
        Ok(read_dir) => {
            read_dir.filter(|entry| entry.path().is_file())
                .map(FileMove::from)
                .collect::<Vec<FileMove>>()
        }
        Err(error) => {
            eprintln!("{error}");
        }
    }
}

fn process_moves(file_moves: &Vec<FileMove>, target_path: &str) {
    file_moves
        .par_iter()
        .for_each(|file_move| {
            let file_name = file_move.source.file_name().into_string().unwrap().replace("\"","");

            let target =  if file_move.target_path.is_some() {
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

fn create_new_folders(file_moves: &Vec<FileMove>, target_path: &str) -> Result<(), Err> {
    let mut required_folders = HashSet::new();

    for file_move in file_moves {
        if file_move.target_path.as_ref().is_some(){
            required_folders.insert(file_move.target_path.as_ref().unwrap());
        }
    }

    for entry in required_folders {
        let path_pieces = entry[1..].split("/").collect::<Vec<&str>>();
        let year_folder = format!("{target_path}/{}", path_pieces[0]);
        let month_folder = format!("{target_path}{entry}");

        if !Path::new(&year_folder).exists(){
            fs::create_dir(year_folder)?;
        }

        if !Path::new(&month_folder).exists(){
            fs::create_dir(month_folder)?;
        }
    }

    Ok(())
}