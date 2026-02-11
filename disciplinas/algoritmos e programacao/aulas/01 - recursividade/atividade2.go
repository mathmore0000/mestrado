package main

// 13 / 4 (maior ou igual) {-4}
// 9 / 4 (maior ou igual) {-4}
// 5 / 4 (maior ou igual) {-4}
// 1 / 4 (menor) {return 0}

func Dividir(a, b int) int {
	if a < b {
		return 0
	}
	return 1 + Dividir(a-b, b)
}
