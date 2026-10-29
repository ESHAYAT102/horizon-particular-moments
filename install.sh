#!/bin/sh
set -eu
version=v0.1.2
home=${HORIZON_HOME:-${XDG_DATA_HOME:-$HOME/.local/share}/horizon}
dest="$home/horizon-mods/horizon_particular_moments.hmod"
url="https://github.com/ESHAYAT102/horizon-particular-moments/releases/download/$version/horizon_particular_moments.hmod"
mkdir -p "$(dirname "$dest")"
tmp=$(mktemp "$(dirname "$dest")/.particular-moments.XXXXXX")
trap 'rm -f "$tmp"' EXIT HUP INT TERM
curl -fsSL "$url" -o "$tmp"
if [ -e "$dest" ]; then
    backup="$dest.backup.$(date -u +%Y%m%dT%H%M%SZ)"
    cp -p "$dest" "$backup"
    echo "Saved previous mod: $backup"
fi
mv -f "$tmp" "$dest"
echo "Installed $dest. Restart Horizon and enable Horizon Particular Moments in Options > Mods."
