PREV=""
PREV_DONE="0"
PREV_SEC=$SECONDS
while [[ 1 -eq 1 ]]; do
	OUTPUT=$(./check-status.sh)
	CURR=$(echo $OUTPUT | sha256sum)
	if [[ $CURR != $PREV ]]; then
		echo -ne "\r"
		echo -n $OUTPUT
		echo -n " "


		DONE=$(echo $OUTPUT | cut -d " " -f 3 | cut -d "/" -f 1)
		TOTAL=$(echo $OUTPUT | cut -d " " -f 3 | cut -d "/" -f 2)
		REMAINING=$(($TOTAL - $DONE))
		DONE_NOW=$(($DONE - $PREV_DONE))
		ELAPSED=$(($SECONDS - $PREV_SEC))

		# echo $REMAINING $DONE $ELAPSED

		if [[ $ELAPSED -ne 0 ]]; then
			ETA=$(($ELAPSED * $REMAINING / $DONE_NOW))
		else
			ETA="0"
		fi

		echo -n $(date -d "00:00 today + $ETA seconds" '+%H:%M:%S')

		PREV_SEC=$SECONDS
		PREV_DONE=$DONE
	fi
	PREV=$CURR
	sleep 15
done