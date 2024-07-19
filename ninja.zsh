NINJA_DIR=${NINJA_DIR-${0:A:h}}

function expand_tab() {
    if [[ "${CURSOR}" != "${#BUFFER}" ]] && [[ "${RBUFFER[1]}" != " " ]]; then
        BUFFER="${LBUFFER} ${RBUFFER}"
    fi
    zle expand-or-complete
}
zle -N expand_tab

function copy_cwd() {
    echo "${PWD}" | tr -d "\n" | pbcopy
}
zle -N copy_cwd

function expand_filepath() {
    read _CURSOR BUFFER <<< $(${NINJA_DIR}/rust/expand_path/target/release/expand_path "${BUFFER}" $CURSOR)
    CURSOR=$_CURSOR
    zle redisplay
}
zle -N expand_filepath

bindkey "^i" expand_tab
bindkey "^[d" copy_cwd
bindkey "^[e" expand_filepath
