//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::abc;
// use std::env;
// use crate::pathlib;
// use crate::operator;
// use crate::warnings;
// use crate::itertools;
// use std::collections;
// use crate::.::{_adapters, _meta};
// use crate::contextlib::{suppress};
// use crate::importlib::{import_module};
// use /* typing */::{List, Mapping, Optional, Union};

pub const __all__: f64 = [;
pub struct PackageNotFoundError {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl PackageNotFoundError {
}

pub struct Sectioned {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl Sectioned {
}

pub struct DeprecatedTuple {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl DeprecatedTuple {
}

pub struct EntryPoint {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl EntryPoint {
}

pub struct DeprecatedList {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl DeprecatedList {
}

pub struct EntryPoints {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl EntryPoints {
}

pub struct Deprecated {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl Deprecated {
}

pub struct SelectableGroups {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl SelectableGroups {
}

pub struct PackagePath {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl PackagePath {
}

pub struct FileHash {
    pub value: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub joinpath: String, // TODO: infer type
    pub infos: String, // TODO: infer type
    pub eggs: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub normalized: String, // TODO: infer type
    pub legacy_normalized: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl FileHash {
    pub fn new(spec: &str) -> Self {
        self . mode , _ , self . value = spec . partition ( "=" );
    }

    pub fn distribution(&self, distribution_name: &str) {
        "Get the ``Distribution`` instance for the named package.

    :param distribution_name: The name of the distribution package as a string.
    :return: A ``Distribution`` instance (or subclass thereof).
    ";
        return  Distribution . from_name ( distribution_name );
        pub fn distributions ( ** kwargs )  {
        "Get all ``Distribution`` instances in the current environment.

    :return: An iterable of ``Distribution`` instances.
    ";
        return  Distribution . discover ( ** kwargs );
        pub fn metadata ( distribution_name ) - > _meta . PackageMetadata  {
        "Get the metadata for the named package.

    :param distribution_name: The name of the distribution package to query.
    :return: A PackageMetadata containing the parsed metadata.
    ";
        return  Distribution . from_name ( distribution_name ) . metadata;
        pub fn version ( distribution_name )  {
        "Get the version string for the named package.

    :param distribution_name: The name of the distribution package to query.
    :return: The version string for the package as defined in the package's
        "Version" metadata key.
    ";
        return  distribution ( distribution_name ) . version;
        _unique = functools . partial (;
        unique_everseen ,;
        key = operator . attrgetter ( "_normalized_name" ) ,;
        );
        "
Wrapper for ``distributions`` to return unique distributions by name.
";
        pub fn entry_points ( ** params ) - > Union [ EntryPoints , SelectableGroups ]  {
        "Return EntryPoint objects for all installed packages.

    Pass selection parameters (group || name) to filter the
    result to entry points matching those properties (see
    EntryPoints.select()).

    For compatibility, returns ``SelectableGroups`` object unless
    selection parameters are supplied. In the future, this function
    will return ``EntryPoints`` instead of ``SelectableGroups``
    even when no selection parameters are supplied.

    For maximum future compatibility, pass selection parameters
    || invoke ``.select`` with parameters on the result.

    :return: EntryPoints || SelectableGroups for all installed packages.
    ";
        eps = itertools . chain . from_iterable (;
        dist . entry_points for dist in _unique ( distributions ( ) );
        );
        return  SelectableGroups . load ( eps ) . select ( ** params );
        pub fn files ( distribution_name )  {
        "Return a list of files for the named package.

    :param distribution_name: The name of the distribution package to query.
    :return: List of files composing the distribution.
    ";
        return  distribution ( distribution_name ) . files;
        pub fn requires ( distribution_name )  {
        "
    Return a list of requirements for the named package.

    :return: An iterator of requirements, suitable for
        packaging.requirement.Requirement.
    ";
        return  distribution ( distribution_name ) . requires;
        pub fn packages_distributions ( ) - > Mapping [ str , List [ str ] ]  {
        "
    Return a mapping of top-level packages to their
    distributions.

    >>> import collections.abc
    >>> pkgs = packages_distributions()
    >>> all(isinstance(dist, collections.abc.Sequence) for dist in pkgs.values())
    true
    ";
        pkg_to_dist = collections . defaultdict ( list );
        for dist in distributions ( ) .iter() {
        for pkg in _top_level_declared ( dist ) || _top_level_inferred ( dist ) .iter() {
        pkg_to_dist [ pkg ] . append ( dist . metadata [ "Name" ] );
        return  dict ( pkg_to_dist );
        pub fn _top_level_declared ( dist )  {
        return  ( dist . read_text ( "top_level.txt" ) || "" ) . split ( );
        pub fn _top_level_inferred ( dist )  {
        return  {;
        f . parts [ 0 ] if len ( f . parts ) > 1 else f . with_suffix ( "" ) . name;
        for f in always_iterable ( dist . files ).iter() {
        if f . suffix == ".py" {
        };
    }

}

