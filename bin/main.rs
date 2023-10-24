use anyhow::*;
use clap::{App, Arg};
use log::info;
use qgsd_multiplexer::Multiplexer;

const QGS_PATH: &str = "/var/run/tdx-qgs/qgs.socket";
const VSOCK_NAME: &str = "kata.hvsock_40";
const VSOCK_PATH: &str = "/var/lib/vc/dragonball";

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
        .arg(
            Arg::with_name("vsock_name")
                .long("vsock_name")
                .value_name("vsock_name")
                .help("vsock file name")
                .takes_value(true)
                .default_value(VSOCK_NAME)
                .required(false),
        )
        .arg(
            Arg::with_name("vsock_path")
                .long("vsock_path")
                .value_name("vsock_path")
                .help("vsock path name")
                .takes_value(true)
                .default_value(VSOCK_PATH)
                .required(false),
        )
        .get_matches();

    let qgs_socket = matches.value_of("qgs").expect("get qgs path failed");
    let vsock_name = matches
        .value_of("vsock_name")
        .expect("get vsock name failed");
    let vsock_path = matches
        .value_of("vsock_path")
        .expect("get vsock path failed");
    info!("start watch and use qgs path {qgs_socket}, vsock name {vsock_name}, vsock path {vsock_path}");

    let mut multiplexer = Multiplexer::new()?;
    multiplexer.start(qgs_socket, vsock_name, vsock_path).await
}
