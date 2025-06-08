$exeUrl = "https://github.com/abhix2112/Devgeini/releases/latest/download/devgeini.exe"  # Replace with your actual release link
$installDir = "$env:ProgramData\devgeini"

# Create install directory
if (!(Test-Path -Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Download the exe
Write-Host "Downloading devgeini.exe..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $exeUrl -OutFile "$installDir\devgeini.exe"

# Add to system PATH if not already there
$envPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if (-not $envPath.Split(";") -contains $installDir) {
    Write-Host "Adding $installDir to PATH..." -ForegroundColor Yellow
    [System.Environment]::SetEnvironmentVariable("Path", "$envPath;$installDir", [System.EnvironmentVariableTarget]::Machine)
} else {
    Write-Host "$installDir already in PATH" -ForegroundColor Green
}

Write-Host "`n✅ Devgeini is installed globally!" -ForegroundColor Green
Write-Host "➡️  You can now run 'devgeini' from any terminal." -ForegroundColor Cyan
Write-Host "`nYou may need to restart your terminal." -ForegroundColor Yellow
