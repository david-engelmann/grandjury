# bash completion for grandjury
_grandjury() {
  local cur
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "tally stadium cost agreement compare census ledger dissent swing minority bench pool aggregate canvass roll muster margin accord standing answers pairs shares -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  return 0
}
complete -o bashdefault -o default -F _grandjury grandjury
