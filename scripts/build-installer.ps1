# ValleyFlow Build Script
# Builds the MSI installer with embedded Whisper model

param(
    [switch]$SkipModel,
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "ValleyFlow Build Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check prerequisites
Write-Host "`nChecking prerequisites..." -ForegroundColor Yellow

$rustVersion = rustc --version 2>$null
if (-not $rustVersion) {
    Write-Host "ERROR: Rust is not installed. Please install from https://rustup.rs" -ForegroundColor Red
    exit 1
}
Write-Host "  Rust: $rustVersion" -ForegroundColor Green

$nodeVersion = node --version 2>$null
if (-not $nodeVersion) {
    Write-Host "ERROR: Node.js is not installed. Please install from https://nodejs.org" -ForegroundColor Red
    exit 1
}
Write-Host "  Node: $nodeVersion" -ForegroundColor Green

$pnpmVersion = pnpm --version 2>$null
if (-not $pnpmVersion) {
    Write-Host "ERROR: pnpm is not installed. Run: npm install -g pnpm" -ForegroundColor Red
    exit 1
}
Write-Host "  pnpm: $pnpmVersion" -ForegroundColor Green

# Download model if not skipped
if (-not $SkipModel) {
    $modelPath = "src-tauri\models\ggml-small.bin"
    if (-not (Test-Path $modelPath)) {
        Write-Host "`nDownloading Whisper model..." -ForegroundColor Yellow
        & ".\scripts\download-model.ps1" -Model "small"
    } else {
        Write-Host "`nWhisper model already exists: $modelPath" -ForegroundColor Green
    }
}

# Install dependencies
Write-Host "`nInstalling dependencies..." -ForegroundColor Yellow
pnpm install

# Build frontend
Write-Host "`nBuilding frontend..." -ForegroundColor Yellow
pnpm build

# Build Tauri app
Write-Host "`nBuilding Tauri application..." -ForegroundColor Yellow
if ($Release) {
    pnpm tauri build --release
} else {
    pnpm tauri build
}

# Check output
$msiPath = Get-ChildItem "src-tauri\target\release\bundle\msi\*.msi" -ErrorAction SilentlyContinue | Select-Object -First 1
if ($msiPath) {
    Write-Host "`n========================================" -ForegroundColor Green
    Write-Host "Build successful!" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "MSI Installer: $($msiPath.FullName)" -ForegroundColor Cyan
    Write-Host "Size: $([math]::Round($msiPath.Length / 1MB, 2)) MB" -ForegroundColor Gray
} else {
    Write-Host "`nBuild completed but MSI not found. Check build output above." -ForegroundColor Yellow
}
