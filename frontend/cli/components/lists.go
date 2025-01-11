package components

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
)

type lists struct {
	choices  []string
	cursor   int
	selected string
}

func NewLists() lists {
	return lists{
		choices:  []string{"Search", "Library"},
		selected: "",
	}
}

func (m lists) Init() tea.Cmd {
	return nil
}

func (m lists) Update(msg tea.Msg) (lists, tea.Cmd) {
	// return type must be model at these low levels, but tea.Model at the highest
	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.String() {
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter", " ":
			if m.selected == m.choices[m.cursor] {
				m.selected = ""
			} else {
				m.selected = m.choices[m.cursor]
			}
		}
	}

	return m, nil
}

func (m lists) View() string {
	s := "Your Lists:\n\n"

	for i, choice := range m.choices {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}

		checked := " "
		if choice == m.selected {
			checked = "x"
		}

		s += fmt.Sprintf("%s [%s] %s\n", cursor, checked, choice)
	}

	return s
}
