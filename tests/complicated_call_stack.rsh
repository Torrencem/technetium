
call_count = 0

func create_inc() {
	call_count += 1
	func inc(x) {
		call_count += 1
		return x + 1
	}
	return inc
}

func apply_func(f, arg) {
	inc = create_inc()
	call_count = inc(call_count)
	return f(arg)
}

func to_apply(x) {
	return x * 2
}

print(apply_func(to_apply, 10))
print(call_count)
