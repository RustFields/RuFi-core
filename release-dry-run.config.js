const config = require('semantic-release-preconfigured-conventional-commits')
const verifyReleaseCommands = `
sed -i 's/version.*/version = "\${nextRelease.version}"/g' Cargo.toml || exit 1
git add -A || exit 2
git commit -m "chore: [skip ci] update version in Cargo.toml" || exit 3
cargo publish --dry-run || exit 4
`
const releaseBranches = ["*"]
config.branches = releaseBranches
config.plugins.push(
    ["@semantic-release/exec", {
        "verifyReleaseCmd": verifyReleaseCommands,
    }],
)
module.exports = config