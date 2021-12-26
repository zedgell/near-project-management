FROM gitpod/workspace-full

RUN bash -cl ". ./nvm/nvm.sh \
                && nvm install v12 && nvm alias default v12"
