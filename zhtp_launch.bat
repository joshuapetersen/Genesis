@echo off
set ZHTP_AUTO_WALLET=1
"C:\GENESIS\target\debug\zhtp.exe" node start --dev --config "C:\GENESIS\zhtp\configs\dev-node.toml"
