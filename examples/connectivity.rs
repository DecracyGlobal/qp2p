use anyhow::anyhow;
use log::info;
use qp2p::{Config, QuicP2p, Result};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let mut config = Config::from_args();
    config.use_igd = true;
    if config.local_ip.is_none() && config.hard_coded_contacts.is_empty() {
        anyhow!("Both local IP and hard coded contacts are not provided. If this is the first node, provide the local IP. Else provide HCC.");
    }
    let qp2p = QuicP2p::with_config(Some(config), &vec![], false)?;
    let (ep, _, _, _) = qp2p.new_endpoint().await?;
    let addr = ep.socket_addr();
    info!("Socket address is: {}", addr);
    loop {}
    Ok(())
}

fn init_logging() {
    use flexi_logger::{DeferredNow, Logger};
    use log::Record;
    use std::io::Write;

    // Custom formatter for logs
    let do_format = move |writer: &mut dyn Write, clock: &mut DeferredNow, record: &Record| {
        let handle = std::thread::current();
        write!(
            writer,
            "[{}] {} {} [{}:{}] {}",
            handle
                .name()
                .unwrap_or(&format!("Thread-{:?}", handle.id())),
            record.level(),
            clock.now().to_rfc3339(),
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.args()
        )
    };

    Logger::with_env()
        .format(do_format)
        .suppress_timestamp()
        .start()
        .map(|_| ())
        .unwrap_or(());
}
