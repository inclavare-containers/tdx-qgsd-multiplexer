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

build and launch the qgsd-multiplexer

```bash
git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git
cd tdx-qgsd-multiplexer
cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release
./target/release/qgsd-multiplexer
```

Now it will automatically multiplex the qgsd to different tdx guests.

### Build image containing tdx QPL stack and launch

```bash
git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git
cd tdx-qgsd-multiplexer
sudo bash tool/build.sh
```

Get proper pccs configration
```bash
token=$(curl -s -X PUT -H "X-aliyun-ecs-metadata-token-ttl-seconds: 5" "http://100.100.100.200/latest/api/token")
region_id=$(curl -s -H "X-aliyun-ecs-metadata-token: $token" http://100.100.100.200/latest/meta-data/region-id)

PCCS_URL=https://sgx-dcap-server-vpc.${region_id}.aliyuncs.com/sgx/certification/v3/
cat > /etc/sgx_default_qcnl.conf << EOF
# PCCS server address
PCCS_URL=${PCCS_URL}
# To accept insecure HTTPS cert, set this option to FALSE
USE_SECURE_CERT=TRUE
EOF
```

Run as a container
```bash
docker run -v /etc/sgx_default_qcnl.conf:/etc/sgx_default_qcnl.conf -v /var/lib/vc/dragonball:/var/lib/vc/dragonball --device /dev/sgx_enclave --device /dev/sgx_provision --privileged -d al3-tdx-qpl
```
