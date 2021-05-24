# cargo-distribute

WIP


## Installation
```
brew install fuyutarow/tap/cargo-distribute
```


## Usage
In order to distribute packages using homebrew, two repositories are required.
- `username/myproject`, https://github.com/usename/myproject
    - This is the repository that contains the package you want to distribute.
- `username/homebrew-tap`,  https://github.com/usename/homebrew-tap
    - This is the tap repository you need to distribute using homebrew


### Step 1. Create a `usename/homebrew-tap` repository
```
~$ mkdir ~/homebrew-tap
~$ cd $_
~/homebrew-tap$ git init
~/homebrew-tap$ git push
```

### Step 2. Generate a github toke from this link. https://github.com/settings/tokens
![](https://raw.githubusercontent.com/fuyutarow/cargo-distribute/alpha/assets/step2.png)


### Step 3.  Register the value of the token created in Step 2 in the actions sercrets of this link. https://github.com/usename/myproject/settings/secrets/actions
![](https://raw.githubusercontent.com/fuyutarow/cargo-distribute/alpha/assets/step3.png)

### Step 4. Use cargo-ditribute to generate the necessary files

Two files will be generated.
    - `myproject/.github/workflows/release.yml`
    - `homebrew-tap/templates/myproject.rb`

Do a git commit and push in each of the two repositories.  `username/myproject` and `username/homebrew-tap`
```
~$ cd ~/myproject
~/myproject$ echo ./Cargo.toml
~/myproject$ cargo-distribute --tap ~/homebrew-tap
~/myproject$ git -A && git commit -m "commit"
~/myproject$ git push
~$ cd ~/homebrew-tap
~/homebrew-tap$ git -A && git commit -m "commit"
~/homebrew-tap$ git push
```

### Step 5. Add a tag for `username/myproject` and push it

This will trigger the distribution.
```
~$ cd myproject
~/myproject$ git tag vX.Y.Z
~/myproject$ git push --tags
```