#!/bin/bash
set -euo pipefail

# Script to update AUR package
# Usage: update-aur.sh <package-name> <pkgbuild-source> <version>
# Requires: AUR_SSH_PRIVATE_KEY environment variable

PACKAGE_NAME="$1"
PKGBUILD_SOURCE="$2"
VERSION="$3"

if [ -z "$PACKAGE_NAME" ] || [ -z "$PKGBUILD_SOURCE" ] || [ -z "$VERSION" ]; then
  echo "Usage: $0 <package-name> <pkgbuild-source> <version>" >&2
  exit 1
fi

if [ -z "$AUR_SSH_PRIVATE_KEY" ]; then
  echo "Error: AUR_SSH_PRIVATE_KEY environment variable not set" >&2
  exit 1
fi

echo "Updating AUR package: $PACKAGE_NAME to version $VERSION" >&2

# Setup SSH for AUR
echo "Setting up SSH for AUR..." >&2
mkdir -p ~/.ssh
echo "$AUR_SSH_PRIVATE_KEY" > ~/.ssh/aur
chmod 600 ~/.ssh/aur
ssh-keyscan aur.archlinux.org >> ~/.ssh/known_hosts 2>/dev/null
cat >> ~/.ssh/config <<EOF
Host aur.archlinux.org
  IdentityFile ~/.ssh/aur
  User aur
EOF
chmod 600 ~/.ssh/config

# Clone with explicit SSH command
GIT_SSH_COMMAND="ssh -i ~/.ssh/aur -o IdentitiesOnly=yes" git clone "ssh://aur@aur.archlinux.org/${PACKAGE_NAME}.git" aur-repo
cd aur-repo

# Copy updated PKGBUILD
cp "../${PKGBUILD_SOURCE}" PKGBUILD

if [ ! -f PKGBUILD ]; then
  echo "Error: Failed to copy PKGBUILD from ${PKGBUILD_SOURCE}" >&2
  exit 1
fi

echo "Generating .SRCINFO..." >&2

# Generate .SRCINFO using Arch Linux container
docker run --rm \
  -v "$PWD:/pkg" -w /pkg \
  -e HOST_UID=$(id -u) -e HOST_GID=$(id -g) \
  archlinux:latest bash -lc '
    set -e
    # Prepare pacman keyring to avoid pacman-key errors (redirect to stderr)
    pacman-key --init >&2 || true
    pacman-key --populate archlinux >&2 || true

    pacman -Syu --noconfirm base-devel >&2

    # Create group/user with host UID/GID so files remain owned by the runner
    groupadd -g $HOST_GID builder >&2 || true
    useradd -m -u $HOST_UID -g $HOST_GID builder >&2 || true

    # Ensure /pkg is owned by the host UID/GID inside the container
    chown -R $HOST_UID:$HOST_GID /pkg >&2

    # Run makepkg as the non-root builder user and print srcinfo
    su - builder -c "cd /pkg && makepkg --printsrcinfo"
  ' > .SRCINFO

# Configure git
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

# Commit and push to AUR
git add PKGBUILD .SRCINFO
git diff --cached --quiet || git commit -m "Update to v${VERSION}"
GIT_SSH_COMMAND="ssh -i ~/.ssh/aur -o IdentitiesOnly=yes" git push

echo "✓ Successfully updated $PACKAGE_NAME to v$VERSION"
