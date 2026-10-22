# Cross-reference the GCode string anchors in acclplanner to find the
# functions that emit controller GCode. Reuses the existing snapshot.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "600000"

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$dll = "C:\Program Files\LightBurn\acclplanner.dll"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots\acclplanner.snap"

$anchors = @{
  "setvel"     = "0x18028eea0"   # SET_VELOCITY_LIMIT ACCEL=
  "startgcode" = "0x18028ef70"   # \n;Start GCode\nG90\n
  "startkey"   = "0x18028f078"   # start_gcode
  "bingcode"   = "0x18029c2f0"   # binary_gcode_file:
}
foreach ($k in $anchors.Keys) {
  Write-Output ("[{0}] xrefs {1} {2}" -f (Get-Date -Format HH:mm:ss), $k, $anchors[$k])
  rea xrefs "$dll" $anchors[$k] --snapshot "$snap" --format json `
    2> ("{0}\xref-{1}.err" -f $ev, $k) | Out-File -Encoding utf8 ("{0}\xref-{1}.json" -f $ev, $k)
}
Write-Output "XREFS COMPLETE"
