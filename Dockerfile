FROM rust:latest as builder

WORKDIR /App
COPY . /App
RUN rustc --version
RUN cargo --version

RUN cargo build --release

FROM amazoncorretto:8

### 2.Copying ContainerHealthCheck directory.
COPY --from=builder /App/target/release/container-health-check /ContainerHealthCheck/container-health-check

### 3.Make ContainerHealthCheck Executable.
RUN chmod +x /ContainerHealthCheck/container-health-check

### 4.Executing bash.
ENTRYPOINT bash