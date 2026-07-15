cargo build --release
mkdir -p "$HOME/.local/bin"
cp ./target/release/texbuilder "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin"

LINE='export PATH="$PATH:$HOME/.local/bin"'
if grep -Fxq "$LINE" "$HOME/.bashrc"; then
  echo "teXbuilder has been installed. run with 'texbuilder'."
else
  echo "$LINE" >> "$HOME/.bashrc"
  source "$HOME/.bashrc"
  echo "teXbuilder has been installed. run with 'texbuilder'."
fi
