#!/usr/bin/env bash

set -e

if [ $# -lt 2 ]; then
	echo "Missing arguments. Received $#, requires 2."
	exit 1
fi

script_path=$(dirname $0)
font=$1
size=$2
output=
if [[ -z $3 ]]; then
	output=./texture-atlas.png
else
	output=$3
	if [[ -d $3 ]]; then
		output="$3/texture-atlas.png"
	elif [[ -f $3 ]]; then
		output="$3"
	else
		echo "'$3' is not a valid file or path."
		exit 1
	fi
fi
min_code_point=1
max_code_point=1280

tmpdir=/tmp/generate-sdf-atlas
mkdir -p "$tmpdir"

for code_point in $(seq $min_code_point $max_code_point); do
	echo "Generating signed distance field for code point $code_point..."
	printf -v formatted_code_point "%05d\n" $code_point
	msdfgen msdf -font "$font" "$code_point" -o "$tmpdir/$formatted_code_point.png" -size "$size" "$size" -autoframe
done

echo "Combining signed distance fields..."
montage -tile 64x -geometry +1+1 -background black "$tmpdir"/* "$output"
echo "Signed distance field created as \"$output\"."

echo "Cleaning up temporary files..."
rm -r "$tmpdir"
