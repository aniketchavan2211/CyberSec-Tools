#####################################
# SCRIPT IS NOT READY FOR EXECUTION #
#####################################

VERSION=v0.1.0

# Linux
cargo build --release 
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# Mac
cargo build --release --target x86_64-apple-darwin

# Android 
cargo build --release --target aarch64-linux-android

# Create dist/bin directory 
mkdir -p dist/nullifer_sword-$(VERSION)//bin

# Copy binary to dist/bin directory
cp /target/release/nullifer_sword dist/nullifer_sword-$(VERSION)/bin/

# Copy README LICENSE and Binary
cp README.md LICENSE dist/nullifer_sword-$(VERSION)/

# Create tar.gz
cd dist
tar -czvf nullifer_sword-$(VERSION)-linux-x86_64.tar.gz nullifer_sword-$(VERSION)

# Create zip
zip -r nullifer_sword-$(VERSION)-linux-x86_64.zip nullifer_sword-$(VERSION)

# OPTIONAL (MAKE EXECUTABLE SAFE)
chmod +x nullifer_sword-$(VERSION)/bin/nullifer_sword-$(VERSION)

# INSTALLATION FROM PACKAGE (USER SIDE)
cd nullifer_sword-$(VERSION)
sudo cp bin/nullifer_sword /usr/local/bin/
