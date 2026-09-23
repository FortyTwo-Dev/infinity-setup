#!/usr/bin/env bash
# @module: packages
# @description: Check for available updates without installing anything
# @distro: fedora
# @exit-codes: 100

dnf check-upgrade
