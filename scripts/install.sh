#!/bin/bash

# SOURCE: https://github.com/Schniz/fnm/blob/master/.ci/install.sh

set -e

INSTALL_DIR="$HOME/.snm"
RELEASE="v0.8.0"
OS="$(uname -s)"

# Parse Flags
parse_args() {
  while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
    -d | --install-dir)
      INSTALL_DIR="$2"
      shift # past argument
      shift # past value
      ;;
    -s | --skip-shell)
      SKIP_SHELL="true"
      shift # past argument
      ;;
    # --force-install | --force-no-brew)
    #   echo "\`--force-install\`: I hope you know what you're doing." >&2
    #   FORCE_INSTALL="true"
    #   shift
    #  ;;
    -r | --release)
      RELEASE="$2"
      shift # past release argument
      shift # past release value
      ;;
    *)
      echo "Unrecognized argument $key"
      exit 1
      ;;
    esac
  done
}

set_filename() {
  if [ "$OS" == "Linux" ]; then
    # Based on https://stackoverflow.com/a/45125525
    case "$(uname -m)" in
      arm | armv7*)
        FILENAME="snm-$RELEASE-armv7-unknown-linux-gnueabihf"
        ;;
      aarch* | armv8*)
        FILENAME="snm-$RELEASE-aarch64-unknown-linux-gnu"
        ;;
      *)
        FILENAME="snm-$RELEASE-x86_64-unknown-linux-gnu"
    esac
  # elif [ "$OS" == "Darwin" ] && [ "$FORCE_INSTALL" == "true" ]; then
  elif [ "$OS" == "Darwin" ]; then
    FILENAME="snm-x86_64-apple-darwin"
    # USE_HOMEBREW="false"
    echo "Downloading the latest snm binary from GitHub..."
    # echo "  Pro tip: it's eaiser to use Homebrew for managing snm in MacOS."
    # echo "           Remove the \`--force-no-brew\` so it will be easy to upgrade."
  # elif [ "$OS" == "Darwin" ]; then
  #   USE_HOMEBREW="true"
  #   echo "Downloading snm using Homebrew..."
  else
    echo "OS $OS is not supported."
    echo "If you think that's a bug - please file an issue to https://github.com/numToStr/snm/issues"
    exit 1
  fi
}

download_snm() {
  # if [ "$USE_HOMEBREW" == "true" ]; then
  #   brew install snm
  # else
    if [ "$RELEASE" == "latest" ]; then
      URL="https://github.com/numToStr/snm/releases/latest/download/$FILENAME.tar.gz"
    else
      URL="https://github.com/numToStr/snm/releases/download/$RELEASE/$FILENAME.tar.gz"
    fi

    DOWNLOAD_DIR=$(mktemp -d)

    echo "Downloading $URL..."

    mkdir -p "$INSTALL_DIR" &>/dev/null

    if ! curl --progress-bar --fail -L "$URL" -o "$DOWNLOAD_DIR/$FILENAME.tar.gz"; then
      echo "Download failed.  Check that the release/filename are correct."
      exit 1
    fi

    tar -xf "$DOWNLOAD_DIR/$FILENAME.tar.gz" -C "$DOWNLOAD_DIR"

    if [ -f "$DOWNLOAD_DIR/snm" ]; then
      mv "$DOWNLOAD_DIR/snm" "$INSTALL_DIR/snm"
    else
      mv "$DOWNLOAD_DIR/$FILENAME/snm" "$INSTALL_DIR/snm"
    fi

    chmod u+x "$INSTALL_DIR/snm"

    echo "Installed at $INSTALL_DIR/snm"
  # fi
}

check_dependencies() {
  echo "Checking dependencies for the installation script..."

  echo -n "Checking availability of curl... "
  if hash curl 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  echo -n "Checking availability of tar... "
  if hash tar 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  # if [ "$USE_HOMEBREW" = "true" ]; then
  #   echo -n "Checking availability of Homebrew (brew)... "
  #   if hash brew 2>/dev/null; then
  #     echo "OK!"
  #   else
  #     echo "Missing!"
  #     SHOULD_EXIT="true"
  #   fi
  # fi

  if [ "$SHOULD_EXIT" = "true" ]; then
    exit 1
  fi
}

ensure_containing_dir_exists() {
  local CONTAINING_DIR
  CONTAINING_DIR="$(dirname "$1")"
  if [ ! -d "$CONTAINING_DIR" ]; then
    echo " >> Creating directory $CONTAINING_DIR"
    mkdir -p "$CONTAINING_DIR"
  fi
}

setup_shell() {
  CURRENT_SHELL="$(basename "$SHELL")"

  if [ "$CURRENT_SHELL" == "zsh" ]; then
    CONF_FILE=${ZDOTDIR:-$HOME}/.zshrc
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Zsh. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # snm'
    echo '  export PATH='"$INSTALL_DIR"':$PATH'
    echo '  eval "`snm env zsh`"'

    echo '' >>$CONF_FILE
    echo '# snm' >>$CONF_FILE
    echo 'export PATH='$INSTALL_DIR':$PATH' >>$CONF_FILE
    echo 'eval "`snm env zsh`"' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" == "fish" ]; then
    CONF_FILE=$HOME/.config/fish/conf.d/snm.fish
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Fish. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # snm'
    echo '  set PATH '"$INSTALL_DIR"' $PATH'
    echo '  snm env fish | source'

    echo '# snm' >>$CONF_FILE
    echo 'set PATH '"$INSTALL_DIR"' $PATH' >>$CONF_FILE
    echo 'snm env fish | source' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" == "bash" ]; then
    if [ "$OS" == "Darwin" ]; then
      CONF_FILE=$HOME/.profile
    else
      CONF_FILE=$HOME/.bashrc
    fi
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # snm'
    echo '  export PATH='"$INSTALL_DIR"':$PATH'
    echo '  eval "`snm env bash`"'

    echo '' >>$CONF_FILE
    echo '# snm' >>$CONF_FILE
    echo 'export PATH='"$INSTALL_DIR"':$PATH' >>$CONF_FILE
    echo 'eval "`snm env bash`"' >>$CONF_FILE

  else
    echo "Could not infer shell type. Please set up manually."
    exit 1
  fi

  echo ""
  echo "In order to apply the changes, open a new terminal or run the following command:"
  echo ""
  echo "  source $CONF_FILE"
}

parse_args "$@"
set_filename
check_dependencies
download_snm
if [ "$SKIP_SHELL" != "true" ]; then
  setup_shell
fi
