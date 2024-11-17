#!/bin/bash

SERVICE_NAME=com.github.peerchemist.Bukara1
SERVICE_PATH=~/.config/systemd/user
EXECUTABLE_NAME=bukara-rs
EXECUTABLE_SRC=target/release/$EXECUTABLE_NAME
EXECUTABLE_DEST=$HOME/bin/$EXECUTABLE_NAME

install() {
    echo "Installing $SERVICE_NAME systemd user service..."

    # Build the Rust project
    echo "Building the Rust project..."
    cargo build --release

    # Create $HOME/bin if it doesn't exist
    mkdir -p "$HOME/bin"

    # Move the executable to $HOME/bin
    echo "Moving $EXECUTABLE_SRC to $EXECUTABLE_DEST"
    cp "$EXECUTABLE_SRC" "$EXECUTABLE_DEST"
    chmod +x "$EXECUTABLE_DEST"

    # Create the systemd service file
    mkdir -p "$SERVICE_PATH"

    cat > "$SERVICE_PATH/bukara.service" <<EOL
[Unit]
Description=Bukara Service
After=default.target

[Service]
ExecStart=$EXECUTABLE_DEST
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOL

    echo "Reloading systemd daemon..."
    systemctl --user daemon-reload

    echo "Enabling bukara service..."
    systemctl --user enable bukara

    echo "Starting bukara service..."
    systemctl --user start bukara

    echo "Bukara service installed and started."
}

uninstall() {
    echo "Uninstalling Bukara systemd user service..."

    echo "Stopping bukara service..."
    systemctl --user stop bukara

    echo "Disabling bukara service..."
    systemctl --user disable bukara

    echo "Removing service file: bukara.service..."
    rm -f "$SERVICE_PATH/bukara.service"

    echo "Reloading systemd daemon..."
    systemctl --user daemon-reload

    echo "Removing executable from $EXECUTABLE_DEST"
    rm -f "$EXECUTABLE_DEST"

    echo "Bukara service uninstalled."
}

if [ "$1" == "install" ]; then
    install
elif [ "$1" == "uninstall" ]; then
    uninstall
else
    echo "Usage: $0 [install|uninstall]"
fi
