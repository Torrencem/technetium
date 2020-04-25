
func evil_func() {
	hidden_counter = 0
	func to_return() {
		hidden_counter += 1
		return hidden_counter
	}
	return to_return
}

counter = evil_func()

a = counter()
b = counter()
c = counter()

print(a, b, c)
