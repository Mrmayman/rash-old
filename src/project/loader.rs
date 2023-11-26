use std::io::Read;

use super::base::Project;

impl<'a> Project<'a> {
    pub fn parse(path: &std::path::PathBuf) -> serde_json::Value {
        let mut file = std::fs::File::open(&path.join("project.json"))
            .expect("Could not open project.json file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Could not read project.json file");

        serde_json::from_str(&content).expect("Could not parse project.json")
    }

    pub fn read_file_to_bytes(path: &str) -> std::io::Result<Vec<u8>> {
        // Open the file
        let mut file = std::fs::File::open(path)?;

        // Read the contents into a Vec<u8>
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    pub fn load_project_file(file_path: String) -> (tempfile::TempDir, std::path::PathBuf) {
        // Create a temporary directory
        let temp_dir = tempfile::TempDir::new().expect("Failed to create temporary directory");

        // Get the path of the temporary directory
        let temp_dir_path: std::path::PathBuf = temp_dir.path().to_path_buf();

        let archive: Vec<u8> =
            Project::read_file_to_bytes(&file_path.as_str()).expect("Could not read .sb3 file");
        let target_dir = std::path::PathBuf::from(&temp_dir_path);
        // Doesn't need to exist

        // The third parameter allows you to strip away toplevel directories.
        // If `archive` contained a single folder, that folder's contents would be extracted instead.
        zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true)
            .expect("Could not extract .sb3 zip");
        (temp_dir, temp_dir_path)
    }
}
