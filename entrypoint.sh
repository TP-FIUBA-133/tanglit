#!/bin/sh

build() {
  export NVM_DIR="$HOME/.nvm"
  [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
  cd $TANGLIT_DIR/frontend
  npm install
  make build
}

case "${1}" in

  build)
    build
  ;;

  *)
    exec "${@}"
  ;;

esac
