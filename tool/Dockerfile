FROM registry.cn-hangzhou.aliyuncs.com/alinux/alinux3 AS builder1

RUN yum -y install systemd
RUN yum -y groupinstall 'Development Tools'
RUN yum -y install \
ocaml \
ocaml-ocamlbuild \
wget \
python2 \
openssl-devel \
git \
openssl-devel \
libcurl-devel \
protobuf-devel \
cmake \
rpm-build \
createrepo \
yum-utils \
openssl-devel \
libcurl-devel \
protobuf-devel \
vim \
tree \
dos2unix \
git-lfs \
numactl-libs \
numactl-devel \
numactl \
pkgconf \
boost-devel \
protobuf-lite-devel \
protobuf-c-compiler \
protobuf-c-devel
RUN alternatives --set python /usr/bin/python2

RUN cd /root && git clone https://github.com/intel/linux-sgx.git; \
cd /root/linux-sgx && git reset --hard effae6280234302a12169f89c561b96e54d80723; \
cd /root/linux-sgx/external/dcap_source && git reset --hard 68a77a852cd911a44a97733aec870e9bd93a3b86; \
cd /root/linux-sgx && make preparation; \
make sdk_install_pkg; \
echo yes | /root/linux-sgx/linux/installer/bin/sgx_linux_x64_sdk_*.bin; \
source /root/linux-sgx/sgxsdk/environment; \
make rpm_local_repo

RUN yum -y install yum-utils; \
yum-config-manager --add-repo file:////root/linux-sgx/linux/installer/rpm/sgx_rpm_local_repo; \
yum install -y --setopt=install_weak_deps=False --nogpgcheck sgx-dcap-pccs.x86_64 tdx-qgs libsgx-dcap-default-qpl

RUN cd /root && git clone https://github.com/jialez0/SGXDataCenterAttestationPrimitives.git; \
cd /root/SGXDataCenterAttestationPrimitives/QuoteGeneration/quote_wrapper/qgs; \
source /root/linux-sgx/sgxsdk/environment; \
make clean && make

FROM registry.cn-hangzhou.aliyuncs.com/alinux/alinux3 AS builder2
RUN yum -y install git gcc

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git && \
    cd tdx-qgsd-multiplexer && \
    cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release

FROM registry.cn-hangzhou.aliyuncs.com/alinux/alinux3

COPY --from=builder2 /tdx-qgsd-multiplexer/target/release/qgsd-multiplexer /usr/local/bin
COPY --from=builder1 /root/SGXDataCenterAttestationPrimitives/QuoteGeneration/quote_wrapper/qgs/qgs /usr/local/bin

COPY --from=builder1 /root/linux-sgx/linux/installer/rpm/sgx_rpm_local_repo /tmp/sgx_rpm_local_repo

RUN yum -y install yum-utils && yum-config-manager --add-repo file:///tmp/sgx_rpm_local_repo; \
    yum install -y --setopt=install_weak_deps=False --nogpgcheck sgx-dcap-pccs.x86_64 tdx-qgs libsgx-dcap-default-qpl supervisor

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

RUN rm -rf /tmp/* && mkdir -p /var/run/tdx-qgs

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
