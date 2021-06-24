use sass_rs::{Options, compile_file, OutputStyle};
use std::fs::{read_dir, DirEntry, File};
use std::io::Write;
use std::ffi::OsString;
use std::str::FromStr;

fn main() {
    println!("cargo:rerun-if-changed=www/style/scss");

    let dir = read_dir("www/style/scss").expect("Could not read path 'www/style/scss'");

    let mut options = Options::default();
    options.output_style = OutputStyle::Compressed;

    dir.filter_map(Result::ok)
        .filter(|f: &DirEntry| f.path().extension().unwrap_or(&*OsString::from_str("NONE").unwrap()) == &*OsString::from_str("scss").unwrap())
        .for_each(|f: DirEntry| {
            println!("cargo:rerun-if-changed=www/style/scss/{}.scss", f.file_name().to_str().unwrap());
            match compile_file(&f.path(), options.clone()) {
                Ok(result) => {
                    let name = f.file_name();
                    let v: Vec<&str> = name.to_str().unwrap().split('.').collect();
                    let mut nf = File::create(format!("www/style/css/{}.css", v.get(0).unwrap())).expect("Failed to create a file");
                    nf.write_all(result.as_bytes()).expect("Failed to write to file");
                }
                Err(error) => {
                    panic!("{}", error);
                }
            }
        });
}