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
	var useAscii = false
	if m.width > 162 && m.height > 22 {
		useAscii = true
	}
	var header = ""
	if useAscii {
		header += renderImage()
	} else {
		header += "Welcome to Alexandria!!!"
	}

	// Login page styling
	headerStyle := lipgloss.NewStyle().
		Width(m.width).
		AlignHorizontal(lipgloss.Center)
	loginStyle := focusedModelStyle.
		Width(m.width - 2).
		Height(m.height / 2)
	loginContentStyle := lipgloss.NewStyle().
		AlignHorizontal(lipgloss.Left).
		Width(34) // textinput.CharLimit + 2
	loginContent := lipgloss.JoinVertical(
		lipgloss.Top,
		loginStyle.Render(loginContentStyle.Render(m.login.View())),
		helpStyle.Render("enter: evaluate login "))
	loginPage := lipgloss.JoinVertical(
		lipgloss.Top,
		headerStyle.
			PaddingBottom((m.height-lipgloss.Height(header)-lipgloss.Height(loginContent))/2).
			PaddingTop((m.height-lipgloss.Height(header)-lipgloss.Height(loginContent))/2).
			Render(fmt.Sprintf("%s", header)),
		loginContent)

	// Create view strings based on active window
	// Need to clean up the hardcoded offsets used to show the border properly
	var s string
	model := m.currentFocusedModel()
	if m.state == leftView {
		s += lipgloss.JoinHorizontal(lipgloss.Top,
			focusedModelStyle.
				Height(m.height-3).
				Width(15).
				Render(fmt.Sprintf("%4s", m.left.View())),
			modelStyle.
				Height(m.height-3).
				Width(m.width-19).
				Render(m.right.View()))
		s += helpStyle.Render(fmt.Sprintf("\ntab: focus %s ", model))
	} else if m.state == rightView {
		s += lipgloss.JoinHorizontal(lipgloss.Top,
			modelStyle.
				Width(15).
				Height(m.height-3).
				Render(fmt.Sprintf("%4s", m.left.View())),
			focusedModelStyle.
				Width(m.width-19).
				Height(m.height-3).
				Render(m.right.View()))
		s += helpStyle.Render(fmt.Sprintf("\ntab: focus %s ", model))
	} else {
		s += loginPage
	}
	s += helpStyle.Render("• q/ctrl+c: exit")
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

func renderImage() string {
	// Ignore weird offset by one I can't figure out right now
	asciiArt := `
/ ____      ____      __                                       _                 _       __                                      __           _          _  _  _
//|_  _|    |_  _|    [  |                                     / |_              / \     [  |                                    |  ]         (_)        | || || |
//  \ \  /\  / /.---.  | |  .---.   .--.   _ .--..--.  .---.  '| |-' .--.       / _ \     | | .---.  _   __  ,--.   _ .--.   .--.| |  _ .--.  __   ,--.  | || || |
//   \ \/  \/ // /__\\ | | / /''\]/ .''\ \[ '.-. .-. |/ /__\\  | | / .''\ \    / ___ \    | |/ /__\\[ \ [  ] '_\ : [ '.-. |/ /''\' | [ '/''\][  | ''_\ : | || || |
//    \  /\  / | \__., | | | \__. | \__. | | | | | | || \__.,  | |,| \__.|   _/ /   \ \_  | || \__., > '  < // | |, | | | || \__/  |  | |     | | // | |,|_||_||_|
//     \/  \/   '.__.'[___]'.___.' '.__.' [___||__||__]'.__.'  \__/ '.__.'  |____| |____|[___]'.__.'[__]'\_]\'-;__/[___||__]'.__.;__][___]   [___]\'-;__/(_)(_)(_)

`
	return asciiArt
}
