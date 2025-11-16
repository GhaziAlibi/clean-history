# Maintainer: ALIBI Ghazi <123127137+GhaziAlibi@users.noreply.github.com>
pkgname=clean-history
pkgver=1.0
pkgrel=1
pkgdesc="Clean multiline entries from shell history files"
arch=('x86_64' 'i686' 'aarch64' 'armv7h')
url="https://github.com/GhaziAlibi/clean-history"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/GhaziAlibi/clean-history/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Will be updated after release

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
