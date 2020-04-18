
grep = require('grep')

search = ($ ls | grep ".rsh$").join()

files = search.stdout.lines()

for file in files {
	$ echo {file.upper()}
}
