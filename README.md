# Bukara

Bukara Recorder Service

Bukara is a systemd-enabled D-Bus service designed to simplify audio recording on Linux using PipeWire. It integrates with the D-Bus message bus and allows remote control of audio recordings via record and stop commands. Recordings are saved in the Downloads directory with filenames based on the current timestamp.

## Motivation

I'm exploring audio production and enjoy working with samples. This tool lets me quickly capture interesting sound snippets from YouTube videos, movies, Netflix shows, and more. Simply press Ctrl+Shift+R to start recording and Ctrl+Shift+T to stop. The recording is automatically saved in $HOME/Downloads, ready for you to edit later in Audacity or your preferred sound-editing software. After refining, you can add it to your sample library.

## Dependencies

This project relies on modern Linux desktop environment:

* PipeWire (pw-record)
* systemd for service management

And naturally, Rust with Cargo for building the project.

## Installation

To install Bukara as a systemd user service, use the provided installation script:

> ./install.sh install

This script will:

    Build the Rust project in release mode.
    Place the compiled binary in ~/bin.
    Create a systemd service file and enable it.
    Start the service immediately.

Now manually set keyboard shortcuts, here is how to do it in Gnome:

In Gnome Settings, Keyboard

-> View and Customize Shortcuts

-> Custom Shortcuts

-> +

-> Start Audio Recording
command: ```busctl --user call com.github.peerchemist.Bukara1 /com/github/peerchemist/Bukara1 com.github.peerchemist.Bukara1 Record```

-> + Stop Audio Recording
command: ```busctl --user call com.github.peerchemist.Bukara1 /com/github/peerchemist/Bukara1 com.github.peerchemist.Bukara1 Stop```

## Uninstallation

To remove Bukara, run:

./install.sh uninstall