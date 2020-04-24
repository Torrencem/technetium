
func fib(n) {
	print(n)
	print(n < 3)
	if n < 3 {
		return 1
	}
	return fib(n - 1) + fib(n - 2)
}

print(fib(30))
