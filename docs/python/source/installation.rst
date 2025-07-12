Installation Guide
==================

This guide will help you install Veloxx Python bindings on your system.

Requirements
------------

* Python 3.7 or higher
* pip (Python package installer)

For development or building from source:

* Rust 1.70 or higher
* maturin (for building Python wheels)

Quick Installation
------------------

Install from PyPI (Recommended)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The easiest way to install Veloxx is from PyPI:

.. code-block:: bash

    pip install veloxx==0.2.4

This will install the latest stable version with pre-compiled binaries for most platforms.

Verify Installation
~~~~~~~~~~~~~~~~~~~

To verify that Veloxx is installed correctly, run:

.. code-block:: python

    import veloxx
    print(f"Veloxx version: {veloxx.__version__}")

If this runs without errors, you're ready to go!

Development Installation
------------------------

If you want to contribute to Veloxx or need the latest development version, you can install from source.

Prerequisites
~~~~~~~~~~~~~

First, install Rust and maturin:

.. code-block:: bash

    # Install Rust (if not already installed)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env

    # Install maturin
    pip install maturin

Clone and Build
~~~~~~~~~~~~~~~

.. code-block:: bash

    # Clone the repository
    git clone https://github.com/Conqxeror/veloxx.git
    cd veloxx

    # Build the Python wheel
    maturin build --release

    # Install the wheel
    pip install target/wheels/veloxx-*-py3-none-any.whl

Virtual Environment (Recommended)
----------------------------------

It's recommended to use a virtual environment to avoid conflicts with other packages:

.. code-block:: bash

    # Create virtual environment
    python -m venv veloxx-env

    # Activate virtual environment
    # On Windows:
    veloxx-env\Scripts\activate
    # On macOS/Linux:
    source veloxx-env/bin/activate

    # Install Veloxx
    pip install veloxx==0.2.4

Platform-Specific Notes
-----------------------

Windows
~~~~~~~

* Ensure you have Microsoft Visual C++ 14.0 or greater installed
* If you encounter build issues, install Visual Studio Build Tools

macOS
~~~~~

* Xcode command line tools are required for building from source
* Install with: ``xcode-select --install``

Linux
~~~~~

* GCC or Clang compiler required for building from source
* On Ubuntu/Debian: ``sudo apt-get install build-essential``
* On CentOS/RHEL: ``sudo yum groupinstall "Development Tools"``

Troubleshooting
---------------

Import Error
~~~~~~~~~~~~

If you get an import error, make sure you're using the correct Python environment:

.. code-block:: bash

    which python
    pip list | grep veloxx

Build Errors
~~~~~~~~~~~~

If you encounter build errors when installing from source:

1. Ensure Rust is properly installed and up to date
2. Update maturin: ``pip install --upgrade maturin``
3. Clear the build cache: ``rm -rf target/``
4. Try building again

Performance Issues
~~~~~~~~~~~~~~~~~~

For optimal performance:

* Use the latest version of Veloxx
* Ensure you're using a 64-bit Python installation
* Consider using PyPy for even better performance in some cases

Getting Help
------------

If you encounter issues during installation:

* Check our `GitHub Issues <https://github.com/Conqxeror/veloxx/issues>`_
* Read the `Contributing Guide <https://github.com/Conqxeror/veloxx/blob/main/CONTRIBUTING.md>`_
* Join our community discussions