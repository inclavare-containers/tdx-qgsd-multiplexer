# QGSD-Multiplexer

This tool is used to multiplex QGS to serve many TD-Guests created by the light-weight
Hypervisor such as Dragonball and Cloud Hypervisor.

The tool will watch `/var/lib/vc/dragonball`. Whenever a new guest is created, a related
vsock file `/var/lib/vc/dragonball/<guest-id>/root/kata.hvsock_40` will be created. The
vsock file will be connect to qgs socket on the other side.

If a new one is created, there will be a new connection between the socket file and the
QGS' socketfile `/var/run/tdx-qgs/qgs.socket` by default.

Run the following command to build the binary:

```bash
cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release
```

And you can find in `target/release/qgsd-multiplexer`.

By default, it will use `"/var/run/tdx-qgs/qgs.socket"` as the qgs socket.

## Usage

### Build the binary

1. Build and launch the modified version of qgs

```bash
cd tool
docker built .
```

launch a container using newly built image, copy the built binary `qgs`
```bash
docker cp <container-id>:/usr/local/bin/qgs/qgs ./qgs
```

create a directory to put qgs socket and launch the qgs
```bash
mkdir -p /var/run/tdx-qgs
./qgs -uds_path=/var/run/tdx-qgs/qgs.socket
```

2. build and launch the qgsd-multiplexer

```bash
git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git
cd tdx-qgsd-multiplexer
cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release
./target/release/qgsd-multiplexer
```

Now it will automatically multiplex the qgsd to different tdx guests.

### Build image containing tdx stack of anolis8.6 and the binary

```bash
git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git
cd tdx-qgsd-multiplexer
sudo bash tool/build.sh
```
