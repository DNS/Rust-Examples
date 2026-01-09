package main

import "fmt"

func main() {
	var i int
	x := 1.0
	
	for i = 0; i < 99999999; i++ {
		x = (float64(i+i+2*i+1) - 0.379) / x
	}
	
	fmt.Printf("%f\n", x)
}
