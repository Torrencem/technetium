
rm -rf ./docs/

cd docsource
make html

cd ..
mv docsource/_build/html ./docs/

touch ./docs/.nojekyll
