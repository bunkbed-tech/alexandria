package components

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
)

type content struct {
	choices  []string
	cursor   int
	selected map[int]struct{}
}

func NewContent() content {
	return content{
		choices:  []string{},
		selected: make(map[int]struct{}), // what is this syntax?
	}
}

func (m content) Init() tea.Cmd {
	return nil
}

func (m content) Refresh(left_selected string) (content, tea.Cmd) {
	m = NewContent()
	if left_selected != "" {
		m.choices = []string{left_selected, left_selected, left_selected}
	}

	return m, nil
}

func (m content) Update(msg tea.Msg) (content, tea.Cmd) {
	// return type must be model at these low levels, but tea.Model at the highest
	// grab the selected list from the left component

	switch msg := msg.(type) {

	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter", " ":
			_, ok := m.selected[m.cursor]
			if ok {
				delete(m.selected, m.cursor)
			} else {
				m.selected[m.cursor] = struct{}{} // how is this different from make(...)
			}
		}
	}

	return m, nil
}

func (m content) View() string {
	s := ""

	for i, choice := range m.choices {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}

		checked := " "
		_, ok := m.selected[i]
		if ok {
			checked = "x"
		}

		s += fmt.Sprintf("%s [%s] %s\n", cursor, checked, choice)
	}

	if s == "" {
		s += "Welcome to Alexandria!"
	}

	return s
}
