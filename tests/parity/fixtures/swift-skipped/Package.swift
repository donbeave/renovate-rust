// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "Example",
    dependencies: [
        .package(url: "https://gitlab.com/example/lib.git", from: "1.0.0"),
        .package(url: "https://bitbucket.org/example/other.git", from: "2.0.0"),
    ]
)
