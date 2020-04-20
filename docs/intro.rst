
.. _intro:

====================
Introduction to rush
====================

.. contents:: Contents
    :local:
    :depth: 2

The Shell Operator
-------------

Rush is designed around making accessing shell commands as easy as possible. The core of this design is the shell operator ($). If you begin a line with the shell operator, it will be run on the command line::

        my_name = "matt"
        $ echo {my_name}

This code, when run, will invoke the `echo` command, substituting the value of the expression `my_name`, which has the effect of printing out `matt` to the console.

The shell operator makes it very easy to write long chains of commands in a similar style to a bash script. Similar to in bash, each command will run synchronously and in the foreground. The shell operator can also be used to create process objects, which can be assigned to variables, spawned in the background, or even passed to functions::

        my_tmp_path = "/tmp/hello"
        $ mkdir {my_tmp_path}
        file_deleter = ($ rm -rf {my_tmp_path})
        global_defer(file_deleter)  # Run file_deleter on program exit


