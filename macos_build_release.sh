#!/bin/bash

# Rimuove le vecchie directory di rilascio per entrambe le architetture
rm -rf "release/macos-intel"
rm -rf "release/macos-arm"

# Funzione per creare la directory e copiare gli eseguibili
copy_executables() {
    ARCH=$1

    # Verifica se ARCH è uguale a aarch64-apple-darwin
    if [ "$ARCH" == "aarch64-apple-darwin" ]; then
        RELEASE_DIR="release/macos-arm"
    else
        RELEASE_DIR="release/macos-intel"
    fi

    # Crea la directory se non esiste
    if [ ! -d "$RELEASE_DIR" ]; then
        mkdir -p "$RELEASE_DIR"
    fi

    # Copia gli eseguibili nella directory di rilascio specifica
    if [ -f "target/$ARCH/release/Group-35" ]; then
        cp "target/$ARCH/release/Group-35" "$RELEASE_DIR/Group-35"
    fi

    if [ -f "target/$ARCH/release/main" ]; then
        cp "target/$ARCH/release/main" "$RELEASE_DIR/main"
    fi

    if [ -f "target/$ARCH/release/gui" ]; then
        cp "target/$ARCH/release/gui" "$RELEASE_DIR/gui"
    fi

    if [ -f "target/$ARCH/release/popup_gui" ]; then
          cp "target/$ARCH/release/popup_gui" "$RELEASE_DIR/popup_gui"
    fi

    if [ -f "target/$ARCH/release/uninstall" ]; then
        cp "target/$ARCH/release/uninstall" "$RELEASE_DIR/uninstall"
    fi
}

# Compilazione per architettura Intel (x86_64)
echo "Compilazione per Intel (x86_64)..."
cargo build --release --target x86_64-apple-darwin
copy_executables "x86_64-apple-darwin"

# Compilazione per architettura ARM (Apple Silicon)
echo "Compilazione per ARM (aarch64)..."
cargo build --release --target aarch64-apple-darwin
copy_executables "aarch64-apple-darwin"

echo "Build e processo di copia completato con successo per entrambe le architetture."

