package main

import (
    "fmt"
)

// Function to calculate the number of divisors of a number
func countDivisors(n int) int {
    count := 0
    for i := 1; i*i <= n; i++ {
        if n%i == 0 {
            count++ // i is a divisor
            if i != n/i {
                count++ // n/i is also a divisor
            }
        }
    }
    return count
}

// Function to find the first triangular number with over 500 divisors
func findTriangularNumber() int {
    n := 1
    triangleNumber := 0

    for {
        triangleNumber += n // Generate the nth triangular number
        if countDivisors(triangleNumber) > 500 {
            return triangleNumber // Return the first triangular number with over 500 divisors
        }
        n++
    }
}

func main() {
    result := findTriangularNumber()
    fmt.Println("The first triangular number with over 500 divisors is:", result)
}
