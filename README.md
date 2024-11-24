# Git Cli Wrapper for Ducks 🦆

For the orange\yellow bill inclined amongst us
- Tha goal is a git cli wrapper that does 95% of what you need to do but SUPER slick.
- gota be able to yank text from the terminal with EASE

![demo](demo.gif)

## Usage
### Status info
`git status` command (default method) but copy pasteable
```sh
> dukit -b

 Staged
 src/lib.rs

 Modified
 README.md
 src/base_commands.rs

 Untracked
 hi.txt
```
### Branch info 
Notice how easy copy pasting should be 

```sh
> dukit -b

  Your branch is up to date with 'origin/main'.

  feature/git-switch-interactive
  main

```

### Fuzzy branch switching 
```sh
> dukit -f

 #opens fzf
    test2
    test1
▌   feature/git-switch-interactive
  3/3 ─────────────────────────────────────────────────────────────
> {search bar here}

 Switched to branch 'feature/git-switch-interactive'
```

### Interactive `git add` 
This opens your default editor with files to add

```sh
> dukit -i

# Staged
 out.tape

# -------------------------------------

# Unstaged
[ ] README.md
[ ] .gitignore
[x] demo.gif

# Selected files to be staged like so below V
# [x] file.txt
# Lines begining with (#) will be ignored

> running git add demo.gif
  demo.gif staged

```
## Installation
```
cargo install dukit
apt install fzf
```



## Contributing
- If you want to suggest a feature put in an issue with the feature request.
- If you spot a bug (which there probabably are many), put in an issue with how to reproduce it. 
- If you want to contribute code, make a pull request. anything short of a war crime will probably be accepted.

## TODO
- [x] status info
- [x] branch and remote info
- [x] git add using editor
- [x] unwrap unwrap unwrap unwrap unwrap unwrap 
- [x] fuzzy branch switching?
- [x] nice git log info
- [x] ez copy commit hashes 
- [ ] ez rebase
- [ ] ez cherry pick 
- [ ] random other git porcelein shizz 

![duck duck duck duck](duck.gif)
