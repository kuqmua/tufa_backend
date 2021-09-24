FROM rust:latest as rust-latest-base-image
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup install nightly
WORKDIR /usr/src/tufa_backend
COPY . .
RUN apt-get update
RUN apt-get upgrade
RUN apt install musl-tools -y
# RUN musl-gcc
# RUN apt install pkg-config
# RUN apt install libssl-dev

# RUN git clone git://git.openssl.org/openssl.git
# RUN cd openssl ./config --openssldir=/usr/local/ssl  && \
#     make && \
#     make test && \
#     make install
# RUN make 
# RUN make test 
# RUN make install

# RUN apt install openssl1.0 libssl1.0.0 
# #libssl1.0-dev
# RUN cargo clean
# RUN OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu" 
# RUN OPENSSL_INCLUDE_DIR="/usr/include/openssl" 

# RUN apt-get install pkg-config
# RUN apt-get install libssl-dev

# RUN apt-get purge --auto-remove openssl -y
# the SSL certificate is invalid; class=Ssl (16); code=Certificate (-17)
# RUN apt-get install openssl



# RUN openssl version
# RUN openssl version -d
# RUN openssl version -a
# OpenSSL 1.1.1k  25 Mar 2021
# built on: Tue Aug 24 08:28:12 2021 UTC
# platform: debian-amd64
# options:  bn(64,64) rc4(8x,int) des(int) blowfish(ptr) 
# compiler: gcc -fPIC -pthread -m64 -Wa,--noexecstack -Wall -Wa,--noexecstack -g -O2 -ffile-prefix-map=/build/openssl-MBf3dh/openssl-1.1.1k=. -fstack-protector-strong -Wformat -Werror=format-security -DOPENSSL_USE_NODELETE -DL_ENDIAN -DOPENSSL_PIC -DOPENSSL_CPUID_OBJ -DOPENSSL_IA32_SSE2 -DOPENSSL_BN_ASM_MONT -DOPENSSL_BN_ASM_MONT5 -DOPENSSL_BN_ASM_GF2m -DSHA1_ASM -DSHA256_ASM -DSHA512_ASM -DKECCAK1600_ASM -DRC4_ASM -DMD5_ASM -DAESNI_ASM -DVPAES_ASM -DGHASH_ASM -DECP_NISTZ256_ASM -DX25519_ASM -DPOLY1305_ASM -DNDEBUG -Wdate-time -D_FORTIFY_SOURCE=2
# OPENSSLDIR: "/usr/lib/ssl"
# ENGINESDIR: "/usr/lib/x86_64-linux-gnu/engines-1.1"
# Seeding source: os-specific


# RUN export OPENSSL_DIR="/usr/local/ssl"
# RUN export OPENSSL_DIR="/usr/lib/ssl"
# ARG OPENSSL_LIB_DIR=/usr/lib/ssl

# ARG OPENSSL_INCLUDE_DIR=/usr/lib/ssl
# ARG OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu" 
# ARG OPENSSL_INCLUDE_DIR="/usr/include/openssl" 
# ENV OPENSSL_DIR=/usr/local/musl/ \
#     OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
#     DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ 


# ARG OPENSSL_DIR=/usr/lib/ssl
RUN ls
# RUN cd ~/  && ls



#  cd rust-openssl cargo build
RUN cargo +nightly build --target=x86_64-unknown-linux-musl --release

FROM alpine:latest
RUN addgroup -g 1000 tufa_backend
RUN adduser -D -s /bin/sh -u 1000 -G tufa_backend tufa_backend
WORKDIR /home/rust_rest/bin/
COPY --from=rust-latest-base-image /usr/src/tufa_backend/target/x86_64-unknown-linux-musl/release/tufa_backend .
RUN ls
RUN chown tufa_backend:tufa_backend tufa_backend
USER tufa_backend
EXPOSE 8080
CMD [ "./tufa_backend" ]