use num_cpus;

pub const FILE_NAME_LEN: u32 = 20;
pub const CONTENT_LEN: u32 = 20000;

pub const TXT_EXTENSION: &str = ".txt";
// pub const DOCX_EXTENSION: &str = ".docx";
// pub const PNG_EXTENSION: &str = ".png";
// pub const PDF_EXTENSION: &str = ".pdf";
// pub const MP4_EXTENSION: &str = ".mp4";
pub const FOLDER: &str = "";

pub fn get_max_threads_limit() -> u32 {
    return (num_cpus::get() * 2) as u32;
}
