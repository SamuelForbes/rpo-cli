use chrono::{Datelike, Month, NaiveDateTime};
use exif::{Exif, In, Tag};
use num_traits::FromPrimitive;
use std::fs;
use std::fs::DirEntry;

#[derive(Debug)]
pub struct FileMove {
    pub source: DirEntry,
    pub target_path: Option<String>,
}

impl FileMove {
    pub fn from(source: DirEntry) -> FileMove {
        let exif = FileMove::get_exif_from_source(&source);
        println!("move from {:?}, {}", source.path(), exif.is_some());

        let mut move_base = FileMove {
            source,
            target_path: None,
        };

        if exif.is_some() {
            move_base.target_path = Self::get_target_path(exif.unwrap())
        }

        move_base
    }

    fn get_exif_from_source(source: &DirEntry) -> Option<Exif> {
        let file = fs::File::open(source.path())
            .expect(format!("Couldn't open file {:?}", source.path()).as_str());

        let mut reader = std::io::BufReader::new(&file);
        match exif::Reader::new().read_from_container(&mut reader) {
            Ok(exif) => Some(exif),
            _ => None,
        }
    }

    fn get_target_path(exif: Exif) -> Option<String> {
        println!("{:?}", exif.get_field(Tag::DateTime, In::PRIMARY));
        if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
            let created_date =
                NaiveDateTime::parse_from_str(&field.display_value().to_string(), "%F %T")
                    .expect("could not parse bad time value");

            let month_string = Month::from_u32(created_date.month()).unwrap().name();

            Some(format!("/{}/{}", created_date.year(), month_string))
        } else {
            None
        }
    }
}

#[test]
fn creates_move() {
    let moves = fs::read_dir("../")
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_name().eq("IMG_20150411_125741.jpg"))
        .map(FileMove::from)
        .collect::<Vec<FileMove>>();

    assert_eq!(moves.len(), 1);
    assert_eq!(Some(String::from("/2015/April")), moves[0].target_path);
}

#[test]
fn no_metadata() {
    let moves = fs::read_dir("../")
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_name().eq("0001.png"))
        .map(FileMove::from)
        .collect::<Vec<FileMove>>();

    assert_eq!(moves.len(), 1);
    assert_eq!(None, moves[0].target_path);
}