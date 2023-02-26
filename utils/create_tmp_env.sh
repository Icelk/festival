#!/usr/bin/env bash

# Sets up a packaging environment in [/tmp]

set -ex
[[ $PWD = */festival/utils ]] && cd ..
[[ $PWD = */festival ]]

# Make sure the folder doesn't already exist
GIT_COMMIT=$(cat .git/refs/heads/main)
FOLDER="gupax_${GIT_COMMIT}"
[[ ! -e /tmp/${FOLDER} ]]

mkdir /tmp/${FOLDER}
cp -r utils/* /tmp/${FOLDER}/
cp CHANGELOG.md /tmp/${FOLDER}/skel/

set +ex

echo
ls --color=always /tmp/${FOLDER}
echo "/tmp/${FOLDER} ... OK"
