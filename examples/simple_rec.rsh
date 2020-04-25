
func my_func(x) {
	print(x)
	if x < 1 {
		return 0
	}
	return my_func(x - 1) + 1
}

return my_func(10)
