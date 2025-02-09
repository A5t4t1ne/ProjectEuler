package main

import (
	"fmt"
	"math"
)

func main() {
	const row_length = 5
	tile_len := 2
	// var tile_count int
	iterations := int(math.Ceil(float64(row_length) / float64(tile_len)))

	for i := 0; i < iterations; i++ {
		fmt.Println(i)
	}
}
