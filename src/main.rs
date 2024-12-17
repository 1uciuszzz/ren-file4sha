use sha2::{Digest, Sha256};
use std::env::args;
use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    args.iter().for_each(|file_path| {
        let meta = fs::metadata(file_path).unwrap();
        match meta.is_file() {
            true => {
                let sha = calc_sha(file_path).unwrap();
                let abs_path = fs::canonicalize(file_path).unwrap();
                let mut slugs: Vec<&str> = abs_path.to_str().unwrap().split("/").collect();
                let file_name = slugs.pop().unwrap();
                let suffix = file_name.split(".").last().unwrap();
                let target_path = format!("{}/{}.{}", slugs.join("/"), sha, suffix);
                println!("{} : {}", file_path, &target_path);
                match fs::rename(file_name, &target_path) {
                    Ok(()) => {
                        println!("{} -> {}", file_path, &target_path);
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
            false => (),
        }
    })
}

fn calc_sha(file_path: &String) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                hasher.update(&buffer[..n]);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
}
