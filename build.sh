#!/bin/sh

function buildPdf() {
    cargo run -- "$@"

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
