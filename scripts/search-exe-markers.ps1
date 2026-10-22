# Targeted regex search of LightBurn.exe strings for controller dialects
# and the .lbrn file model. Reuses the existing snapshot.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "3600000"

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$exe = "C:\Program Files\LightBurn\LightBurn.exe"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots\lightburn.snap"
$pat = "Ruida|RDWorks|RDC|GRBL|Smoothie|Marlin|Trocen|TopWisdom|EzCad|Galvo|\.lbrn|lbrn2|GCode"

Write-Output ("[{0}] search LightBurn.exe markers" -f (Get-Date -Format HH:mm:ss))
rea search "$exe" --pattern $pat --mode regex --kind strings --snapshot "$snap" --format json `
  2> "$ev\search-exe-markers.err" | Out-File -Encoding utf8 "$ev\search-exe-markers.json"
Write-Output ("[{0}] done: {1} bytes" -f (Get-Date -Format HH:mm:ss), (Get-Item "$ev\search-exe-markers.json" -ErrorAction SilentlyContinue).Length)
Write-Output "SEARCH COMPLETE"
