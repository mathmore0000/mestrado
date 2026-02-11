package main

func Multiplicar(a, b int) int {
	if b == 0 {
		return 0
	}
	return a + Multiplicar(a, b-1)
}
