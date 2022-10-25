FROM rust:1.64.0-bullseye

ARG GROUP_ID=1000
ARG USER_ID=1000
ARG USER_NAME=develop
ARG WORK_DIR=/home/$USER_NAME/learn-rust

RUN groupadd --gid $GROUP_ID $USER_NAME \
    && useradd --uid $USER_ID --gid $GROUP_ID --create-home $USER_NAME
USER $USER_NAME
WORKDIR $WORK_DIR
