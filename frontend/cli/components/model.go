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
	loginView
)

type model struct {
	state  sessionState
	left   lists
	right  content
	login  login
	width  int
	height int
}

func NewModel() model {
	m := model{state: loginView}
	m.login = NewLogin()
	m.left = NewLists()
	m.right = NewContent()
	m.width, m.height, _ = term.GetSize(int(os.Stdin.Fd()))
	return m
}

func (m model) Init() tea.Cmd {
	return m.login.Init()
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	// return type must be tea.Model at the highest level
	var cmd tea.Cmd
	var cmds []tea.Cmd
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
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
			case "tab":
				m.state = rightView
			}
		case rightView:
			m.right, cmd = m.right.Update(msg)
			cmds = append(cmds, cmd)
			switch msg.String() {
			case "tab":
				m.state = leftView
			}
		case loginView:
			m.login, cmd = m.login.Update(msg)
			cmds = append(cmds, cmd)
			switch msg.String() {
			case "enter":
				if m.login.EvaluateLogin() {
					cmds = append(cmds, tea.Batch(m.left.Init(), m.right.Init()))
					m.state = leftView
				}
			}
		}
	}
	return m, tea.Batch(cmds...)
}

func (m model) View() string {
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
		s += helpStyle.Render(fmt.Sprintf("\n\ntab: focus %s ", model))
	} else if m.state == rightView {
		s += lipgloss.JoinHorizontal(lipgloss.Top,
			modelStyle.
				Width(15).
				Height(m.height).
				Render(fmt.Sprintf("%4s", m.left.View())),
			focusedModelStyle.
				Width(m.width-15).
				Height(m.height).
				Render(m.right.View()))
		s += helpStyle.Render(fmt.Sprintf("\n\ntab: focus %s ", model))
	} else {
		s += focusedModelStyle.
			Width(m.width).
			Height(m.height).
			Render(lipgloss.NewStyle().
				AlignHorizontal(lipgloss.Left).
				Render(m.login.View()))
		s += helpStyle.Render(fmt.Sprintf("\n\nenter: evaluate %s ", model))
	}
	s += helpStyle.Render("• q: exit\n")
	return s
}

func (m model) currentFocusedModel() string {
	if m.state == leftView {
		return "left"
	} else if m.state == rightView {
		return "right"
	}
	return "login"
}
