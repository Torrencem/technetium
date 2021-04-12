
.. _math:

====================
Math Functions
====================

.. contents:: Contents
    :local:
    :depth: 2

Modulus
-------

Modulus in technetium always gives a non-negative result. So, for example::

    print(-2 % 3)

will print 1, since -2 is equivalent to 1 mod 3.

Numeric Functions
-----------------

.. function:: [trigonometric function]([float or int])  [float]

    Available trigonometric functions are [sin, cos, tan, arcsin, arccos, arctan]

.. function:: abs([float or int])  [float or int]

    Returns the absolute value of its argument

.. function:: sqrt([float or int])  [float]

    Returns the square root of its argument

.. function:: exp([float or int])  [float]

    Returns e to the power of its argument

.. function:: ln([float or int])  [float]

    Returns the natural logarithm of its argument

Random Functions
----------------

.. function:: rand()  [float]

    Returns a random floating point number between 0 and 1

.. function:: rand_range([float or int], [float or int])  [float or int]

    Returns a random floating point number between it's first argument, and up to but not including its second argument

.. function:: rand_normal(mean:[float or int]?, stdev[float or int]?)  [float]

    Returns a randomly sampled number in the normal distribution of a given mean and standard deviation (or 0 and 1 respectively if not given)

