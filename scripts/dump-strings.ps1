# Dump the full analyzed string inventory from each DLL snapshot.
# Snapshots must already exist (produced by analyze-dlls.ps1).
# Strings are the primary protocol-identity signal because the PDBs are stripped.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "600000"

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots"
$lb = "C:\Program Files\LightBurn"

$targets = @("transfer","executor","acclplanner")
foreach ($name in $targets) {
  $dll = Join-Path $lb ("{0}.dll" -f $name)
  $snapPath = "{0}\{1}.snap" -f $snap, $name
  if (-not (Test-Path $snapPath)) { Write-Output ("skip {0}: no snapshot" -f $name); continue }
  Write-Output ("[{0}] dump strings {1}" -f (Get-Date -Format HH:mm:ss), $name)
  rea search "$dll" --pattern ".+" --mode regex --kind strings --snapshot "$snapPath" --format json `
    2> ("{0}\strings-{1}.err" -f $ev, $name) | Out-File -Encoding utf8 ("{0}\strings-{1}.json" -f $ev, $name)
  $bytes = (Get-Item ("{0}\strings-{1}.json" -f $ev, $name) -ErrorAction SilentlyContinue).Length
  Write-Output ("[{0}] {1} strings dumped: {2} bytes" -f (Get-Date -Format HH:mm:ss), $name, $bytes)
}
Write-Output "STRING DUMPS COMPLETE"
