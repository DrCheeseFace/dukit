# Git Cli Wrapper for Ducks 🦆

For the orange\yellow bill inclined amongst us
- Tha goal is a git cli wrapper that does 95% of what you need to do but SUPER slick.
- gota be able to yank text from the terminal with EASE

![demo](demo.gif)

## Usage
### Status info
`git status` command (default method) but copy pasteable
```sh
> dukit

  S
  src/lib.rs

  M
  src/lib.rs

  U
  .gitignore
  README.md
  duck.gif
```
### Branch info 
Notice how easy copy pasting should be 

```sh
> dukit -b

  Your branch is up to date with 'origin/main'.

  feature/git-switch-interactive
  main

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
```



## Contributing
- If you want to suggest a feature put in an issue with the feature request.
- If you spot a bug (which there probabably are many), put in an issue with how to reproduce it. 
- If you want to contribute code, make a pull request. anything short of a war crime will probably be accepted.

## TODO
- [ ] testing NO CLUE 
- [ ] what is up docs 
- [x] status info
- [x] branch and remote info
- [x] git add using editor
- [ ] git switch branch using editor
- [ ] fuzzy branch switching?
- [ ] ez stash and poppin info 
- [ ] nice git log info
- [ ] ez copy commit hashes 
- [ ] ez rebase
- [ ] ez cherry pick 
- [ ] random other git porcelein shizz 
- [ ] unwrap unwrap unwrap unwrap unwrap unwrap 

![duck duck duck duck](duck.gif)

