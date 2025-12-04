// swift-tools-version: 5.9
// LEX-Ω Browser Package Configuration
//
// [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

import PackageDescription

let package = Package(
    name: "LEXOmegaBrowser",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .executable(name: "LEXOmegaBrowser", targets: ["LEXOmegaBrowser"]),
        .library(name: "InvarianceCore", targets: ["InvarianceCore"]),
        .library(name: "SSMRuntime", targets: ["SSMRuntime"]),
    ],
    dependencies: [
        // CryptoKit is included in macOS SDK
    ],
    targets: [
        .executableTarget(
            name: "LEXOmegaBrowser",
            dependencies: ["InvarianceCore", "SSMRuntime"],
            path: "Sources/LEXOmegaBrowser"
        ),
        .target(
            name: "InvarianceCore",
            dependencies: [],
            path: "Sources/InvarianceCore"
        ),
        .target(
            name: "SSMRuntime",
            dependencies: ["InvarianceCore"],
            path: "Sources/SSMRuntime"
        ),
        .testTarget(
            name: "LEXOmegaBrowserTests",
            dependencies: ["LEXOmegaBrowser", "InvarianceCore"],
            path: "Tests/LEXOmegaBrowserTests"
        ),
    ]
)

