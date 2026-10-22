# Analyze the high-value LightBurn DLLs with REA + Ghidra, sequentially.
# Each maps to a LaserCraft crate. Small binaries finish quickly.
$ErrorActionPreference = "Continue"
$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH
$env:GHIDRA_INSTALL_DIR = "C:\Users\willi\tools\ghidra\ghidra_12.1.4_PUBLIC"
$jdk = (Get-ChildItem "C:\Program Files\Microsoft\jdk*" -Directory | Select-Object -First 1).FullName
$env:JAVA_HOME = $jdk; $env:PATH = "$jdk\bin;" + $env:PATH
$env:REA_GHIDRA_STARTUP_TIMEOUT_MS = "1800000"   # 30 min ceiling per DLL

$ev = "C:\Users\willi\source\lasercraft\rea-evidence"
$snap = "C:\Users\willi\source\lasercraft\rea-snapshots"
$lb = "C:\Program Files\LightBurn"

$targets = @(
  @{ name = "transfer";    file = "transfer.dll" },
  @{ name = "executor";    file = "executor.dll" },
  @{ name = "acclplanner"; file = "acclplanner.dll" }
)

foreach ($t in $targets) {
  $src = Join-Path $lb $t.file
  Write-Output ("[{0}] analyze {1}" -f (Get-Date -Format HH:mm:ss), $t.file)
  rea analyze "$src" --provider ghidra --snapshot ("{0}\{1}.snap" -f $snap, $t.name) --format json `
    2> ("{0}\analyze-{1}.err" -f $ev, $t.name) | Out-File -Encoding utf8 ("{0}\analyze-{1}.json" -f $ev, $t.name)
  $jsonPath = "{0}\analyze-{1}.json" -f $ev, $t.name
  $bytes = (Get-Item $jsonPath -ErrorAction SilentlyContinue).Length
  $head = (Get-Content $jsonPath -TotalCount 1 -ErrorAction SilentlyContinue)
  $isErr = $head -match '"error"'
  Write-Output ("[{0}] {1} done: {2} bytes, error={3}" -f (Get-Date -Format HH:mm:ss), $t.name, $bytes, $isErr)
}
Write-Output "ALL DLL ANALYSES COMPLETE"
