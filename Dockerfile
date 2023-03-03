FROM openanolis/anolisos:8.6 AS qgsd-multiplexer-builder

LABEL maintainer="Ding Ma <xynnn@linux.alibaba.com>"

RUN yum -y install git gcc

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

ADD ./ /home/tdx-qgsd-multiplexer

RUN cd /home/tdx-qgsd-multiplexer && \
    cargo build --bin qgsd-multiplexer --features="tokio/rt-multi-thread main tokio/macros" --release

FROM openanolis/anolisos:8.6

COPY --from=qgsd-multiplexer-builder /home/tdx-qgsd-multiplexer/target/release/qgsd-multiplexer /usr/local/bin

WORKDIR /tmp

RUN curl https://download.01.org/intel-sgx/sgx-dcap/1.15/linux/distro/Anolis86/sgx_rpm_local_repo.tgz --output sgx_rpm_local_repo.tgz && \
    tar zxvf sgx_rpm_local_repo.tgz && \
    yum -y install yum-utils && yum-config-manager --add-repo file:///tmp/sgx_rpm_local_repo && \
    yum -y install epel-release && \
    yum install -y --setopt=install_weak_deps=False --nogpgcheck sgx-dcap-pccs.x86_64 tdx-qgs libsgx-dcap-default-qpl supervisor && \
    yum clean all && \
    rm -rf /tmp/*

RUN mkdir -p /var/run/tdx-qgs

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

COPY qgs.conf /etc/qgs.conf

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
