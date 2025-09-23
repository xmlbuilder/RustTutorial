use std::env;
use std::fmt::Arguments;
use std::path::PathBuf;

/// 문자열 포맷: printf 스타일
pub fn format_string(format: &str, args: impl std::fmt::Display) -> String {
    format!("{}", args)
}

/// 문자열 치환: 모든 패턴을 교체
pub fn replace_all(message: &str, pattern: &str, replacement: &str) -> String {
    message.replace(pattern, replacement)
}

/// 임시 경로 반환: OS별 temp 디렉토리 + "nxdbms"
pub fn get_db_temp_path() -> PathBuf {
    env::temp_dir().join("nxdbms")
}

/// Rust의 format!을 감싸는 유틸리티
pub fn format_string_args(args: Arguments) -> String {
    format!("{}", args)
}
