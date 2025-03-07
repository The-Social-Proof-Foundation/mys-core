ARG MYS_TOOLS_IMAGE_TAG

FROM mysten/mys-tools:$MYS_TOOLS_IMAGE_TAG AS setup

RUN apt update
RUN apt install python3 python3-pip -y

# copy configuration files to root
COPY ./new-genesis.sh /new-genesis.sh
COPY ./genesis /genesis

WORKDIR /

ARG MYS_NODE_A_TAG
ARG MYS_NODE_B_TAG
ARG MYS_RUST_LOG
ENV MYS_NODE_A_TAG=$MYS_NODE_A_TAG
ENV MYS_NODE_B_TAG=$MYS_NODE_B_TAG

RUN ./new-genesis.sh
RUN echo "MYS_NODE_A_TAG=$MYS_NODE_A_TAG" >> /.env
RUN echo "MYS_NODE_B_TAG=$MYS_NODE_B_TAG" >> /.env
RUN echo "MYS_RUST_LOG=$MYS_RUST_LOG" >> /.env

COPY ./docker-compose-antithesis.yaml /docker-compose.yaml

# when running an upgrade test, we required two docker files, one for before upgrade and one for after upgrade
# One of the files runs only the primary mys-node image, and the other runs only the mys-node-alt image.
RUN <<EOF
sed 's/mys-node-alt:${MYS_NODE_B_TAG}/mys-node:${MYS_NODE_A_TAG}/' < /docker-compose.yaml > docker-compose-1.yaml
sed 's/mys-node:${MYS_NODE_A_TAG}/mys-node-alt:${MYS_NODE_B_TAG}/' < /docker-compose.yaml > docker-compose-2.yaml
EOF

FROM scratch

COPY --from=setup /docker-compose-1.yaml /docker-compose-1.yaml
COPY --from=setup /docker-compose-2.yaml /docker-compose-2.yaml
COPY --from=setup /docker-compose.yaml /docker-compose.yaml

COPY /genesis/overlays/* /genesis/overlays/
COPY /genesis/static/* /genesis/static/
COPY --from=setup /genesis/files/* /genesis/files/
COPY --from=setup /.env /.env

