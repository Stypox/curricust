#!/bin/sh

while true; do
    if [ "$1" = "--" ]; then
        shift;
        break;

    elif [ "$1" = "--dark" ]; then
        DARK="true"

    elif [ "$1" = "--watch" ]; then
        WATCH="true"

    elif [ "$TEMPLATE_PATH" = "" ]; then
        TEMPLATE_PATH="$1"
        TEMPLATE_DIRECTORY="$(dirname "$1")"
        TEMPLATE_FILENAME="$(basename "$1")"
        TEMPLATE_NAME="${TEMPLATE_FILENAME%.cls}"
        if [ "$TEMPLATE_NAME" = "" ]; then
            echo "Empty template name: $1"
            exit 3
        elif [ "$TEMPLATE_NAME" = "$TEMPLATE_FILENAME" ]; then
            echo "Template file must end with .cls: $1"
            exit 2
        fi
        echo "|$TEMPLATE_PATH|" "|$TEMPLATE_DIRECTORY|" "|$TEMPLATE_FILENAME|" "|$TEMPLATE_NAME|"

    else
        echo "Unexpected argument: $1"
        exit 1
    fi

    shift;
done;

function buildPdf() {
    cargo run -- "$@" || return
    if [ "$DARK" = "true" ]; then
        sed -i -e "s/{resumecvrusttemplate}/{$TEMPLATE_NAME}\\\\pagecolor[rgb]{0,0,0}\\\\color[rgb]{1,1,1}/" "$2"
    else
        sed -i -e "s/{resumecvrusttemplate}/{$TEMPLATE_NAME}/" "$2"
    fi

    pushd "$(dirname "$2")" >/dev/null
    TEXINPUTS="$TEMPLATE_DIRECTORY/;" pdflatex \
        --interaction=nonstopmode "$(basename "$2")" \
        | awk 'BEGIN{IGNORECASE = 1}/warning|!/,/^$/;' \
        || echo "Could not compile latex"
    popd >/dev/null
}

buildPdf "$@"

if [ "$WATCH" = "true" ]; then
    inotifywait --recursive --monitor \
        --event modify,move,create,delete \
        "$(dirname "$1")" "./src/" "./resume_cv_proc_macro" "$TEMPLATE_PATH" \
        | while read whatchanged; do
            buildPdf "$@"
        done
fi