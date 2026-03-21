Set WshShell = CreateObject("WScript.Shell")
' Silent launch - no console windows visible
WshShell.Run "C:\SarahCore\Sarah_AutoStart.bat", 0, False
Set WshShell = Nothing
