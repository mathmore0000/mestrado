package main

import (
	"testing"
)

func TestRestos(t *testing.T) {
	testCases := []struct {
		a, b     int
		expected int
	}{
		{a: 13, b: 4, expected: 1},
		{a: 22, b: 5, expected: 2},
		{a: 9, b: 2, expected: 1},
		{a: 15, b: 6, expected: 3},
	}

	for _, tc := range testCases {
		result := Restos(tc.a, tc.b)
		if result != tc.expected {
			t.Errorf("Restos(%d, %d) = %d, expected %d", tc.a, tc.b, result, tc.expected)
		}
	}
}
