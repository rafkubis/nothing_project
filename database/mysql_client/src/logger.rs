use env_logger::fmt::Formatter;
use env_logger::{Builder, Target};
use std::fs::File;
use std::io::Write;

pub fn init_logger(file_path: Option<String>) {
    let format_func = |buf: &mut Formatter, record: &log::Record| {
        writeln!(
            buf,
            "[{}  {} {:?} Task({:?}) {}.{}] {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            std::thread::current().id(),
            tokio::task::try_id(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    };
    if file_path.is_some() {
        let file: Box<File> = Box::new(std::fs::File::create(file_path.unwrap()).unwrap());
        Builder::from_default_env()
            .target(Target::Pipe(file))
            .format(format_func)
            .init();
    } else {
        Builder::from_default_env()
            .target(Target::Stdout)
            .format(format_func)
            .filter_level(log::LevelFilter::Info)
            .init();
    }
}
