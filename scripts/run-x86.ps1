Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Test-Path "dist\os-x86_64-bootimage.bin")) {
    throw "Missing dist\os-x86_64-bootimage.bin. Run .\scripts\build-x86.ps1 first."
}

$tmp = Join-Path $env:TEMP ("rustos-x86-" + [System.Guid]::NewGuid().ToString() + ".bin")
Copy-Item "dist\os-x86_64-bootimage.bin" $tmp -Force
try {
Write-Host "Running QEMU (x86_64)..." -ForegroundColor Cyan
qemu-system-x86_64 `
  -drive format=raw,file="$tmp" `
  -serial stdio `
  -display none `
  -m 256M
} finally {
  Remove-Item $tmp -Force -ErrorAction SilentlyContinue
}


