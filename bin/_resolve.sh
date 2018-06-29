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