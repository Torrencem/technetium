
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

.. function:: [string].lines()  [lines]

    Returns an iterator over the lines in the string. The lines can be seperated by either '\\n' or '\\n\\r', and the strings returned will not contain any newlines.

.. function:: [string].escape()  [string]

    Returns an "escaped" version of this string. The escaped version will replace non-printable characters such as newlines and tabs with escape codes such as '\\n' and '\\t' respectively.


List
----

Lists are created through square bracket ``[]`` literals. Lists can be iterated over through for loops::

    l = ["alpha", 123, 3.14]

    for val in l {
        print(val)
    }

List Methods
^^^^^^^^^^^^

.. function:: [list].length()  [int]

    Returns the length of the list

Tuple
-----

Tuples are similar to lists, except that they are immutable. A tuple is typically expected to be a fixed length container, where the type of each part is known in advanced, but this is not enforced. Tuples are constructed through parenthetic ``()`` literals, similar to lists, and can similarly be indexed, but cannot be changed once constructed.

Tuple Methods
^^^^^^^^^^^^^

.. function:: [tuple].length()  [int]

    Returns the number of elements in the tuple


Numeric Types (float and int)
-----------------------------

Numerical types can be constructed through literals, or through their respective ``float`` and ``int`` conversion functions. Integers and floats can be operated together, and the result will be a float. Floats are internally 64-bit double precision floating point numbers.


Boolean
-------

Everything in technetium is either truthy or falsey, in addition to the boolean type. Booleans can either be created through the ``true`` and ``false`` literals, or through the conversion function ``bool``. The conversion function returns ``true`` when it's argument is truthy, and false otherwise.


Char
----

A char is a 'Unicode scalar value' which mirrors `the "char" type in Rust <https://doc.rust-lang.org/std/primitive.char.html>`_. Char's can either be constructed through char literals in single quotes ``''``, by indexing a string, or through the conversion function ``char``. Through string indexing, specific characters can be set in a string, even though strings internally are rerpresented as valid UTF-8::

    my_utf8_char = 'ℝ'
    my_phrase = "The real numbers are sometimes called _"
    my_phrase[-1] = my_utf8_char
    print(my_phrase)


Slice
-----

Slices are references to sections in a list or string. Slices can be constructed with range syntax the same as that of python (see `this python tutorial <https://docs.python.org/3/tutorial/introduction.html>`_ for some examples)::

    my_phrase = "i like to eat Σ π"
    a_slice = my_phrase[:6]
    my_phrase[0] = 'I'
    print(a_slice)
