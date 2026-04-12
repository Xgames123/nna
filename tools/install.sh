#!/bin/bash
set -e

HELP=$(cat <<END
Usage: $0 [-u]
Builds and installs the nna dev tools (requires rust compiler)

Options:
 -u   uninstall instead of install
END
)

TOOLS=(nnaasm)

if [[ "$1" = "--help" ]] ; then
  echo "$HELP"
fi

if [ "$(whoami)" = "root" ] ; then
  echo "Don't run as root."
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
  echo "Copying files to /usr/local/bin..."
  sudo cp target/release/$tool /usr/local/bin/$tool
  sudo chmod +x /usr/local/bin/$tool
done

echo "DONE (Run 'tools/install.sh -u' to uninstall)"
