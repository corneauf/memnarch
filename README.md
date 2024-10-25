# Corn's Config Cinstaller

This will create all the necessary setup for a brand new environment. It will

- Install `neovim` if not installed.
- Install `zsh` if not installed.
- Make `zsh` main terminal
- Install `oh-my-zsh`
- Create `.zsh` folder structure
- Create Neovim configuration folder structure
- Install `git` if not installed


## Configuration

- repo to save to: what git repo to push to with latest changes
- keep local archive: if we keep a local copy of the git repo or set it up as temp everytime
- run on start-up: run as `--watcher` on startup
- tools to not download

## Command-line
- --init: create the folder and install tools
    - --from (optional): get repo to install from
    - --overwrite (optional): remove files if they are already present
- --watcher: launch as daemon to watch for changes
- --update: manually update config files and push to repo
