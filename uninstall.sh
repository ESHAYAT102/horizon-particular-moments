#!/bin/sh
set -eu
home=${HORIZON_HOME:-${XDG_DATA_HOME:-$HOME/.local/share}/horizon}
dest="$home/horizon-mods/horizon_particular_moments.hmod"
if [ -f "$dest" ]; then
    rm -- "$dest"
    echo "Removed $dest. Restart Horizon."
else
    echo "Not installed: $dest"
fi
