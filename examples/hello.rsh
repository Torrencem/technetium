
my_num = 100

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3}
} else {
	print("Number too small!")
}
