package main

func MergeSort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	middleI := int(len(arr) / 2)
	leftHalf := arr[0:middleI]
	rightHalf := arr[middleI:]

	left := MergeSort(leftHalf)
	right := MergeSort(rightHalf)

	return merge(left, right)
}

func merge(left, right []int) []int {
	mergedArray := make([]int, 0, len(left)+len(right))
	iLeft, iRight := 0, 0

	for iLeft < len(left) && iRight < len(right) {
		if left[iLeft] < right[iRight] {
			mergedArray = append(mergedArray, left[iLeft])
			iLeft++
		} else {
			mergedArray = append(mergedArray, right[iRight])
			iRight++
		}
	}

	for iLeft < len(left) {
		mergedArray = append(mergedArray, left[iLeft])
		iLeft++
	}

	for iRight < len(right) {
		mergedArray = append(mergedArray, right[iRight])
		iRight++
	}

	return mergedArray
}
