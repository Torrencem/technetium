
.. _utilities:

====================
Utility Functions
====================

.. contents:: Contents
    :local:
    :depth: 2

Command Line Utils
------------------

.. function:: cd([string])  [void]

    Change the present working directory of the main process. Note that the following do *not* have the same effect::

        cd("my_dir")
        # This one does not work:
        $ cd my_dir

    This is because using the shell operator ($) spawns a subprocess, which does not affect the parent process.

.. function:: args()  [list(string)]

    Returns a list of the command line arguments passed to the script, excluding the executable name and script name, if applicable.

.. function:: os()  [string]

    Returns "Linux" or "Darwin" depending on the operating system the script is running on.

.. function:: linux_distro()  [string]

    Returns the name of the linux distribution the script is running on, or "Unknown" if it's not known. This information is sourced from /etc/os-release. See `this rust crate <https://docs.rs/sys-info/0.6.1/sys_info/fn.linux_os_release.html>`_ for more information
