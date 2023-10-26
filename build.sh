#!/bin/sh

if [ "$1" = "--dark" ]; then
    DARK="true";
    shift;
fi

function buildPdf() {
    cargo run -- "$@" || return
    if [ "$DARK" = "true" ]; then
        sed -i -e "s/{developercv}/{developercv}\\\\pagecolor[rgb]{0,0,0}\\\\color[rgb]{1,1,1}/" "$2"
    fi

    pushd "$(dirname "$2")" >/dev/null
    pdflatex --interaction=nonstopmode "$(basename "$2")" \
        | awk 'BEGIN{IGNORECASE = 1}/warning|!/,/^$/;' \
        || echo "Could not compile latex"
    popd >/dev/null
}

buildPdf "$@"

inotifywait --recursive --monitor \
    --event modify,move,create,delete \
    "$(dirname "$1")" "./src/" "./resume_cv_proc_macro" \
    | while read whatchanged; do
        buildPdf "$@"
    done
