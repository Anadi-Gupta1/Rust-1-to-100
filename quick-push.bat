@echo off
REM Auto-commit and push script for Rust learning project (Windows)

echo Adding all changes to Git...
git add .

set /p commit_message="Enter commit message (or press Enter for default): "

if "%commit_message%"=="" (
    for /f "delims=" %%i in ('powershell -command "Get-Date -Format 'yyyy-MM-dd HH:mm:ss'"') do set datetime=%%i
    set commit_message=Update: !datetime!
)

echo Committing changes...
git commit -m "%commit_message%"

echo Pushing to GitHub...
git push origin main

echo ✅ Changes successfully pushed to GitHub!
pause
