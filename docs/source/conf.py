import os

project = 'python-daachorse'
copyright = '2026, daac-tools'
author = 'Koichi Akabe'

extensions = [
    'sphinx.ext.autodoc',
]

exclude_patterns = []

on_rtd = os.environ.get('READTHEDOCS', None) == 'True'
if not on_rtd:
    html_theme = 'sphinx_rtd_theme'
