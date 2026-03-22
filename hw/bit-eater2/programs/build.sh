mkdir -p bin
for file in *.asm ; do
  echo "Building $file"
  nnaasm "$file" -o "bin/${file::-4}.hex"
done
echo "Output files are placed in bin/ directory"
