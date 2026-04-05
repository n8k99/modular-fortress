// Dragonpunk Desktop — Wails v3 native app for Modular Fortress.
// Opens a macOS window with webview UI connected to master_chronicle PostgreSQL.
// System tray icon for background presence.
package main

import (
	"embed"
	"log"
	"log/slog"
	"runtime"

	"github.com/wailsapp/wails/v3/pkg/application"
	"github.com/wailsapp/wails/v3/pkg/icons"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	app := application.New(application.Options{
		Name:        "Dragonpunk",
		Description: "Modular Fortress — Your desk. Your ghosts. Your data.",
		Services: []application.Service{
			application.NewService(&DbService{}),
		},
		Assets: application.AssetOptions{
			Handler: application.AssetFileServerFS(assets),
		},
		Mac: application.MacOptions{
			// Accessory policy: app can run as tray-only without dock icon when window is hidden
			ActivationPolicy: application.ActivationPolicyAccessory,
		},
	})

	// Create the main window
	mainWindow := app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:  "Dragonpunk — Modular Fortress",
		Width:  1280,
		Height: 800,
		Mac: application.MacWindow{
			InvisibleTitleBarHeight: 50,
			Backdrop:               application.MacBackdropTranslucent,
			TitleBar:               application.MacTitleBarHiddenInset,
		},
		BackgroundColour: application.NewRGB(8, 8, 13),
		URL:              "/",
	})

	// System tray
	tray := app.SystemTray.New()

	// Set icon based on platform
	if runtime.GOOS == "darwin" {
		tray.SetTemplateIcon(icons.SystrayMacTemplate)
	} else {
		tray.SetIcon(icons.SystrayLight)
		tray.SetDarkModeIcon(icons.SystrayDark)
	}
	tray.SetTooltip("Dragonpunk — Modular Fortress")

	// Tray click toggles window
	tray.OnClick(func() {
		if mainWindow.IsVisible() {
			mainWindow.Hide()
			slog.Info("tray: window hidden")
		} else {
			mainWindow.Show()
			mainWindow.Focus()
			slog.Info("tray: window shown")
		}
	})

	// Tray context menu
	trayMenu := app.NewMenu()
	trayMenu.Add("Show Dragonpunk").OnClick(func(ctx *application.Context) {
		mainWindow.Show()
		mainWindow.Focus()
		slog.Info("tray menu: show")
	})
	trayMenu.AddSeparator()
	trayMenu.Add("Quit").OnClick(func(ctx *application.Context) {
		slog.Info("tray menu: quit")
		app.Quit()
	})
	tray.SetMenu(trayMenu)

	if err := app.Run(); err != nil {
		log.Fatal(err)
	}
}
