install:
  cargo build --bin ifocob --release
  cargo build --bin ifconvert --release
  sudo cp target/release/ifocob /usr/bin/
  sudo cp target/release/ifconvert /usr/bin/
  cp ifocob.desktop $HOME/.local/share/applications/
  cp ifconvert.desktop $HOME/.local/share/applications/
remove:
  rm $HOME/.local/share/applications/ifocob.desktop
  rm $HOME/.local/share/applications/ifconvert.desktop
  sudo rm /usr/bin/ifocob
  sudo rm /usr/bin/ifconvert