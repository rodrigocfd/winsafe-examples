# Runs cargo check on each project.

PROJS=(
	01_button_click
	02_native_controls
	03_dialog_resources
	04_custom_control
	05_resizable_layout
	06_tabs
	07_video_playback
)

set -e

print_elapsed () {
	MIN=$(( ($1 - ($1 % (60 * 1000))) / (1000 * 60) ))
	SEC=$(( ($TF - ($MIN * 1000 * 60) - ($1 % 1000)) / 1000 ))
	MS=$(( $1 % 1000 ))

	PURP='\033[0;35m'
	NC='\033[0m'
	if (($MIN > 0)); then
		printf "    ${PURP}Duration${NC} %02d:%02d.%03d min\n" $MIN $SEC $MS
	else
		printf "    ${PURP}Duration${NC} %d.%03d sec\n" $SEC $MS
	fi
}

for PROJ in "${PROJS[@]}" ; do
	echo "$PROJ..."
	T0=$(date +%s%N)

	cd $PROJ
	cargo check
	cd ..

	TF=$((($(date +%s%N) - $T0)/1000000))
	print_elapsed $TF
done
