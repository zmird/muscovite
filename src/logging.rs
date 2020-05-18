use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log4rs::filter::threshold::ThresholdFilter;


pub fn config_logs(filename: String) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .build(format!("logs/{}", filename))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder()
            .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
            .build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("stdout")
            .appender("logfile")
            .build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
