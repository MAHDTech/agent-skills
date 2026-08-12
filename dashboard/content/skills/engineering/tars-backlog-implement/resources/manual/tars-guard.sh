# tars-guard.sh -- shared location guard, sourced by the tars-* runners.
#
# Refuses to operate on a directory that is neither a spoke clone under
# $TARS_SPOKE_ROOT nor the run's own repository root. The failure this exists
# for is mechanical, not disobedient: a script does
#
#     cd "$SPOKE_DIR"          # fails (clone not created yet, typo, race)
#     git checkout -b subagent-N
#
# and, because `set -e` does NOT fire when the failure is masked by a pipeline
# or a redirect, the git command runs in whatever directory the shell was
# already in -- typically the developer's live repository.
#
# Sets: TARS_GUARD_REPO_ROOT (the run's real repo root)
# Honours: TARS_ALLOW_ANY_CWD=1 to bypass (prints a warning; use only when a
#          human has consciously decided the target is correct).

tars_guard_repo_root() {
	# run.env lives at <repo>/.tars/run.env. In a spoke clone `.tars` is a
	# symlink to the parent, so resolving it yields the REAL repo either way.
	_ge=$(readlink -f "$1" 2>/dev/null || echo "$1")
	_gd=$(dirname -- "$_ge")   # <repo>/.tars
	dirname -- "$_gd"          # <repo>
}

# tars_guard_check <tool-name> <target-dir> <run-env-path>
tars_guard_check() {
	_gtool=$1
	_gtarget=$2
	_grunenv=$3

	TARS_GUARD_REPO_ROOT=$(tars_guard_repo_root "$_grunenv")
	_greal=$(readlink -f "$_gtarget" 2>/dev/null || echo "$_gtarget")
	_groot=$(readlink -f "$TARS_GUARD_REPO_ROOT" 2>/dev/null || echo "$TARS_GUARD_REPO_ROOT")

	# The run's own repository is always legitimate: the Hub gates the topic
	# branch there after each batch.
	if [ "$_greal" = "$_groot" ]; then
		return 0
	fi

	# A spoke clone must live under the spoke root.
	if [ -n "${TARS_SPOKE_ROOT:-}" ]; then
		_gspokes=$(readlink -f "$TARS_SPOKE_ROOT" 2>/dev/null || echo "$TARS_SPOKE_ROOT")
		case "$_greal/" in
		"$_gspokes"/*) return 0 ;;
		esac
	fi

	if [ "${TARS_ALLOW_ANY_CWD:-0}" = "1" ]; then
		echo "$_gtool: WARNING operating outside the repo root and spoke root (TARS_ALLOW_ANY_CWD=1): $_greal" >&2
		return 0
	fi

	echo "$_gtool: REFUSING to operate on '$_greal'." >&2
	echo "$_gtool:   It is neither the run's repository root ($_groot)" >&2
	echo "$_gtool:   nor a spoke clone under \$TARS_SPOKE_ROOT (${TARS_SPOKE_ROOT:-unset})." >&2
	echo "$_gtool: This usually means an earlier 'cd' failed and the command is about" >&2
	echo "$_gtool: to run in the wrong repository. Fix the cd; do not set TARS_ALLOW_ANY_CWD." >&2
	exit 65
}
