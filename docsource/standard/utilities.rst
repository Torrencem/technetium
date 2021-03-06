
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

.. function:: which([string])  [string]

    Equivelent of the Unix command ``which``. Wraps the `rust crate 'which' <https://docs.rs/which/3.1.1/which/>`_.

.. function:: open([string])  [void]

    Opens a path (either a URL or a file path) with the default system program. Uses ``xdg-open`` on linux, and ``open`` on mac. See the `rust crate 'opener' <https://docs.rs/opener/0.4.1/opener/fn.open.html>`_ for more information.

.. function:: exists([string])  [bool]

    Test if a path exists.

.. function:: is_directory([string])  [bool]

    Test if a path is a directory. Returns false if directory does not exist.

.. function:: canonicalize([string])  [string]

    Canonicalize e.g. a relative path, to make an absolute path.

.. function:: strip_path_prefix(path: [string], prefix: [string])  [string]

    Removes a prefix from a given path. This could be used, for example, to find the relative path of a file within a directory. Throws an error if the given path doesn't contain the given prefix.

.. function:: args()  [list(string)]

    Returns a list of the command line arguments passed to the script, excluding the executable name and script name, if applicable.

.. function:: os()  [string]

    Returns "Linux" or "Darwin" depending on the operating system the script is running on.

Miscellaneous Utilities
-----------------------

The following utilities either come from the rust ``sys-info`` crate, or the rust ``whoami`` crate, both for getting various information about the system.

.. function:: linux_distro()  [string]

    Returns the name of the linux distribution the script is running on, or "Unknown" if it's not known. Throws an error on unsupported systems (non-linux systems). This information is sourced from /etc/os-release. See `the sys-info rust crate <https://docs.rs/sys-info/0.6.1/sys_info/fn.linux_os_release.html>`_ for more information.


.. function:: hostname()  [string]

    Returns the host device's hostname.

.. function:: device_name()  [string]

    Returns a "pretty name" for the system, which is used for bluetooth pairing.

.. function:: real_name()  [string]

    Returns the real name of the current user.

.. function:: username()  [string]

    Returns the username of the current user.

.. function:: languages()  [list(string)]

    Returns a list of the languages in order of preference of the current user. For example: ``['en-US', 'en']``

.. function:: desktop_env()  [string]

    Returns the current desktop environment of the user, or ``"Unknown: ..."``
