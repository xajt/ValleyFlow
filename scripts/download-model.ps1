# Download Whisper model script
# Run this before building the installer

param(
    [string]$Model = "small",
    [string]$OutputDir = "src-tauri\models"
)

$ModelUrl = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-$Model.bin"
$OutputPath = Join-Path $OutputDir "ggml-$Model.bin"

Write-Host "Downloading Whisper model: $Model" -ForegroundColor Cyan
Write-Host "URL: $ModelUrl" -ForegroundColor Gray
Write-Host "Output: $OutputPath" -ForegroundColor Gray

# Create output directory if it doesn't exist
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Download the model
try {
    Invoke-WebRequest -Uri $ModelUrl -OutFile $OutputPath -UseBasicParsing
    Write-Host "Model downloaded successfully!" -ForegroundColor Green
    Write-Host "Size: $((Get-Item $OutputPath).Length / 1MB) MB" -ForegroundColor Gray
} catch {
    Write-Host "Failed to download model: $_" -ForegroundColor Red
    exit 1
}
