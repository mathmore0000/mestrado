package main

// 13 / 4 (maior ou igual) {-4}
// 9 / 4 (maior ou igual) {-4}
// 5 / 4 (maior ou igual) {-4}
// 1 / 4 (menor) {return 1/4}

func Restos(a, b int) int {
	if a < b {
		return a
	}
	return Restos(a-b, b)
}
