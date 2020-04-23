
func fib(n) {
    a = 1
    b = 1
    while n > 0 {
        tmp = b
        b += a
        a = tmp
        n = n - 1
	print(b)
    }
    return b
}

return fib(10)
