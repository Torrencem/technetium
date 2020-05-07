
.. _intro:

====================
Introduction to rush
====================

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

