FROM rust:1.67.1 as build

WORKDIR /build
ADD . /build/
RUN cd /build && bash build.sh

FROM ubuntu

WORKDIR /output
COPY --from=build /build/output /output
ENTRYPOINT [ "sh", "run.sh" ]

