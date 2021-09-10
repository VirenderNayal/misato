use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::exit,
    vec,
};

pub enum SourceFileType {
    Html,
    _Md,
}

pub struct SourceFile {
    pub filetype: SourceFileType,
    pub path: String,
}

fn available_layouts(layout_folder: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut layouts = vec![];

    for path in fs::read_dir(layout_folder)? {
        let path = path?.path();
        if let Some("html") = path.extension().and_then(OsStr::to_str) {
            layouts.push(path.to_owned());
        }
    }
    Ok(layouts)
}

fn names_from_path(paths: Vec<PathBuf>) -> Vec<String> {
    let mut layout_list: Vec<String> = vec![];
    let mut file_name: String;

    for p in paths {
        file_name = p
            .file_name()
            .unwrap()
            .to_str()
            .to_owned()
            .ok_or("[ERR] Could not get filename of layout")
            .unwrap()
            .to_string();

        layout_list.push(file_name.split('.').collect::<Vec<&str>>()[0].to_string());
    }

    layout_list
}

pub fn detect_layout(source_file: SourceFile, layout_folder: &Path) -> Option<String> {
    let mut layout: String = "".to_string();
    let contents =
        fs::read_to_string(source_file.path).expect("Something went wrong reading the file");

    for line in contents.lines() {
        match source_file.filetype {
            SourceFileType::Html => {
                if line.contains("<!-- layout:") {
                    layout = String::from(&line[13..(line.len() - 4)]);
                    break;
                }
            }
            SourceFileType::_Md => {}
        }
    }

    let available_layouts =
        names_from_path(available_layouts(layout_folder.to_path_buf()).unwrap());
    if available_layouts.is_empty() {
        eprintln!("[ERR] No layouts defined in _layout.");
        exit(1);
    }
    println!("{}", layout);

    for l in available_layouts {
        println!("{}", l);
        if l == layout {
            return Some(layout);
        }
    }

    None
}
