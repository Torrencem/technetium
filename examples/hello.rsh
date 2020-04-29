
my_num = 10

if my_num > 50 {
	$ cat /dev/urandom | head -c {my_num * 3}
} else {
	print(~"Number too small! The number is {my_num}")
}
