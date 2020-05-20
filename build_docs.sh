
rm -rf ./docs/

git worktree prune

cd docsource
make html

cd ..

git worktree add docs gh-pages

rm -rf ./docs/*

cp -r docsource/_build/html/* ./docs/

touch ./docs/.nojekyll

cargo doc --no-deps --workspace

mkdir ./docs/crates

cp -r ./target/doc/* ./docs/crates
