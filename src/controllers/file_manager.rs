pub fn move_file (
    source_file: &std::path::PathBuf,
    destination_directory: &std::path::PathBuf,
) -> Result<(), std::io::Error> {
    // Create the destination directory if it doesn't exist
    std::fs::create_dir_all(destination_directory)?;

    // Construct the destination file path
    let destination_file = destination_directory.join(source_file.file_name().unwrap());

    let source_file_arw = source_file.clone().with_extension("ARW");
    let destination_file_arw = destination_directory.join(source_file.file_stem().unwrap()).with_extension("ARW");

    match std::fs::rename(&source_file_arw, &destination_file_arw) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to move ARW file (maybe it doesn't exist): {:?}", source_file_arw);
        }
    }

    std::fs::rename(source_file, &destination_file)
}