FROM centos:7

ARG TOOLCHAIN=nightly-2022-12-11

RUN yum install -y git gcc java-11-openjdk

ENV PATH=$PATH:/rust/bin:/root/.cargo/bin
ENV JAVA_HOME=/usr/lib/jvm/java-11-openjdk-11.0.19.0.7-1.el7_9.x86_64

# Install rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=${TOOLCHAIN} --profile=minimal && rustc --version
RUN rustup component add rustfmt

