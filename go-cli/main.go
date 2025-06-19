package main

import (
	"github.com/common-nighthawk/go-figure"
	"github.com/fatih/color"
)

func main() {
	// 1. Create the ASCII Art Header
	myFigure := figure.NewFigure("My App", "slant", true)
	myFigure.Print()
	color.Cyan("A super fast CLI tool\n")


	// 2. Print the "Hello World" message with rich text
	color.Yellow("Hello, World!")

	// 3. Add a hint for future expansion
	p := color.New(color.FgGreen).Add(color.Underline)
	p.Println("\nNext steps: Try adding a new command!")
}