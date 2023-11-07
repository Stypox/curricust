#!/bin/sh

while true; do
    if [ "$#" = "0" ]; then
        break;

    elif [ "$1" = "--" ]; then
        # the next arguments will be passed directly to the executable
        shift;
        break;

    elif [ "$1" = "--help" ]; then
        echo "Wrapper around resume-cv-rust for automatic conversion from LaTeX to PDF."
        echo "Usage: $0 [--help] [--dark] [--watch] <BASE_YML_PATH> <OUTPUT_TEX_PATH> <TEMPLATE_PATH>"
        echo "Pass arguments to resume-cv-rust after --, e.g. $0 ... -- --check-links"
        echo
        echo "Arguments:"
        echo "  <BASE_YML_PATH>   The base YAML file to use as input"
        echo "  <OUTPUT_TEX_PATH> The output LaTeX file to then convert to PDF"
        echo "                    (the folder it is in will be cluttered with build files!)"
        echo "  <TEMPLATE_PATH>   The .cls template to use"
        echo
        echo "Options:"
        echo "  --help  Print help"
        echo "  --dark  Invert colors in the PDF"
        echo "  --watch Automatically rebuild the .tex and the PDF when any relevant file changes"
        exit 0

    elif [ "$1" = "--dark" ]; then
        DARK="true"

    elif [ "$1" = "--watch" ]; then
        WATCH="true"

    elif [ "$BASE_YML_PATH" = "" ]; then
        BASE_YML_PATH="$1"

    elif [ "$OUTPUT_TEX_PATH" = "" ]; then
        OUTPUT_TEX_PATH="$1"

    elif [ "$TEMPLATE_PATH" = "" ]; then
        TEMPLATE_PATH="$1"

    else
        echo "Unexpected argument: $1"
        echo "Use this to get help about the build script: $0 --help"
        echo "Use this to get help about resume-cv-rust: $0 -- --help"
        exit 1
    fi

    shift;
done;

function buildPdf() {
    cargo run -- "$BASE_YML_PATH" "$OUTPUT_TEX_PATH" "$@" || return
    if [ "$DARK" = "true" ]; then
        sed -i -e "s/{resumecvrusttemplate}/{resumecvrusttemplate}\\\\pagecolor[rgb]{0,0,0}\\\\color[rgb]{1,1,1}/" "$OUTPUT_TEX_PATH"
    fi

    NEW_CLS_PATH="$(dirname "$OUTPUT_TEX_PATH")/resumecvrusttemplate.cls"
    cp "$TEMPLATE_PATH" "$NEW_CLS_PATH"
    sed -i -e "s/ProvidesClass{[^}]*}/ProvidesClass{resumecvrusttemplate}/" "$NEW_CLS_PATH"

    pushd "$(dirname "$OUTPUT_TEX_PATH")" >/dev/null
    pdflatex \
        --interaction=nonstopmode "$(basename "$OUTPUT_TEX_PATH")" \
        | awk 'BEGIN{IGNORECASE = 1}/warning|!/,/^$/;' \
        || echo "Could not compile latex"
    popd >/dev/null
}

buildPdf "$@"

if [ "$WATCH" = "true" ]; then
    inotifywait --recursive --monitor \
        --event modify,move,create,delete \
        "$(dirname "$BASE_YML_PATH")" "./src/" "./resume_cv_proc_macro" "$TEMPLATE_PATH" \
        | while read whatchanged; do
            buildPdf "$@"
        done
fi