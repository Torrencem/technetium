
grep = require('grep')

search = ($ ls | grep ".rsh$").spawn().join()

files = search.stdout.lines()

for file in files {
	$ echo {file.upper()}
}
