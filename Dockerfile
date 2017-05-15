FROM alpine
LABEL maintainer "mozamimy (Moza USANE) <alice@mozami.me>"

ARG URL=https://github.com/mozamimy/telescreen.git
ARG VERSION=0.1.2
ARG ENTRYKIT_VERSION=0.4.0

RUN apk update
RUN apk add \
    ca-certificates \
    curl
RUN update-ca-certificates

WORKDIR /app

RUN curl -L https://github.com/mozamimy/telescreen/releases/download/v${VERSION}/telescreen-v${VERSION}-linux-musl.tar.gz | tar zxf -

RUN curl -L https://github.com/progrium/entrykit/releases/download/v${ENTRYKIT_VERSION}/entrykit_${ENTRYKIT_VERSION}_Linux_x86_64.tgz | tar zxf - && \
    mv entrykit /bin/entrykit && \
    chmod +x /bin/entrykit && \
    entrykit --symlink

COPY docker/app/config.yml.tmpl /app/config.yml.tmpl

CMD [ \
  "render", \
    "/app/config.yml", \
  "--", \
  "/app/telescreen", "--help" \
]
