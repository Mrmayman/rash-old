use std::io::Read;

use super::project_main::Project;

impl<'a> Project<'a> {
    pub fn load_json(path: &std::path::Path) -> serde_json::Value {
        let mut file = std::fs::File::open(path.join("project.json"))
            .expect("Could not open project.json file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Could not read project.json file");

        serde_json::from_str(&content).expect("Could not parse project.json")
    }

    pub fn extract_zip_file(
        file_path: String,
    ) -> Result<(tempfile::TempDir, std::path::PathBuf), String> {
        // Create a temporary directory
        let temp_dir = match tempfile::TempDir::new() {
            Ok(temp_dir) => temp_dir,
            Err(err) => {
                return Err(format!(
                    "[error] Zipfile: Failed to create temporary directory: {err:?}"
                ))
            }
        };

        // Get the path of the temporary directory
        let temp_dir_path: std::path::PathBuf = temp_dir.path().to_path_buf();

        let archive: Vec<u8> =
            read_file_to_bytes(file_path.as_str()).expect("Could not read .sb3 file");

        let target_dir = std::path::PathBuf::from(&temp_dir_path);

        // The third parameter allows you to strip away toplevel directories.
        // If `archive` contained a single folder, that folder's contents would be extracted instead.
        zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true)
            .expect("Could not extract .sb3 zip");

        Ok((temp_dir, temp_dir_path))
    }
}

fn read_file_to_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    // Open the file
    let mut file = std::fs::File::open(path)?;

    // Read the contents into a Vec<u8>
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
