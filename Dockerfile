FROM rust AS builder

WORKDIR /src
COPY . .
RUN chown -R 1000:1000 /src

RUN cargo build --release

FROM archlinux

RUN pacman -Syu --noconfirm \
      texlive-basic \
      texlive-latexrecommended \
      texlive-latexextra \
      texlive-fontsextra

RUN pacman -S --noconfirm inotify-tools

COPY --from=builder /src/target/release/curriculust /usr/bin/curriculust
COPY --from=builder /src/build.sh /usr/bin/curriculust-watch
RUN sed -i 's#cargo run --#/usr/bin/curriculust#' /usr/bin/curriculust-watch
RUN sed -i 's#"./src/" "./resume_cv_proc_macro"##' /usr/bin/curriculust-watch

WORKDIR /src
RUN useradd -u 1000 user -M -d /src
RUN chown -R 1000:1000 /src
USER 1000:1000
ENTRYPOINT ["/usr/bin/curriculust-watch"]
