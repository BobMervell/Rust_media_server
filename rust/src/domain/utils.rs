pub fn is_video_file(file_name: &str) -> bool {
    let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"];

    if let Some(ext) = file_name.rsplit('.').next() {
        video_extensions.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

pub fn is_not_featurette(file_path: &str) -> bool {
    let featurette_names = ["featurettes", "featurette", "feat"];
    if let Some(ext) = file_path.rsplit('/').next() {
        !featurette_names.contains(&ext.to_lowercase().as_str())
    } else {
        true
    }
}
