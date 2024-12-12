package main

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"os"
	"practice-cli/components"
)

func main() {
	p := tea.NewProgram(components.NewMainModel(), tea.WithAltScreen())
	_, err := p.Run()
	if err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
