New-Item -ItemType Directory -Force -Name logs | Out-Null
cargo build

Get-Content tests/alloc.workload  | cargo run | Tee-Object logs/alloc.log
Get-Content tests/vm.workload     | cargo run | Tee-Object logs/vm.log
Get-Content tests/cache.workload  | cargo run | Tee-Object logs/cache.log

Write-Host "Logs written to /logs"
