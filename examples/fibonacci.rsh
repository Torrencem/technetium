
fun fibonacci(n) {
	case n of {
		1 | 2 => return 1
		_ => return fibonacci(n - 1) + fibonacci(n - 2)
	}
}
