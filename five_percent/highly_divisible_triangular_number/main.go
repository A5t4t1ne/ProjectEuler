package main

import (
	"fmt"
)

func main() {
	hit := 0
	for i := 2; ; i++ {
		var triang_num int
		for j := 1; j <= i; j++ {
			triang_num += j
		}

		div_count := 2 // 1 and self

		for i := 2; i <= int(triang_num/2)+1; i++ {
			if triang_num%i == 0 {
				div_count++
			}
		}
		// fmt.Printf("%d: %d\n", triang_num, div_count)
		if div_count > 100 && hit == 0 {
			fmt.Println(triang_num, div_count)
			hit++
		}
		if div_count > 200 && hit == 1 {
			fmt.Println(triang_num, div_count)
			hit++
		}
		if div_count > 300 && hit == 2 {
			fmt.Println(triang_num, div_count)
			hit++
		}
		if div_count > 400 && hit == 3 {
			fmt.Println(triang_num, div_count)
			hit++
		}
		if div_count > 500 {
			fmt.Println(triang_num, div_count)
			break
		}
	}
}
