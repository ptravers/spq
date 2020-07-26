if [ "$MARKER" = "durability" ]; then
  py.test -v -m durability
else
  py.test -v -m "not durability"
fi
