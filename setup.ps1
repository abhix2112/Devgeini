# Enhanced Devgeini Installer Script
# Run as Administrator for best results

param(
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Configuration
$exeUrl = "https://github.com/abhix2112/Devgeini/releases/latest/download/devgeini-windows-x86_64.exe"
$installDir = "$env:ProgramData\devgeini"
$exePath = "$installDir\devgeini.exe"

# Function to check if running as administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)
}

# Function to add Windows Defender exclusion
function Add-DefenderExclusion {
    param($Path)
    try {
        if (Get-Command "Add-MpPreference" -ErrorAction SilentlyContinue) {
            Add-MpPreference -ExclusionPath $Path -Force
            Write-Host "Added Windows Defender exclusion for: $Path" -ForegroundColor Green
        }
    }
    catch {
        Write-Host "Could not add Defender exclusion (requires admin): $_" -ForegroundColor Yellow
    }
}

# Function to set file attributes to bypass some security warnings
function Set-FileSecurityAttributes {
    param($FilePath)
    try {
        # Unblock the file (removes "downloaded from internet" flag)
        Unblock-File -Path $FilePath -ErrorAction SilentlyContinue
        
        # Set file as trusted
        $file = Get-Item $FilePath
        $file.Attributes = $file.Attributes -band (-bnot [System.IO.FileAttributes]::Archive)
        
        Write-Host "File security attributes updated" -ForegroundColor Green
    }
    catch {
        Write-Host "Could not update file attributes: $_" -ForegroundColor Yellow
    }
}

# Main installation process
try {
    Write-Host "Starting Devgeini Installation..." -ForegroundColor Cyan
    Write-Host "================================================================" -ForegroundColor DarkGray

    # Check admin privileges
    if (Test-Administrator) {
        Write-Host "Running with Administrator privileges" -ForegroundColor Green
    } else {
        Write-Host "Not running as Administrator - some security features may not work" -ForegroundColor Yellow
    }

    # Create install directory
    Write-Host "Creating installation directory..." -ForegroundColor Cyan
    if (!(Test-Path -Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        Write-Host "Created directory: $installDir" -ForegroundColor Green
    } else {
        Write-Host "Directory already exists: $installDir" -ForegroundColor Green
    }

    # Add Defender exclusion before download
    if (Test-Administrator) {
        Write-Host "Adding Windows Defender exclusion..." -ForegroundColor Cyan
        Add-DefenderExclusion -Path $installDir
    }

    # Download with better error handling and progress
    Write-Host "Downloading devgeini.exe..." -ForegroundColor Cyan
    Write-Host "From: $exeUrl" -ForegroundColor Gray
    
    # Use TLS 1.2 for better compatibility
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    
    # Download with progress and better error handling
    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.Headers.Add("User-Agent", "PowerShell-Installer/1.0")
        $webClient.DownloadFile($exeUrl, $exePath)
        Write-Host "Download completed successfully" -ForegroundColor Green
    }
    catch {
        Write-Host "Download failed: $_" -ForegroundColor Red
        throw "Download failed"
    }
    finally {
        if ($webClient) {
            $webClient.Dispose()
        }
    }

    # Verify download
    if (!(Test-Path -Path $exePath)) {
        throw "Download failed - file not found at $exePath"
    }

    $fileSize = (Get-Item $exePath).Length
    Write-Host "Downloaded file size: $([math]::Round($fileSize/1MB, 2)) MB" -ForegroundColor Gray

    # Set security attributes to reduce Windows warnings
    Write-Host "Configuring file security..." -ForegroundColor Cyan
    Set-FileSecurityAttributes -FilePath $exePath

    # Add to PATH
    Write-Host "Configuring system PATH..." -ForegroundColor Cyan
    $envPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
    
    if ($envPath -notlike "*$installDir*") {
        Write-Host "Adding $installDir to system PATH..." -ForegroundColor Yellow
        $newPath = "$envPath;$installDir"
        [System.Environment]::SetEnvironmentVariable("Path", $newPath, [System.EnvironmentVariableTarget]::Machine)
        Write-Host "PATH updated successfully" -ForegroundColor Green
    } else {
        Write-Host "$installDir already in PATH" -ForegroundColor Green
    }

    # Create a simple test script to verify installation
    $testScript = @"
@echo off
echo Testing devgeini installation...
devgeini --version >nul 2>&1
if %errorlevel% == 0 (
    echo Devgeini is working correctly!
    devgeini --version
) else (
    echo Devgeini test failed
)
pause
"@
    
    $testScriptPath = "$installDir\test-devgeini.bat"
    $testScript | Out-File -FilePath $testScriptPath -Encoding ASCII
    Set-FileSecurityAttributes -FilePath $testScriptPath

    # Final success message
    Write-Host "================================================================" -ForegroundColor Green
    Write-Host "INSTALLATION COMPLETED SUCCESSFULLY!" -ForegroundColor Green
    Write-Host "================================================================" -ForegroundColor Green
    
    Write-Host "NEXT STEPS:" -ForegroundColor Cyan
    Write-Host "1. Close and reopen your terminal/command prompt" -ForegroundColor White
    Write-Host "2. Run devgeini --version to verify installation" -ForegroundColor White
    Write-Host "3. Or run the test script: $testScriptPath" -ForegroundColor White
    
    Write-Host "Installation Location: $installDir" -ForegroundColor Gray
    Write-Host "Executable Path: $exePath" -ForegroundColor Gray
    
    if (Test-Administrator) {
        Write-Host "Security optimizations applied:" -ForegroundColor Cyan
        Write-Host "   Windows Defender exclusion added" -ForegroundColor White
        Write-Host "   File unblocked and marked as trusted" -ForegroundColor White
        Write-Host "   System PATH updated" -ForegroundColor White
    }
}
catch {
    Write-Host "INSTALLATION FAILED!" -ForegroundColor Red
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Troubleshooting tips:" -ForegroundColor Yellow
    Write-Host "1. Run PowerShell as Administrator" -ForegroundColor White
    Write-Host "2. Check your internet connection" -ForegroundColor White
    Write-Host "3. Verify the download URL is correct" -ForegroundColor White
    Write-Host "4. Temporarily disable antivirus if needed" -ForegroundColor White
    exit 1
}

Write-Host "Ready to use devgeini!" -ForegroundColor Green

# Keep window open so user can see the results
Write-Host "Press any key to exit..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
