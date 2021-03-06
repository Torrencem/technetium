# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
# import os
# import sys
# sys.path.insert(0, os.path.abspath('.'))


# -- Project information -----------------------------------------------------

project = 'technetium'
copyright = '2020, Matt Torrence'
author = 'Matt Torrence'

# The full version, including alpha/beta/rc tags
release = '0.1.0'


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
]

# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'alabaster'

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ['static']
master_doc = 'index'

from pygments.lexer import RegexLexer
from pygments.token import *
# from technetium_lexer import TechnetiumLexer
class TechnetiumLexer(RegexLexer):
    name = 'Technetium'
    aliases = ['technetium', 'tech', 'tc']
    filenames = ['*.tc']

    tokens = {
        'root': [
            (r'false|true|if|else|elif|for|in|while|case|of|func|return|unit', Keyword),
            (r'\d+\.?\d*', Number),
            (r'\-\>|\=\>|\<\=|\>\=|\!\=|\=\=|\|\||\&\&|\+\=|\-\=|\*\=|\/\=|\%\=|\*|\/|\+|\-|\%|\:\|\>|\<|\=|\~|\[|\]|\:|\\', Operator),
            (r'\{|\}|\(|\)|\.|,', Punctuation),
            (r'\s+', Text),
            (r'#.*?$', Comment),
            ('\'', String.Char, 'char'),
            (r'"', String, 'string'),
            (r'\$', Operator, 'shell'),
            (r'\w[\w0-9]*', Name),
        ],
        'string': [
            (r'[^"\\]+', String),
            (r'\\"', String.Escape),
            (r'"', String, '#pop'),
        ],
        'char': [
            (r"[^'\\]+", String.Char),
            (r"\\'", String.Escape),
            (r"'", String.Char, '#pop'),
        ],
        'shell': [
            (r'\\{', String.Escape),
            (r'\{|\}', Punctuation),
            (r'[^\n\{\}\\]+', String.Affix),
            (r'\n', Text, '#pop'),
        ],
    }

highlight_language = "technetium"
from sphinx.highlighting import lexers
lexers['technetium'] = TechnetiumLexer(startinline=True)
