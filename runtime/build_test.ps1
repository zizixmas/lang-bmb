# BMB Bootstrap Build Test Script
# Compiles LLVM IR and links with runtime to create executable
# Run from Developer PowerShell for VS 2022

param(
    [switch]$Clean,
    [switch]$Run
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location $ScriptDir

Write-Host "=== BMB Bootstrap Build Test ===" -ForegroundColor Cyan
Write-Host ""

# Clean up
if ($Clean) {
    Write-Host "Cleaning..." -ForegroundColor Yellow
    Remove-Item -Force *.obj, *.exe 2>$null
}

# Find clang
$Clang = "C:\Program Files\LLVM\bin\clang.exe"
if (-not (Test-Path $Clang)) {
    $Clang = (Get-Command clang -ErrorAction SilentlyContinue).Source
}
if (-not $Clang) {
    Write-Host "ERROR: clang not found" -ForegroundColor Red
    exit 1
}
Write-Host "Using clang: $Clang" -ForegroundColor Gray

# Check for cl.exe (needed for linking on Windows)
$HasCL = $null -ne (Get-Command cl -ErrorAction SilentlyContinue)
if (-not $HasCL) {
    Write-Host "WARNING: cl.exe not found. Run from Developer PowerShell for full build." -ForegroundColor Yellow
    Write-Host ""
}

# Step 1: Compile LLVM IR to object
Write-Host "[1/3] Compiling LLVM IR..." -ForegroundColor Green
& $Clang -c test_add.ll -o test_add.obj 2>&1 | Where-Object { $_ -notmatch "warning:" }
if (-not (Test-Path test_add.obj)) {
    Write-Host "  ERROR: Failed to compile LLVM IR" -ForegroundColor Red
    exit 1
}
Write-Host "  Created test_add.obj" -ForegroundColor Gray

# Step 2: Compile runtime
Write-Host "[2/3] Compiling runtime..." -ForegroundColor Green
if ($HasCL) {
    & cl /c /nologo runtime.c /Foruntime.obj 2>&1 | Out-Null
} else {
    # Try clang-cl with Windows SDK paths if available
    $ClangCL = "C:\Program Files\LLVM\bin\clang-cl.exe"
    if (Test-Path $ClangCL) {
        & $ClangCL /c runtime.c /Foruntime.obj 2>&1 | Out-Null
    } else {
        Write-Host "  SKIPPED: No C compiler available" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "To complete the build, run from Developer PowerShell:" -ForegroundColor Cyan
        Write-Host "  & '$ScriptDir\build_test.ps1'" -ForegroundColor White
        Pop-Location
        exit 0
    }
}
if (-not (Test-Path runtime.obj)) {
    Write-Host "  ERROR: Failed to compile runtime" -ForegroundColor Red
    exit 1
}
Write-Host "  Created runtime.obj" -ForegroundColor Gray

# Step 3: Link
Write-Host "[3/3] Linking..." -ForegroundColor Green
if ($HasCL) {
    & cl /nologo test_add.obj runtime.obj /Fe:test_add.exe 2>&1 | Out-Null
} else {
    & $Clang test_add.obj runtime.obj -o test_add.exe 2>&1 | Out-Null
}
if (-not (Test-Path test_add.exe)) {
    Write-Host "  ERROR: Failed to link" -ForegroundColor Red
    exit 1
}
$Size = (Get-Item test_add.exe).Length
Write-Host "  Created test_add.exe ($Size bytes)" -ForegroundColor Gray

Write-Host ""
Write-Host "=== Build successful ===" -ForegroundColor Green

# Run if requested
if ($Run) {
    Write-Host ""
    Write-Host "Running test_add.exe..." -ForegroundColor Cyan
    Write-Host "---"
    & .\test_add.exe
    $ExitCode = $LASTEXITCODE
    Write-Host "---"
    Write-Host "Exit code: $ExitCode" -ForegroundColor Gray
}

Pop-Location
