//! use to inotify the creation of new tdx guest's vsocks

use std::{collections::BTreeMap, ffi::OsString, sync::Arc};

use anyhow::*;
use futures::TryStreamExt;
use inotify::{Event, EventMask, Inotify, WatchMask};
use log::{error, info, warn};
use tokio::{
    io,
    net::{UnixListener, UnixStream},
    sync::{mpsc::Sender, Mutex},
};
use tokio_stream::wrappers::UnixListenerStream;

pub struct Multiplexer {
    inotify: Inotify,
    qgs_map: Arc<Mutex<BTreeMap<String, Sender<usize>>>>,
}

impl Multiplexer {
    pub fn new() -> Result<Self> {
        let inotify = Inotify::init().context("inotify init failed")?;
        Ok(Self {
            inotify,
            qgs_map: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    pub async fn start(
        &mut self,
        qgs_socket_path: &str,
        vsock_name: &str,
        vsock_path: &str,
    ) -> Result<()> {
        self.inotify
            .add_watch(vsock_path, WatchMask::CREATE | WatchMask::DELETE)
            .context("watch failed")?;

        let mut buffer = [0; 4096];
        let mut events = self
            .inotify
            .event_stream(&mut buffer)
            .context("Error while reading events")?;

        while let std::result::Result::Ok(event) = events.try_next().await {
            let event = match event {
                Some(e) => e,
                None => {
                    warn!("get an empty event.");
                    continue;
                }
            };

            if event.mask.contains(EventMask::CREATE) {
                let id = get_guest_id(event)?;
                info!("Create new guest id {id}");

                let guest_socket = format!("{vsock_path}/{id}/{vsock_name}");
                info!("");
                let guest_socket =
                    UnixListener::bind(guest_socket).context("connect guest unix socket")?;
                let listener = UnixListenerStream::new(guest_socket);

                let map = self.qgs_map.clone();
                let qgs_socket_path = qgs_socket_path.to_string();
                tokio::task::spawn(async move {
                    listen_guest(listener, &qgs_socket_path, &id, map).await
                });
            } else if event.mask.contains(EventMask::DELETE) {
                let id = get_guest_id(event)?;
                info!("Remove guest id {id}");

                match self.qgs_map.lock().await.get_mut(&id) {
                    Some(sender) => {
                        if sender.is_closed() {
                            continue;
                        }

                        if let Err(e) = sender.send(0).await {
                            error!("close connection via mpsc failed: {e}");
                        }
                    }
                    None => warn!("{id} exits but no qgs process found"),
                }

                let _ = self.qgs_map.lock().await.remove(&id);
            }
        }

        Ok(())
    }
}

fn get_guest_id(event: Event<OsString>) -> Result<String> {
    let res = event
        .name
        .ok_or_else(|| anyhow!("inotify catches empty filename"))?
        .to_string_lossy()
        .to_string();
    Ok(res)
}

async fn listen_guest(
    mut listener: UnixListenerStream,
    qgs_socket_path: &str,
    id: &str,
    tx_map: Arc<Mutex<BTreeMap<String, Sender<usize>>>>,
) -> Result<()> {
    while let std::result::Result::Ok(guest_socket) = listener.try_next().await {
        let guest_socket = match guest_socket {
            Some(inner) => inner,
            None => continue,
        };

        let (mut gr, mut gw) = guest_socket.into_split();
        let qsocket = UnixStream::connect(qgs_socket_path)
            .await
            .context("qgs unix socket bind")?;

        let (mut qr, mut qw) = qsocket.into_split();
        let id = id.to_string();

        let (tx, mut rx) = tokio::sync::mpsc::channel(10);

        tx_map.lock().await.insert(id.clone(), tx);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Err(e) = io::copy(&mut gr, &mut qw) => error!("{id} connection failed: {e}"),
                    Err(e) = io::copy(&mut qr, &mut gw) => error!("{id} connection failed: {e}"),
                    Some(_) = rx.recv() => {
                        info!("{id} exits.");
                        return
                    }
                    else => {},
                };
            }
        });
    }

    Ok(())
}
