NINJA_TOOL_DIR=${NINJA_TOOL_DIR-${0:A:h}}

function expand_tab() {
    if [[ "${CURSOR}" = "${#BUFFER}" ]] || [[ "${RBUFFER[1]}" = " " ]]; then
        zle expand-or-complete
    else
        BUFFER="${LBUFFER} ${RBUFFER}"
    fi
}
zle -N expand_tab

function copy_cwd() {
    echo "${PWD}" | tr -d "\n" | pbcopy
}
zle -N copy_cwd

bindkey "^i" expand_tab
bindkey "^[d" copy_cwd
