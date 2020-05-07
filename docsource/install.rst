
.. _install:

====================
Installation
====================

.. contents:: Contents
    :local:
    :depth: 2

Technetium is distributed as a single binary, ``tc``. Pre-built binaries aren't yet available.

Building
--------

To build, make sure you have cargo and rust installed. If not, follow `these instructions <https://doc.rust-lang.org/book/ch01-01-installation.html>`_ to install rustup and cargo.

Next, navigate to the root directory of the project. Run ``cargo build --release`` to create a release binary located at ``target/release/tc``.

To build this documentation, first install sphinx-build using `the instructions here <https://www.sphinx-doc.org/en/master/usage/installation.html>`_. Then, run ``build_docs.sh`` from the main directory.
