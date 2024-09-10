@echo off

:: Rimuove la directory release\windows se esiste
if exist release\windows (
    rmdir /S /Q release\windows
)

:: Compila il progetto con Cargo in modalità release
cargo build --release --workspace

:: Crea la directory release\windows se non esiste
if not exist release\windows (
    mkdir release\windows
)

:: Copia l'eseguibile nella cartella release\windows
if exist target\release\Group-35.exe (
    copy target\release\Group-35.exe release\windows\Group-35.exe
)

if exist target\release\uninstall.exe (
    copy target\release\uninstall.exe release\windows\uninstall.exe
)


echo Build and copy process completed successfully.
