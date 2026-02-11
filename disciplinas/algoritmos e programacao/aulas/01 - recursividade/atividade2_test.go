package main

import (
	"testing"
)

func TestDividir(t *testing.T) {
	testCases := []struct {
		a, b     int
		expected int
	}{
		{a: 15, b: 3, expected: 5},
		{a: 5, b: 1, expected: 5},
		{a: 10, b: 2, expected: 5},
	}

	for _, tc := range testCases {
		result := Dividir(tc.a, tc.b)
		if result != tc.expected {
			t.Errorf("Dividir(%d, %d) = %d, expected %d", tc.a, tc.b, result, tc.expected)
		}
	}
}
