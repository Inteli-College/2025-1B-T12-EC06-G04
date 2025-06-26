Name "14-Bis"
OutFile "14Bis_Installer.exe"
InstallDir "$PROGRAMFILES\14Bis"
RequestExecutionLevel admin

!include "MUI2.nsh"
!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"

Section "Install"
    SetOutPath "$INSTDIR"
    File "target/release/Group_14_bis.exe"  ; Certifique-se de que o nome do arquivo está correto
    CreateShortcut "$DESKTOP\14-Bis.lnk" "$INSTDIR\Group_14_bis.exe"
    CreateShortcut "$DESKTOP\Desinstalar 14-Bis.lnk" "$INSTDIR\uninstall.exe"
    WriteUninstaller "$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\14Bis" "DisplayName" "14-Bis"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\14Bis" "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\14Bis" "Install_Dir" "$INSTDIR"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\Group_14_bis.exe"
    Delete "$INSTDIR\uninstall.exe"
    Delete "$DESKTOP\14-Bis.lnk"
    Delete "$DESKTOP\Desinstalar 14-Bis.lnk"
    RMDir "$INSTDIR"
    DeleteRegKey HKLM "Software\14Bis"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\14Bis"
SectionEnd
