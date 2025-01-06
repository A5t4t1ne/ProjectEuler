package main

import "fmt"

func main() {
    maxNum := 20
    num := maxNum

    for {
        found := true
        for i := 1; i <= maxNum; i++ {
            if num%i != 0 {
                found = false
                break
            }
        }
        if found {
            fmt.Println(num)
            break
        }
        num++
    }
}





