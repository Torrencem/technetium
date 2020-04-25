
my_counter = 1000

func should_give_two() {
	my_counter = 0
	func increase() {
		my_counter += 1
	}
	increase()
	increase()
	return my_counter
}


print(should_give_two())
print(my_counter)
