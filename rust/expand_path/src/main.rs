use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please provide two arguments");
        return;
    }
    let buffer: &str = &args[1];
    let pos: usize = args[2].parse::<u32>().unwrap() as usize;

    let (start, end) = find_word_boundaries(buffer, pos);
    let (new_buffer, new_cursor) = transform_word_in_buffer(buffer, start, end, transform);
    println!("{} {}", new_cursor, new_buffer);
}

pub fn transform(path: &str) -> String {
    // ファイルがシンボリックリンクか確認
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            // シンボリックリンクだったらリンク先を返す
            if metadata.file_type().is_symlink() {
                match fs::read_link(path) {
                    Ok(link_path) => link_path.to_str().unwrap().to_string(),
                    Err(_) => path.to_string(),
                }
            }
            // シンボリックリンクでなければ絶対パスを返す
            else {
                match Path::new(path).canonicalize() {
                    Ok(abs_path) => abs_path.to_str().unwrap().to_string(),
                    Err(_) => path.to_string(),
                }
            }
        }
        // 何らかの理由でメタデータ取得に失敗したら元のpathを返す
        Err(_) => path.to_string(),
    }
}

pub fn find_word_boundaries(buffer: &str, cursor_pos: usize) -> (usize, usize) {
    let mut start = cursor_pos;
    let mut end = cursor_pos;

    while start > 0 && buffer.chars().nth(start - 1).unwrap() != ' ' {
        start -= 1;
    }

    while end < buffer.chars().count() && buffer.chars().nth(end).unwrap() != ' ' {
        end += 1;
    }

    (start, end)
}

pub fn transform_word_in_buffer<F: FnOnce(&str) -> String>(
    buffer: &str,
    start: usize,
    end: usize,
    transform: F,
) -> (String, usize) {
    let before = &buffer[..start];
    let after = &buffer[end..];
    let word = &buffer[start..end];
    let transformed_word = transform(word);
    let new_buffer = format!("{}{}{}", before, transformed_word, after);
    let new_cursor_pos = start + transformed_word.chars().count();

    (new_buffer, new_cursor_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let path = "/tmp/powerlog/";
        let abspath = transform(path);
        assert_eq!(abspath, "/private/tmp/powerlog");
        let path = "/NotExist/";
        let abspath = transform(path);
        assert_eq!(abspath, "/NotExist/");
        let curdir = std::env::current_dir().unwrap();
        let path = "src";
        let abspath = transform(path);
        assert_eq!(abspath, curdir.join("src").to_string_lossy().to_string());
    }
    #[test]
    fn test_find_word_boundaries() {
        let buffer = "Hello, world!";
        let cursor_pos = 7;
        let (start, end) = find_word_boundaries(buffer, cursor_pos);
        assert_eq!(start, 7);
        assert_eq!(end, 13);
    }
    #[test]
    fn test_transform_word_in_buffer() {
        let buffer = "Hello, world";
        let start = 7;
        let end = 12;
        let transform = |word: &str| word.to_uppercase();
        let (new_buffer, new_cursor_pos) = transform_word_in_buffer(buffer, start, end, transform);
        assert_eq!(new_buffer, "Hello, WORLD");
        assert_eq!(new_cursor_pos, 12);
    }
}
