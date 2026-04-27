#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: ./release.sh 1.0.0" >&2
}

fail() {
  echo "release.sh: $*" >&2
  exit 1
}

run_step() {
  local label="$1"
  shift

  echo "==> ${label}"
  if ! "$@"; then
    fail "${label} failed"
  fi
}

set_cargo_version() {
  local manifest="$1"
  local tmp_file
  tmp_file="$(mktemp)"

  if ! awk -v version="${version}" '
    BEGIN {
      in_package = 0
      updated = 0
    }

    /^\[package\][[:space:]]*$/ {
      in_package = 1
      print
      next
    }

    /^\[/ && in_package {
      in_package = 0
    }

    in_package && /^version[[:space:]]*=/ && !updated {
      print "version = \"" version "\""
      updated = 1
      next
    }

    {
      print
    }

    END {
      if (!updated) {
        exit 42
      }
    }
  ' "${manifest}" > "${tmp_file}"; then
    rm -f "${tmp_file}"
    fail "could not update ${manifest}"
  fi

  mv "${tmp_file}" "${manifest}"
}

set_cargo_lock_version() {
  local lockfile="$1"
  local package_name="$2"
  local tmp_file
  tmp_file="$(mktemp)"

  if ! awk -v package_name="${package_name}" -v version="${version}" '
    BEGIN {
      in_package = 0
      matched_package = 0
      updated = 0
    }

    /^\[\[package\]\][[:space:]]*$/ {
      in_package = 1
      matched_package = 0
      print
      next
    }

    in_package && /^name[[:space:]]*=/ {
      matched_package = ($0 == "name = \"" package_name "\"")
      print
      next
    }

    in_package && matched_package && /^version[[:space:]]*=/ && !updated {
      print "version = \"" version "\""
      updated = 1
      next
    }

    {
      print
    }

    END {
      if (!updated) {
        exit 42
      }
    }
  ' "${lockfile}" > "${tmp_file}"; then
    rm -f "${tmp_file}"
    fail "could not update ${lockfile}"
  fi

  mv "${tmp_file}" "${lockfile}"
}

if [ "$#" -ne 1 ]; then
  usage
  exit 1
fi

version="${1#v}"
if ! [[ "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  usage
  fail "version must be plain semver, for example 1.0.0"
fi

tag="v${version}"
release_branch="${RELEASE_BRANCH:-master}"

command -v cargo >/dev/null 2>&1 || fail "cargo is required"
command -v git >/dev/null 2>&1 || fail "git is required"

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || fail "must be run from inside the git repository"

branch="$(git symbolic-ref --quiet --short HEAD || true)"
[ -n "${branch}" ] || fail "cannot release from a detached HEAD"
[ "${branch}" = "${release_branch}" ] || fail "must be run from ${release_branch}, current branch is ${branch}"

git remote get-url origin >/dev/null 2>&1 || fail "missing git remote named origin"

if [ -n "$(git status --porcelain)" ]; then
  git status --short >&2
  fail "working tree must be clean before release"
fi

if git rev-parse --quiet --verify "refs/tags/${tag}" >/dev/null; then
  fail "local tag ${tag} already exists"
fi

if git ls-remote --exit-code --tags origin "refs/tags/${tag}" >/dev/null 2>&1; then
  fail "remote tag ${tag} already exists on origin"
fi

echo "==> Applying version ${version}"
set_cargo_version "backend/Cargo.toml"
set_cargo_version "frontend/Cargo.toml"
set_cargo_lock_version "backend/Cargo.lock" "backend"
set_cargo_lock_version "frontend/Cargo.lock" "frontend"

run_step "Backend tests" cargo test --manifest-path backend/Cargo.toml
run_step "Frontend test build" cargo test --manifest-path frontend/Cargo.toml --target wasm32-unknown-unknown --no-run

git add backend/Cargo.toml backend/Cargo.lock frontend/Cargo.toml frontend/Cargo.lock
if git diff --cached --quiet; then
  fail "version ${version} did not change any release files"
fi

git commit -m "chore: release ${tag}"

upstream="$(git rev-parse --abbrev-ref --symbolic-full-name '@{u}' 2>/dev/null || true)"
if [ -n "${upstream}" ]; then
  git push
else
  git push -u origin "${branch}"
fi

git tag -a "${tag}" -m "Release ${tag}"
git push origin "${tag}"

echo "==> Released ${tag}"
