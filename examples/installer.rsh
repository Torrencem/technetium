
git = require("git")
make = require("make")

# Returns an absolute path
build_dir = make_tmp_dir()

defer {
	$ rm -rf {build_dir}
}

$ git clone https://github.com/project/project.git {build_dir}

$ cd {build_dir}

$ make && make install
