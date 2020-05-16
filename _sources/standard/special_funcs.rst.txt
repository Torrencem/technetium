
.. _special:

====================
Special Functions
====================

.. contents:: Contents
    :local:
    :depth: 2

Special Functions
-----------------

.. function:: print([object], ...)  [void]

    Prints its arguments, seperated by tabs. The strings that are printed are the equivelent of calling the ``string`` conversion function.

.. function:: exit([int])  [void]

    Exit from a running script with a given error code (0 indicates success)

.. function:: lock([object])  [void]

    Lock an object as immutable permenantly, so future mutations will throw an error.

.. function:: hash([object])  [int]

    Returns the 64-bit unsigned integer hash corresponding to an object. ``hash`` will throw an error if the object is not hashable.

.. function:: clone([object])  [object]

    Will create a deep-clone of an object, if possible. The clone will be unlocked and mutation will be allowed. ``clone`` will throw an error if the object is not cloneable.
