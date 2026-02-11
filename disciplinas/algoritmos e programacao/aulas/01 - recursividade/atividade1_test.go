package main

import (
	"testing"
)

func TestMultiplicar(t *testing.T) {
	testCases := []struct {
		a, b     int
		expected int
	}{
		{a: 2, b: 15, expected: 30},
		{a: 5, b: 0, expected: 0},
		{a: 0, b: 10, expected: 0},
	}

	for _, tc := range testCases {
		result := Multiplicar(tc.a, tc.b)
		if result != tc.expected {
			t.Errorf("Multiplicar(%d, %d) = %d, expected %d", tc.a, tc.b, result, tc.expected)
		}
	}
}
