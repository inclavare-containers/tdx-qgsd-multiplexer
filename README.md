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

Parameters:
- `--qgs <path to qgs socket file>`: default value is `/var/run/tdx-qgs/qgs.socket`.
- `--vsock_name <vsock file name>`: default is `kata.hvsock_40`.
- `--vsock_path <vsock path>`: default is `/var/lib/vc/dragonball`.

The logic is the notifier will watch directories creations under `vsock_path`. Any new directory creation
would cause a creation of `root/<vsock_name>` under that dir. This file is a unix socket file,
its written end will be bound to `<qgs>`.

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

*warning*: The version of sgx-qpl lib is 1.21

Build the docker image with tdx qpl and qgsd on Anolis8.6. (DCAP 1.21)
```bash
make image
```

Ensure the region id where PCCS of aliyun lies in, e.g. `cn-beijing`.

Run as a container
```bash
docker run \
    -v $(pwd)/sgx_default_qcnl.conf:/etc/sgx_default_qcnl.conf \
    -v /var/lib/vc/dragonball:/var/lib/vc/dragonball \
    --device /dev/sgx_enclave \
    --device /dev/sgx_provision \
    --privileged \
    --env REGION_ID=cn-beijing \
    -d \
    xynnn007/al3-tdx-qpl:yunqi-4
```
