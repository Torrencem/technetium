
.. _intro:

==========================
Introduction to Technetium
==========================

.. contents:: Contents
    :local:
    :depth: 2

The Shell Operator
------------------


Technetium is designed around making accessing shell commands as easy as possible. The core of this design is the shell operator ($). If you begin a line with the shell operator, it will be run on the command line::

        my_name = "matt"
        $ echo {my_name}

This code, when run, will invoke the ``echo`` command, substituting the value of the expression ``my_name``, which has the effect of printing out ``matt`` to the console.

The shell operator makes it very easy to write long chains of commands in a similar style to a bash script. Similar to bash, each command will run synchronously and in the foreground. The ``sh`` function can be used to create process objects, which can be assigned to variables, spawned in the background, or even passed to functions::

        delayed_command = sh("sleep 10 && echo Ten seconds have passed!")
        delayed_command.spawn()
        # ... do more work ...

Technetium makes it very easy to surround shell code by easy to understand syntax::

        for n in range(10) {
                if n <= 5 {
                        $ mkdir {n}
                } else {
                        $ mkdir {n}_other
                        $ touch {n}_other/index.html
                }
        }

Technetium as a Build System
----------------------------


Technetium can be used as a build system. The ``-r`` flag recursively searches up directories until it finds a ``make.tc`` file (this filename is configurable in several ways). It might be helpful to either put ``scripts/tcmake`` in your path, or to alias ``tcmake`` to ``tech -r`` to make it easier to use this feature, so that ``tcmake`` will make your current project.

In using technetium as a build system, one command that will come in handy is the ``stale`` command, which can be used to emulate systems such as ``make`` which check to see if files have changed since you've last built your project:

.. function:: stale([list(string)]) or stale([string], [string], ...)  [list(string)]

    Returns a list of which of the files given as an argument have changed since the last time this function was called. This function also accepts Unix glob-patterns (i.e. ``./src/**/*``)::
        
        if stale(["src/**/*.c", "main.c"]) {
            println("performing build...")
        } else {
            println("nothing needs to be done!")
        }
    
    This code will print "performing build..." whenever any of the C source code files underneath the source directory, or the main.c file have changed since the last run of the script. It keeps track using the file at ``./.tcmake/stale.cache``.

    It might be helpful to add ``script_path()`` as one of the files to check, in case updating the script itself should be of interest (i.e., in a build script).