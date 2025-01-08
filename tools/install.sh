#!/bin/bash
HELP=$(cat <<END
Usage: install.sh [-u]
Installs the nna dev tools
 -u   uninstall instead of install
END
)

TOOLS=(nnaasm)

if [[ "$1" = "--help" ]] ; then
  echo "$HELP"
fi

if [ "$(whoami)" = "root" ] ; then
  echo "Don't run as root"
  exit 1
fi

cd $(dirname $0)

if [ "$1" = "-u" ] ; then
  for tool in $TOOLS ; do
    sudo rm -f /usr/local/bin/$tool
  done
  exit 0
fi


for tool in $TOOLS ; do
  if ! cargo build --release --bin $tool ; then
    echo "Build failed"
    exit 1
  fi
  if ! sudo cp target/release/$tool /usr/local/bin/$tool ; then
    exit 1
  fi
  if ! sudo chmod +x /usr/local/bin/$tool ; then
    exit 1
  fi
done
