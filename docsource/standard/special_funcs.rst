
.. _special:

====================
Special Functions
====================

.. contents:: Contents
    :local:
    :depth: 2

Print Functions
---------------

.. function:: print([object], ...)  [void]

    Prints its arguments, seperated by tabs, to standard output. The strings that are printed are the equivelent of calling the ``string`` conversion function.
    This function will not automatically add a newline. To get this behavior, use ``println``

.. function:: println([object], ...)  [void]

    Prints its arguments, the same as ``print``, except ``println`` appends a newline at the end.

.. function:: printr([object], ...)  [void]

    Prints its arguments, the same as ``print``, except ``printr`` appends a carriage return at the end.

.. function:: eprint([object], ...)  [void]

    The equivelent of ``print``, but outputs to standard error instead of standard output.

.. function:: eprintln([object], ...)  [void]

    The equivelent of ``println``, but outputs to standard error instead of standard output.

.. function:: eprintr([object], ...)  [void]

    The equivelent of ``printr``, but outputs to standard error instead of standard output.


Special Functions
-----------------

.. function:: exit([int])  [void]

    Exit from a running script with a given error code (0 indicates success)

.. function:: lock([object])  [void]

    Lock an object as immutable permenantly, so future mutations will throw an error.

.. function:: hash([object])  [int]

    Returns the 64-bit unsigned integer hash corresponding to an object. ``hash`` will throw an error if the object is not hashable.

.. function:: clone([object])  [object]

    Will create a deep-clone of an object, if possible. The clone will be unlocked and mutation will be allowed. ``clone`` will throw an error if the object is not cloneable.

.. function:: assert([bool], [string]?)  [void]

    Assert a condition, with an optional message in case the condition fails.

.. function:: tech_version()  ([int], [int], [int])

    Returns the major, minor and patch version of technetium.

.. function:: script_path()  [string]

    Returns the absolute (canonicalized) path to the current running script. Throws an error if this doesn't make sense (i.e. if the script is passed via stdin).

.. function:: sleep([int or float])  [void]

    Returns after waiting a given number of seconds.