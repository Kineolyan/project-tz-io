#!/bin/bash
function resolve_file() {
	local target_file=$1

	cd `dirname $target_file`
	target_file=`basename $target_file`

	# Iterate down a (possible) chain of symlinks
	while [ -L "$target_file" ]
	do
			target_file=`readlink $target_file`
			cd `dirname $target_file`
			target_file=`basename $target_file`
	done

	# Compute the canonicalized name by finding the physical path
	# for the directory we're in and appending the target file.
	local readonly phys_dir=`pwd -P`
	echo "$phys_dir/$target_file"
}

readonly THIS_FILE=$(resolve_file ${BASH_SOURCE[0]})
readonly DIR=$(dirname "$THIS_FILE")

function make_parser() {
	cd $DIR/rs-parser
	cargo build
}

function make_reader() {
	cd $DIR/java-reader
	cargo build
}

function make_core_jar() {
	cd $DIR/tzio-core
	gradle build
}

action="$1"
case $action in
parser) make_parser;;
reader) make_reader;;
core) make_core_jar;;
'')
	make_parser
	make_core_jar
	make_reader ;;
esac