FROM epitechcontent/epitest-docker:latest

WORKDIR /root

RUN sudo apt-get update
RUN sudo apt-get -y install zsh
RUN sudo apt-get -y install bear
RUN sudo apt-get -y install clangd
RUN sudo apt-get -y remove rustc

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# RUN echo "ZSH_THEME=\"arrow\"" >> ~/.zshrc
# RUN echo "source \$ZSH/oh-my-zsh.sh" >> ~/.zshrc

RUN echo "PROMPT=\"%(?:%{\$fg_bold[green]%}%1{➜%} :%{\$fg_bold[red]%}%1{➜%} ) %{\$fg[yellow]%}%c%{\$reset_color%} \"" >> ~/.oh-my-zsh/themes/robbyrussell.zsh-theme

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN git clone https://github.com/helix-editor/helix
RUN ~/.cargo/bin/cargo install --path helix/helix-term --locked
RUN mkdir -p ~/.config
RUN mkdir -p ~/.config/helix
RUN cp -r helix/runtime ~/.config/helix

RUN echo "PATH=\"\$PATH:/home/nfauveau/.cargo/bin\"" >> ~/.szhrc
RUN echo "export TERMINAL=xfce4-terminal" >> ~/.zshrc
RUN echo "export COLORTERM=truecolor" >> ~/.zshrc
RUN echo "export TERM=xterm-256color" >> ~/.zshrc

RUN cat <<EOF > ~/.config/helix/config.toml
theme = "everforest_dark"

editor.bufferline = "multiple"

# VsCode-like movement

[keys.normal]
C-j = ["move_line_down", "move_line_down", "move_line_down", "move_line_down", "move_line_down"]
C-k = ["move_line_up", "move_line_up", "move_line_up", "move_line_up", "move_line_up"]
C-h = "move_prev_word_start"
C-l = "move_next_word_start"

# Cursor shape between modes

[editor.cursor-shape]
insert = "bar"
normal = "block"
select = "underline"
EOF

WORKDIR /usr/app
