#! /bin/bash

alias attdev='docker exec -it mosquitto-dev-1 bash'
alias dcud='docker compose up -d'
alias dcd='docker compose down'

alias gits='git status'
alias gitau="git add -u"
alias gitc='git commit'
alias gitca='git commit --amend'
alias gitcan='git commit --amend --no-edit'

ips() {
    docker inspect -f '{{.Name}} - {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $(docker ps -q)
}