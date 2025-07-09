cargo build -r --workspace 

mkdir %~dp0target\bundle\

xcopy %~dp0target\release\sandoitchi_bridge.exe %~dp0target\bundle\sandoitchi_bridge.exe /Y
xcopy %~dp0target\release\sandoitchi_bridge_ui.exe %~dp0target\bundle\sandoitchi_bridge_ui.exe /Y
xcopy %~dp0firewall.bat %~dp0target\bundle\firewall.bat /Y
xcopy %~dp0README.md %~dp0target\bundle\README.md /Y