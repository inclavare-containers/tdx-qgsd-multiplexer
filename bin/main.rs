use anyhow::*;
use clap::{App, Arg};
use log::info;
use qgsd_multiplexer::Multiplexer;

const QGS_PATH: &str = "/var/run/tdx-qgs/qgs.socket";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = App::new("guest inotifier")
        .author("xynnn007")
        .arg(
            Arg::with_name("qgs")
                .long("qgs")
                .value_name("qgs")
                .help("path to the qgs socket file.")
                .takes_value(true)
                .default_value(QGS_PATH)
                .required(false),
        )
        .get_matches();

    let qgs_socket = matches.value_of("qgs").expect("get qgs path failed");
    info!("start watch and use qgs path {qgs_socket}");

    let mut multiplexer = Multiplexer::new()?;
    multiplexer.start(qgs_socket).await
}
