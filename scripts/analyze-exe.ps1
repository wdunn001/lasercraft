# Full Ghidra analysis of LightBurn.exe. The binary is 33.6 MB, so auto-analysis
# is slow. The startup deadline is raised to 60 minutes. This holds the Ruida and
# GRBL controller dialects and the job and layer data model.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "3600000"   # 60 min

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots\lightburn.snap"
$exe = "C:\Program Files\LightBurn\LightBurn.exe"

Write-Output ("[{0}] analyze LightBurn.exe (long run)" -f (Get-Date -Format HH:mm:ss))
rea analyze "$exe" --provider ghidra --snapshot "$snap" --format json `
  2> "$ev\analyze-lightburn.err" | Out-File -Encoding utf8 "$ev\analyze-lightburn.json"
$head = (Get-Content "$ev\analyze-lightburn.json" -TotalCount 1 -ErrorAction SilentlyContinue)
Write-Output ("[{0}] done: {1} bytes, error={2}" -f (Get-Date -Format HH:mm:ss), (Get-Item "$ev\analyze-lightburn.json").Length, ($head -match '"error"'))
Write-Output "EXE ANALYSIS COMPLETE"
