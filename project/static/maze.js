var board = document.getElementById("grid");
var start = set_start();
var goal = set_goal(start);

var map =	[[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];

for (var i = 0; i < board.rows.length; i++) {
    for (var j = 0; j < board.rows[i].cells.length; j++) {
		board.rows[i].cells[j].classList.add("empty");
    }
}

document.getElementById("start").onclick = start_button;
document.getElementById("find").onclick = find_button;

function is_in(arr, ele){
	for (var i = 0; i < arr.length; i++){
		if (coordinate_comp(arr[i], ele)){
			return true;
		}
	}
	return false;
}

function coordinate_comp(arr1, arr2){
	if (arr1[0] == arr2[0] && arr1[1] == arr2[1]){
		return true;
	}
	else{
		return false;
	}
}

function random_gen(){
	return parseInt(Math.random()*10);
}

function set_start(){
	var start = [];
	var carrier = [];
	
	for (var i = 0; i < 1; i++){
		
		carrier[0] = random_gen();
		carrier[1] = random_gen();
		while (is_in(start, carrier)){
			carrier[0] = random_gen();
			carrier[1] = random_gen();
		}
		start[i] = carrier.slice();
	}
	return start;
}

function set_goal(start){
	var goal = [];
	var carrier = [];
	
	for (var i = 0; i < 1; i++){
		
		carrier[0] = random_gen();
		carrier[1] = random_gen();
		while (is_in(goal, carrier) || is_in(start, carrier)){
			carrier[0] = random_gen();
			carrier[1] = random_gen();
		}
		goal[i] = carrier.slice();
	}
	return goal;
}

function start_button () {
	document.getElementById("start").remove();
	document.getElementById("find").style.display = "block";
	for (var i = 0; i < 1; i++){
		board.rows[start[i][0]].cells[start[i][1]].classList.add("start".concat(i));
		board.rows[start[i][0]].cells[start[i][1]].classList.remove("empty");
		board.rows[goal[i][0]].cells[goal[i][1]].classList.add("goal".concat(i));
		board.rows[goal[i][0]].cells[goal[i][1]].classList.remove("empty");
	}
	for (var i = 0; i < board.rows.length; i++) {
		for (var j = 0; j < board.rows[i].cells.length; j++) {
			board.rows[i].cells[j].onclick = board_clicked;
			if(Math.random() > 0.5){
				if (board.rows[i].cells[j].classList.contains("empty")) {
					map[i][j] = 1;
					board.rows[i].cells[j].classList.add("wall");
					board.rows[i].cells[j].classList.remove("empty");
				}
			}
		}
	}
}

//https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
function find_button() {
	fetch("http://localhost/find", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({
			start_loc: start,
			goal_loc: goal,
			my_map: map,
		}),
	})
	.then((response) => response.json())
	.then((data) => build_path(data));
}

function build_path(data){
	if (data.message != null){
		for(var i = 1; i < data.message[0].length-1; i++){
			board.rows[data.message[0][i][0]].cells[data.message[0][i][1]].classList.add("path");
			board.rows[data.message[0][i][0]].cells[data.message[0][i][1]].classList.remove("empty");
		}
	}
	else{
		alert("no path!");
	}
}

function board_clicked(){
	rIndex = this.parentElement.rowIndex;
    cIndex = this.cellIndex;
	if (this.classList.contains("empty")) {
		map[rIndex][cIndex] = 1;
		this.classList.add("wall");
		this.classList.remove("empty");
	}
	else if (this.classList.contains("wall")) {
		map[rIndex][cIndex] = 0;
		this.classList.add("empty");
		this.classList.remove("wall");
	}
}