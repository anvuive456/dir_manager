use super::parser::Command;

pub fn execute(command: Command) -> Result<(), String> {
    match command {
        Command::Search { name, in_path } => {
            println!("Searching for {:?} in {}", name, in_path);
            // Gọi hàm tìm kiếm cũ của bạn
            // search_pattern_recursive(name.unwrap_or(""), &in_path, None, 2)?;
            Ok(())
        }
        Command::Delete { name, in_path } => {
            println!("Deleting {:?} in {}", name, in_path);
            // Xử lý xóa file
            Ok(())
        }
        Command::Add { name, content } => {
            println!("Adding file '{}' with content {:?}", name, content);
            // Tạo file mới
            Ok(())
        }
    }
}
