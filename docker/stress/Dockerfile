ARG MYS_TOOLS_IMAGE_TAG

FROM mysten/mys-tools:$MYS_TOOLS_IMAGE_TAG

ARG MYS_TOOLS_IMAGE_TAG

RUN apt-get update && apt-get -y --no-install-recommends install wget=1.21-1+deb11u1 \
        iputils-ping netcat procps bind9-host bind9-dnsutils curl iproute2 git ca-certificates awscli

# stress needs access to examples/move/basics
RUN git clone https://github.com/MystenLabs/mys.git ; \
        cd mys ; \
        git checkout $MYS_TOOLS_IMAGE_TAG ; \
        cd .. ; \
        mv mys/* .

ARG MYS_TOOLS_IMAGE_TAG
RUN echo $MYS_TOOLS_IMAGE_TAG > /var/run/mys_commit

COPY ./entrypoint.sh .

CMD ["./entrypoint.sh"]
