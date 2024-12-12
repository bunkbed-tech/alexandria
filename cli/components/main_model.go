package components

import (
	"fmt"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"golang.org/x/term"
	"os"
)

var (
	modelStyle = lipgloss.NewStyle().
			Align(lipgloss.Center, lipgloss.Center).
			BorderStyle(lipgloss.HiddenBorder())
	focusedModelStyle = lipgloss.NewStyle().
				Align(lipgloss.Center, lipgloss.Center).
				BorderStyle(lipgloss.NormalBorder()).
				BorderForeground(lipgloss.Color("69"))
	helpStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("241"))
)

type sessionState uint

const (
	leftView sessionState = iota
	rightView
)

type mainModel struct {
	state  sessionState
	left   lists
	right  content
	width  int
	height int
}

func NewMainModel() mainModel {
	m := mainModel{state: leftView}
	m.left = NewLists()
	m.right = NewContent()
	m.width, m.height, _ = term.GetSize(int(os.Stdin.Fd()))
	return m
}

func (m mainModel) Init() tea.Cmd {
	return tea.Batch(m.left.Init(), m.right.Init())
}

func (m mainModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	// return type must be tea.Model at the highest level
	var cmd tea.Cmd
	var cmds []tea.Cmd
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
	case tea.KeyMsg:
		switch msg.String() {
		case "tab":
			if m.state == leftView {
				m.state = rightView
			} else {
				m.state = leftView
			}
		}
		switch m.state {
		// update whichever model is focused
		case leftView:
			m.left, cmd = m.left.Update(msg)
			cmds = append(cmds, cmd)
			switch msg.String() {
			case "enter", " ":
				m.right, cmd = m.right.Refresh(m.left.selected)
				cmds = append(cmds, cmd)
			}
		case rightView:
			m.right, cmd = m.right.Update(msg)
			cmds = append(cmds, cmd)
		}
	}
	return m, tea.Batch(cmds...)
}

func (m mainModel) View() string {
	var s string
	model := m.currentFocusedModel()
	if m.state == leftView {
		s += lipgloss.JoinHorizontal(lipgloss.Top,
			focusedModelStyle.
				Height(m.height).
				Width(15).
				Render(fmt.Sprintf("%4s", m.left.View())),
			modelStyle.
				Height(m.height).
				Width(m.width-15).
				Render(m.right.View()))
	} else {
		s += lipgloss.JoinHorizontal(lipgloss.Top,
			modelStyle.
				Width(15).
				Height(m.height).
				Render(fmt.Sprintf("%4s", m.left.View())),
			focusedModelStyle.
				Width(m.width-15).
				Height(m.height).
				Render(m.right.View()))
	}
	s += helpStyle.Render(fmt.Sprintf("\ntab: focus %s • q: exit\n", model))
	return s
}

func (m mainModel) currentFocusedModel() string {
	if m.state == leftView {
		return "left"
	}
	return "right"
}
