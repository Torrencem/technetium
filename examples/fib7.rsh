
func fib(n) {
	if n < 3 {
		return 1
	}
	return fib(n - 1) + fib(n - 2)
}

return fib(10)
