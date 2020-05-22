
.. _functional:

======================
Functional Programming
======================

.. contents:: Contents
    :local:
    :depth: 2

Functional utility functions
----------------------------

.. function:: map([iterable], [function(object) => object])  [map]

    Returns a map object, which can be iterated over, and will apply its function
    lazily to each element of another iterable.

.. function:: filter([iterable], [function(object) => object])  [filter]

    Returns a filter object, which can be iterated over, and will yield only the
    elements in its iterator which satisfy the given predicate.
