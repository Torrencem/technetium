
rm -rf ./docs/

cd docsource
make html

cd ..
cp -r docsource/_build/html ./docs/

touch ./docs/.nojekyll

cargo doc --no-deps --workspace

mkdir ./docs/crates

cp -r ./target/doc/* ./docs/crates
