
if ! type nnaasm > /dev/null ; then
  echo "nnaasm is not found. Install it by running ./tools/install.sh" 1>&2
  exit 1
fi


mkdir -p bin
for file in *.asm ; do
  echo "Building $file"
  nnaasm "$file" -o "bin/${file::-4}.hex"
done
echo "Output files are placed in bin/ directory"
