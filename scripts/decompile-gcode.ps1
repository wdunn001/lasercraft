# Decompile the acclplanner functions that emit GCode, found via xref.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "600000"

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$dll = "C:\Program Files\LightBurn\acclplanner.dll"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots\acclplanner.snap"

$funcs = @{
  "velocity-limit" = "0x180037456"   # emits SET_VELOCITY_LIMIT
  "start-gcode"    = "0x180034e85"   # builds the Start GCode header
  "binary-gcode"   = "0x180060f80"   # binary gcode writer
}
foreach ($k in $funcs.Keys) {
  Write-Output ("[{0}] decompile {1} {2}" -f (Get-Date -Format HH:mm:ss), $k, $funcs[$k])
  rea decompile "$dll" $funcs[$k] --snapshot "$snap" --format md `
    2> ("{0}\decompile-{1}.err" -f $ev, $k) | Out-File -Encoding utf8 ("{0}\decompile-{1}.md" -f $ev, $k)
  $bytes = (Get-Item ("{0}\decompile-{1}.md" -f $ev, $k) -ErrorAction SilentlyContinue).Length
  Write-Output ("[{0}] {1} done: {2} bytes" -f (Get-Date -Format HH:mm:ss), $k, $bytes)
}
Write-Output "DECOMPILE COMPLETE"
