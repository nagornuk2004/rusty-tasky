#!/bin/sh

flatpak-builder --repo rusty-tasky-repo --force-clean --ccache flatpak-build-dir io.github.nagornuk2004.rusty-tasky.yml
flatpak build-bundle rusty-tasky-repo rusty-tasky.flatpak io.github.nagornuk2004.rusty-tasky