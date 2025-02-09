package main

import (
	"fmt"
)

const (
	numCuboids = 50000
	mod        = 1000000
)

func generateS() []int {
	S := make([]int, 300001)
	for k := 1; k <= 55; k++ {
		S[k] = (100003 - 200003*k + 300007*k*k*k) % mod
	}
	for k := 56; k <= 300000; k++ {
		S[k] = (S[k-24] + S[k-55]) % mod
	}
	return S
}

func calculateVolume(S []int) int64 {
	var totalVolume int64 = 0

	for n := 1; n <= numCuboids; n++ {
		x0 := S[6*n-5] % 10000
		y0 := S[6*n-4] % 10000
		z0 := S[6*n-3] % 10000
		dx := 1 + (S[6*n-2] % 399)
		dy := 1 + (S[6*n-1] % 399)
		dz := 1 + (S[6*n] % 399)

		// Use the variables to avoid unused variable errors
		_ = x0 // Assign to blank identifier
		_ = y0 // Assign to blank identifier
		_ = z0 // Assign to blank identifier

		volume := int64(dx) * int64(dy) * int64(dz)
		totalVolume += volume
	}

	return totalVolume
}

func main() {
	S := generateS()
	totalVolume := calculateVolume(S)
	fmt.Println("Combined volume of all cuboids:", totalVolume)
}

