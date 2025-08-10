use std::{fs::{self, DirEntry, File}};

use rand::{rng, Rng};

pub fn random_verse() -> String {
    let paths = fs::read_dir("./bible");

    match paths {
        Ok(paths) => {
            let mut paths_vec: Vec<DirEntry> = Vec::new();

            for path in paths {
                match path {
                    Ok(dir) => {
                        paths_vec.push(dir);
                    },
                    Err(e) => {
                        println!("{}", e);

                        return "Failed to get a random verse".to_string()
                    }
                }
            }

            let rand_dir = &paths_vec[rng().random_range(0..=paths_vec.len() - 1)];

            let file_name = rand_dir.file_name();

            let s_file_name = file_name.into_string().unwrap();

            let f_file_name = format!("./bible/{}", s_file_name);

            let path =  String::from(f_file_name);

            let file = File::open(path);

            match file {
                Ok(v) => {
                    let content: Result<serde_json::Value, serde_json::Error> = serde_json::from_reader(v);

                    match content {
                        Ok(val) => {
                            let book = val.get("book").unwrap();

                            let chapters = val.get("chapters").unwrap();

                            let items = chapters.as_object().unwrap();

                            let mut chapters_vec: Vec<String> = Vec::new();

                            for item in items {
                                chapters_vec.push(item.0.to_string());
                            }

                            let random_chapter = &chapters_vec[rng().random_range(0..=chapters_vec.len() - 1)];

                            let c_verses = chapters.get(random_chapter).unwrap();

                            let verses = c_verses.get("verses").unwrap();

                            let verse_items = verses.as_object().unwrap();

                            let mut verses_vec: Vec<String> = Vec::new();

                            for item in verse_items {
                                verses_vec.push(item.0.to_string());
                            }

                            let random_verse = &verses_vec[rng().random_range(0..=verses_vec.len() - 1)];

                            let verse = verses.get(random_verse).unwrap();

                            let msg = format!("{} {}:{} {}", book.to_string().replace("\"", ""), random_chapter, random_verse, verse.to_string().replace("\"", ""));

                            return msg
                        },
                        Err(e) => {
                            println!("{}", e);

                            return "Failed to get a random verse".to_string()
                        }
                    }
                },
                Err(e) => {
                    println!("{}", e);

                    return "Failed to get a random verse".to_string()
                }
            }
        },
        Err(e) => {
            println!("{}", e);

            return "Failed to get a random verse".to_string()
        }
    }
}