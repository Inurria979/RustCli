pub mod find_match{
    pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write){
        for line in content.lines(){
            if line.contains(pattern){
                write!(writer, "{}", line);
            }
        }
    }
}


