param(
    [ValidateSet('Doctor', 'Check', 'Build', 'Inspect', 'Catalog', 'Smoke')][string]$Action = 'Check',
    [string]$Toolchain = '',
    [string]$Iso = '',
    [ValidateRange(1, 120)][int]$Seconds = 20,
    [ValidateRange(1, 16)][int]$Jobs = 1,
    [ValidateRange(0, 2)][int]$DebugInfo = 0
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if (-not $Iso) { $Iso = Join-Path $root 'God Hand (USA).iso' }
if (-not $Toolchain -and $env:OS -eq 'Windows_NT') {
    $installed = & rustup toolchain list
    if ($LASTEXITCODE -ne 0) { throw 'rustup toolchain list failed' }
    if ($installed -match '^stable-x86_64-pc-windows-gnu(?:\s|$)') {
        $Toolchain = 'stable-x86_64-pc-windows-gnu'
    }
}
function Run-Tool([string]$program, [string[]]$arguments) {
    & $program @arguments
    if ($LASTEXITCODE -ne 0) { throw "$program failed (exit $LASTEXITCODE)" }
}
function Run-Cargo([string[]]$arguments) {
    $prefix = @()
    if ($Toolchain) { $prefix = @("+$Toolchain") }
    Run-Tool cargo ($prefix + $arguments)
}
Push-Location -LiteralPath $root
$oldDebug = $env:CARGO_PROFILE_DEV_DEBUG
$env:CARGO_PROFILE_DEV_DEBUG = "$DebugInfo"
try {
    switch ($Action) {
        'Doctor' {
            $rustArgs = @('--version')
            if ($Toolchain) { $rustArgs = @("+$Toolchain") + $rustArgs }
            Run-Tool rustc $rustArgs
            Run-Cargo @('--version')
            Write-Output "Selected Cargo toolchain: $(if($Toolchain){$Toolchain}else{'default'})"
            Run-Tool python @('scripts/reference_inventory.py', '--output', "$root/docs/local/reference-doctor.json")
        }
        'Check' {
            Run-Cargo @('fmt', '--all', '--', '--check')
            Run-Cargo @('test', '--workspace', '--locked', '--jobs', "$Jobs")
            Run-Cargo @('check', '-p', 'godhand-lab', '--features', 'viewer', '--locked', '--jobs', "$Jobs")
            Run-Tool python @('scripts/dashboard.py', '--check')
            Run-Tool python @('-m', 'unittest', 'discover', '-s', 'scripts', '-p', 'test_*.py')
        }
        'Build' {
            Run-Cargo @('build', '-p', 'godhand-lab', '--features', 'viewer', '--locked', '--jobs', "$Jobs")
        }
        'Inspect' {
            Run-Cargo @('run', '-p', 'godhand-lab', '--locked', '--', '--iso', $Iso)
        }
        'Catalog' {
            Run-Cargo @('run', '-p', 'godhand-lab', '--locked', '--', '--iso', $Iso, '--catalog')
        }
        'Smoke' {
            Run-Cargo @('run', '-p', 'godhand-lab', '--features', 'viewer', '--locked', '--jobs', "$Jobs", '--', '--iso', $Iso, '--sandbox', '--smoke-seconds', "$Seconds")
        }
    }
} finally { $env:CARGO_PROFILE_DEV_DEBUG = $oldDebug; Pop-Location }
