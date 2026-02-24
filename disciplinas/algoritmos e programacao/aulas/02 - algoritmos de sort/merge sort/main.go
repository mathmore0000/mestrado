package main

import "fmt"

func main() {
	arr := []int{1, 4, 8, 3, 6, 5, 2, 7, 9, 11}
	sortedArr := MergeSort(arr)

	fmt.Println("Before sorting:", arr)
	fmt.Println("After sorting:", sortedArr)
}
