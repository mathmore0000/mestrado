package main

import "testing"

func TestMergeSort(t *testing.T) {
	testCases := []struct {
		arr      []int
		expected []int
	}{
		{arr: []int{1, 4, 8, 3, 6, 5, 2, 7}, expected: []int{1, 2, 3, 4, 5, 6, 7, 8}},
		{arr: []int{10, 9, 8, 7, 6, 5, 4, 3, 2, 1}, expected: []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}},
		{arr: []int{5, 3, 8, 1, 2}, expected: []int{1, 2, 3, 5, 8}},
	}

	for _, tc := range testCases {

		result := MergeSort(tc.arr)
		for i := range result {
			if result[i] != tc.expected[i] {
				t.Errorf("Expected %v but got %v", tc.expected, result)
				break
			}
		}
	}
}
