
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
    println(~"{my_name}'s favorite number is {my_favorite_num}")

String Methods
^^^^^^^^^^^^^^

.. function:: [string].length()  [int]

    Returns the length of the string.

.. function:: [string].contains([char])  [bool]

    Check if the string contains a given character

.. function:: [string].chars()  [chars]

    Returns an iterator over the characters in the string.

.. function:: [string].lines()  [lines]

    Returns an iterator over the lines in the string. The lines can be seperated by either '\\n' or '\\n\\r', and the strings returned will not contain any newlines.

.. function:: [string].escape()  [string]

    Returns an "escaped" version of this string. The escaped version will replace non-printable characters such as newlines and tabs with escape codes such as '\\n' and '\\t' respectively.

.. function:: strip_prefix(base: [string], prefix: [string]) [string or unit]

    Removes a prefix from a string. Returns unit if the string does not contain the prefix.
    
.. function:: strip_suffix(base: [string], prefix: [string]) [string or unit]

    Removes a suffix from a string. Returns unit if the string does not contain the suffix.

TODO: Wrap a bunch of Rust string methods

List
----

Lists are created through square bracket ``[]`` literals, or through the ``list`` conversion function. Lists can be iterated over through for loops::

    l = ["alpha", 123, 3.14]

    for val in l {
        println(val)
    }

List Methods
^^^^^^^^^^^^

.. function:: [list].length()  [int]

    Returns the length of the list

.. function:: [list].contains([object])  [bool]

    Check if the list contains a given object

.. function:: [list].push([object])  [void]

    Push an object on the end of the list

.. function:: [list].pop()  [object]

    Remove and return the object at the end of the list

.. function:: [list].append([iter])  [void]

    Append all of the elements from an iterable to the list

Tuple
-----

Tuples are similar to lists, except that they are immutable. A tuple is typically expected to be a fixed length container, where the type of each part is known in advanced, but this is not enforced. Tuples are constructed through parenthetic ``()`` literals, similar to lists, and can similarly be indexed, but cannot be changed once constructed.

Tuple Methods
^^^^^^^^^^^^^

.. function:: [tuple].length()  [int]

    Returns the number of elements in the tuple

.. function:: [tuple].contains([object])  [bool]

    Check if the tuple contains a given object

Set
---

Sets are containers of hashable objects that ignore duplicates. Sets are constructed through bracket ``{}`` literals, or through the ``set`` conversion function, similar to lists. Note that elements inside sets must be hashable and immutable. As all objects are essentially mutable, any objects added to the set will be "locked" immutable, so further changes to these objects will throw an error.

Set Methods
^^^^^^^^^^^

.. function:: [set].length()  [int]

    Returns the number of elements in the set

.. function:: [set].contains([object])  [bool]

    Check if the set contains a given object

.. function:: [set].add([object])

    Add an object to the set. Locks the given object as immutable. Will throw an error if the object is not hashable

.. function:: [set].remove([object])  [bool]

    Remove an object from the set. Will throw an error if the object is not hashable. Returns whether or not anything was found and removed from the set

Dictionary
----------

Dictionaries are mappings from hashable and immutable key values to objects. Dictionaries are constructed through bracket literals. Dictionaries are primarily useful through the square indexing brackets ``[]``::

        my_dict = {"name": "Matthew", "favorite number": 123}

        println(my_dict["name"])

Dictionaries throw an error if accessed with a key that either doesn't exist, or isn't hashable.

Note that syntatically, dictionaries are allowed to span multiple lines. Sets, although very similar in form, are not, due to parsing restrictions.

Dictionary Methods
^^^^^^^^^^^^^^^^^^

.. function:: [dictionary].length()  [int]

        Returns the number of key value pairs in the dictionary

Numeric Types (float and int)
-----------------------------

Numerical types can be constructed through literals, or through their respective ``float`` and ``int`` conversion functions. Integers and floats can be operated together, and the result will be a float. Floats are internally 64-bit double precision floating point numbers, and integers are arbitrary sized. Using an integer that is outside the set of values representable by a 64-bit integer as an index or in a slice will throw an error.


Boolean
-------

Everything in technetium is either truthy or falsey, in addition to the boolean type. Booleans can either be created through the ``true`` and ``false`` literals, or through the conversion function ``bool``. The conversion function returns ``true`` when it's argument is truthy, and false otherwise.


Unit
----

The unit type has only one member, ``unit``. This type is implicitly returned from all functions which don't return anything else, and is useful as a generic ``None`` object::

    println(println(5) == unit)  # Prints 5, then true


Char
----

A char is a 'Unicode scalar value' which mirrors `the "char" type in Rust <https://doc.rust-lang.org/std/primitive.char.html>`_. Char's can either be constructed through char literals in single quotes ``''``, by indexing a string, or through the conversion function ``char``. Through string indexing, specific characters can be set in a string, even though strings internally are rerpresented as valid UTF-8::

    my_utf8_char = 'ℝ'
    my_phrase = "The real numbers are sometimes called _"
    my_phrase[-1] = my_utf8_char
    println(my_phrase)


Slice
-----

Slices are references to sections in a list or string. Slices can be constructed with range syntax the same as that of python (see `this python tutorial <https://docs.python.org/3/tutorial/introduction.html>`_ for some examples)::

    my_phrase = "i like to eat Σ π"
    a_slice = my_phrase[:6]
    my_phrase[0] = 'I'
    println(a_slice)


Anonymous Functions
-------------------

Anonymous functions, also sometimes known as "lambda functions", are unnamed function objects that have condensed definition syntax::

    add_a = \(x, y) -> x + y
    add_b = \(x, y) -> {
        return x + y
    }
    println(add_a(10, 5))
    println(add_b(10, 5))

Anonymous functions are written as a backslash followed by either the name of a single argument, or a parenthesized list of arguments, followed by an arrow, then either a function block or a single expression. They also can capture their environment::

    func make_adder(c) {
        return \x -> x + c
    }

    f = make_adder(100)
    println(f(25))

To create an anonymous function with no arguments, use an empty list of arguments::

    counter = 1

    f = \() -> counter++

    println(f())  # 1
    println(f())  # 2
