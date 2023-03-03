# QGSD-Multiplexer

This tool is used to multiplex QGS to serve many TD-Guests created by the light-weight
Hypervisor such as Dragonball and Cloud Hypervisor.

The tool will watch `/var/lib/vc/dragonball`. Whenever a new guest is created, a related
vsock file `/var/lib/vc/dragonball/<guest-id>/root/kata.hvsock_40` will be created. This
vsock file serves for the `vsock://40` in guest. Then QGSD-Multiplexer will connect the
vsock file to the QGS' socketfile `/var/run/tdx-qgs/qgs.socket` by default.

`/var/run/tdx-qgs/qgs.socket` is a socket file listened by qgsd to serve get quote requests.

Run the following command to build the binary:

```bash
cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release
```

And you can find in `target/release/qgsd-multiplexer`.

By default, it will use `"/var/run/tdx-qgs/qgs.socket"` as the qgs socket.

## Usage

### Build the binary (Not Recommended)

build and launch the qgsd-multiplexer

```bash
git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git
cd tdx-qgsd-multiplexer
cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release
./target/release/qgsd-multiplexer
```

Now it will automatically multiplex the qgsd to different tdx guests.

### Build image containing tdx QPL stack and launch

Build the docker image with tdx qpl and qgsd on Anolis8.6. (DCAP 1.15)
```bash
bash build.sh
```

Edit a correctly `sgx_default_qcnl.conf` for the container

```toml
PCCS_URL=<end-point-to-the-pccs>
USE_SECURE_CERT=TRUE
```

Run as a container
```bash
docker run \
    -v $(pwd)/sgx_default_qcnl.conf:/etc/sgx_default_qcnl.conf \
    -v /var/lib/vc/dragonball:/var/lib/vc/dragonball \
    --device /dev/sgx_enclave \
    --device /dev/sgx_provision \
    --privileged \
    -d \
    al3-tdx-qpl:dcap-1.15
```
