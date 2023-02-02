FROM openanolis/anolisos:8.6 AS builder
RUN yum -y install git gcc

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone https://github.com/inclavare-containers/tdx-qgsd-multiplexer.git && \
    cd tdx-qgsd-multiplexer && \
    git checkout feat-initial && \
    cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release

FROM openanolis/anolisos:8.6

COPY --from=builder /tdx-qgsd-multiplexer/target/release/qgsd-multiplexer /usr/local/bin

COPY sgx_rpm_local_repo /home/sgx_rpm_local_repo

RUN yum -y install yum-utils && yum-config-manager --add-repo file:///home/sgx_rpm_local_repo; \
    yum install -y --setopt=install_weak_deps=False --nogpgcheck sgx-dcap-pccs.x86_64 tdx-qgs libsgx-dcap-default-qpl

RUN sed -i 's/4050/0/g' /etc/qgs.conf
