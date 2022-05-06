
.. _shell:

====================
Shell Objects
====================

.. contents:: Contents
    :local:
    :depth: 2

Shell Objects
-------------

Shell objects are ready, spawned, or finished processes which are created using the ``sh`` function. Unlike using the shell operator $, shell objects are able to be spawned in the background, and you can read their output. For example::

    # Output files in the current directory to the screen:
    $ ls

    # If I want to iterate through these files, I should use 'sh':
    ls = sh("ls")
    ls.join()

    for filename in ls.stdout().lines() {
        # ...
    }

As you can see, shell objects have methods such as ``join()`` and ``stdout()``, where the shell operator is forced to run synchronously in the foreground.

It's important to note that the shell operator also automatically formats any code inside ``{}`` into the command. To replicate this, use a formatted string with ``~""``::

    my_name = "matt"
    
    sh(~"echo {my_name}")
    
    # Similar to:
    $ echo {my_name}


Shell Object Methods
--------------------

Shell methods that would otherwise return ``unit`` instead return references to themselves, to facilitate builder patterns.

.. function:: [sh].spawn()  [sh]

    Spawn the shell object as a subprocess in the background.

.. function:: [sh].join()  [sh]

    Wait for the completion of the shell object as a subprocess. Calls ``spawn`` if the subprocess has not been created yet.

.. function:: [sh].stdout()  [string]

    Returns the output of the subprocess's standard output as a string.

.. function:: [sh].stderr()  [string]

    Returns the output of the subprocess's standard error as a string.

.. function:: [sh].exit_code()  [int]

    Returns the exit code of a finished process. Throws an error if the process has not finished; use ``.join()`` to make sure the process is finished first.

.. function:: [sh].kill()  [sh]

    Sends a kill signal to a running proccess.

.. function:: [sh].cwd([string]) [sh]

    Set the current working directory of a process, only if called before the process is spawned.
    
    Throws an error if the process has already started.

.. function:: [sh].env([dict or list of tuples]) [sh]

    Sets the environment of a process, only if called before the process is launched.

    Throws an error if the process has already started.