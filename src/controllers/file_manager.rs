pub fn move_file (
    source_file: &std::path::PathBuf,
    destination_directory: &std::path::PathBuf,
) -> Result<(), std::io::Error> {
    // Create the destination directory if it doesn't exist
    std::fs::create_dir_all(destination_directory)?;

    // Construct the destination file path
    let destination_file = destination_directory.join(source_file.file_name().unwrap());

    // Move the file
    std::fs::rename(source_file, &destination_file)
}