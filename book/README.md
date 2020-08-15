# Docker

     $ docker run -it --rm --user "$(id -u)":"$(id -g)" -e USER=$USER -w /usr/src/myapp -v "$PWD":/usr/src/myapp rust:latest bash