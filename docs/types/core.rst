
.. _core:

====================
Core Types
====================

.. contents:: Contents
    :local:
    :depth: 2

String
------

Strings are created through literals surrounded by quotes ``""``, format literals ``~""``, or through the ``string`` conversion function. 


Format Literals
^^^^^^^^^^^^^^^

A format literal is a string literal with a tilde ``~`` in front. Format literals can contain code surrounded by braces ``{}`` that will be evaluated and placed in the appropriate place in the string. The evaluated expressions are converted to strings using the conversion function ``string``::

    my_name = "Matt"
    my_favorite_num = 123 * 345 + 2
    print(~"{my_name}'s favorite number is {my_favorite_num}")

String Methods
^^^^^^^^^^^^^^

.. function:: [string].length()  [int]

    Returns the length of the string.

.. function:: [string].chars()  [chars]

    Returns an iterator over the characters in the string.

.. function:: [string].lines()  [chars]

    Returns an iterator over the lines in the string. The lines can be seperated by either '\\n' or '\\n\\r', and the strings returned will not contain any newlines.

.. function:: [string].escape()  [string]

    Returns an "escaped" version of this string. The escaped version will replace non-printable characters such as newlines and tabs with escape codes such as '\\n' and '\\t' respectively.
