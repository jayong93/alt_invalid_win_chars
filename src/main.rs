use std::{
    env::args,
    fs::{metadata, read_dir, rename},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for ref path in args().skip(1) {
        let meta = metadata(path)?;
        if meta.is_dir() {
            for entry in read_dir(path)? {
                let entry = entry?;
                let old_path = entry.path();

                if let Some(new_name) = old_path.file_name().and_then(|old_name| {
                    invalid_win_char::alter_invalid_win_chars(old_name.to_string_lossy())
                }) {
                    let mut new_path = old_path.clone();
                    new_path.set_file_name(new_name);
                    eprintln!(
                        "From {} To {}",
                        old_path.to_string_lossy(),
                        new_path.to_string_lossy()
                    );
                    rename(old_path, new_path)?;
                }
            }
        } else {
            if let Some(new_name) = invalid_win_char::alter_invalid_win_chars(path.into()) {
                let mut new_path: PathBuf = path.into();
                new_path.set_file_name(new_name);

                eprintln!("From {} To {}", path, new_path.to_string_lossy());

                rename(path, new_path)?;
            }
        }
    }
    Ok(())
}
