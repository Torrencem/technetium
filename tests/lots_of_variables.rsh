
num_calls = 0
a = 10
b = 15
c = 20
d = 30.2

func determinant(a, b, x, y) {
	num_calls += 1
	return a * b - x * y
}

print(determinant(a, b, c, d))  # Should be -454
print(determinant(c, d, b, a))  # Should be 454
print(num_calls)


