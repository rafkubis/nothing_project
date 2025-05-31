#! /bin/bash

. ${PROJECT_USER_DATA}/git/git_config.sh

git config --global --add safe.directory ${WORKDIR}
git config --global user.email ${GIT_USER_EMAIL}
git config --global user.name ${GIT_USER_NAME}