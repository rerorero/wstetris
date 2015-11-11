import $ from 'jquery'


/////////////////////////////////////////////////////////////
// Utility
/////////////////////////////////////////////////////////////
class Logger {
	constructor() {}
	i(m) {console.log(m);}
	e(m) {console.error(m);}
}

const logger = new Logger();

class ServerConn {
	constructor(onRecv, onClose) {
    //this.uri = "ws://127.0.0.1:2794/";
    this.uri = "ws://ec2-52-192-144-227.ap-northeast-1.compute.amazonaws.com/ws/";
		this.onRecv = onRecv;
		this.onClose = onClose;
		this.ws = null;
	}

	open(onOpen) {
    if (this.ws === null) {
      this.ws = new WebSocket(this.uri);
			this.ws.binaryType = "arraybuffer";
	    logger.i("Try to open WS..");

	    this.ws.onopen = (e) => {
	      logger.i("Open succeed. " + e);
				onOpen(e);
	    };

	    this.ws.onmessage = (e) => {
	      logger.i("Received message." + e.data);
				if (e.data instanceof ArrayBuffer) {
					this.onRecv(e.data);
				} else {
					logger.e("not binary message.");
					logger.e(e.data)
				}
	    };

	    this.ws.onclose = (e) => {
	      logger.i("Connection closed completely." + e);
				this.onClose(e);
				this.ws = null;
	    }

	    this.ws.onerror = (e) => {
	      logger.e("Connection error.");
				this.ws.close();
				this.ws = null;
				this.onClose(e);
	    }
		} else {
			logger.e("WS has already opend.");
		}
	}

	available() { return this.state() == "OPEN"; }

	state() {
		if (this.ws === null) {
			  return "CLOSED";
		} else {
			switch (this.ws.readyState) {
				case 0:
				  return "CONNECTING";
				case 1:
				  return "OPEN";
				case 2:
				  return "CLOSING";
				case 3:
				  return "CLOSED";
			}
			return `UNKNOWN(${this.ws.readyState})`
		}
	}

	send(data) {
		if (!this.available()) {
			logger.e("Connections not available: " + this.state());
			return;
		}
		this.ws.send(data);
	}

	close() {
		if (!this.available()) {
			logger.e("Connections not available: " + this.state());
			return;
		}
		this.ws.close(1000);
	}
}

/////////////////////////////////////////////////////////////
// View
/////////////////////////////////////////////////////////////
// board state
const B_EMPTY = 0;
const B_COLOR_MIN = 10;
const B_COLOR_MAX = 16;
const COLORS = [
	"#5C4B51", "#8CBEB2", "#F2EBBF", "#F3B562", "#F06060", "#2E97D8", "#294666"
]
const B_PILEDCOLOR_MIN = 20;
const B_PILEDCOLOR_MAX = 26;
const PILED_COLORS = [
	"#3C1B31","#3C1B31","#3C1B31","#3C1B31",
	"#3C1B31","#3C1B31","#3C1B31",
]
const B_COLOR_TO_COLOR = (i) => {
	if (i > B_COLOR_MAX) {
		return PILED_COLORS[i%PILED_COLORS.length];
	}else{
		return COLORS[i%COLORS.length];
	}
}



class Board {
	constructor(blockSize, canvas) {
		this.BLOCK_SZ = blockSize;
		this.canvas = canvas;
		this.ctx = canvas.getContext('2d');
		this.w = 800;
		this.h = 640;
		this.states = new Array();
		this.states[0] = new Array();
		this.init();
	}

	init() {
		for (var x = 0; x < this.states.length; ++x) {
	    for (var y = 0; y < this.states[x].length; ++y ) {
				this.states[x][y] = B_EMPTY;
			}
		}
	}

	render() {
		this.ctx.clearRect( 0, 0, this.w, this.h );

	  for (var x = 0; x < this.states.length; ++x ) {
	    for (var y = 0; y < this.states[x].length; ++y ) {
				this.drawState(x, y, this.states[x][y]);
	    }
	  }
	}

	drawState(x, y, state) {
		switch (state) {
			case B_EMPTY:
			  this.ctx.fillStyle = '#EAEDF2';
			  this.ctx.strokeStyle = 'gray';
			  this.ctx.lineWidth = 1;
			  break;
			default:
			  this.ctx.fillStyle = B_COLOR_TO_COLOR(state);
			  this.ctx.strokeStyle = 'black';
			  this.ctx.lineWidth = 1;
			  break;
		}
		this.ctx.fillRect( this.BLOCK_SZ * x, this.BLOCK_SZ * y, this.BLOCK_SZ, this.BLOCK_SZ);
	  this.ctx.strokeRect( this.BLOCK_SZ * x, this.BLOCK_SZ * y, this.BLOCK_SZ, this.BLOCK_SZ);
	}

	setStates(states, render = false) {
		this.states = states;
		if (render) {
			this.render();
		}
	}
}


/////////////////////////////////////////////////////////////
// Data
// {
//    "op" : key
//    "d" : ???
// }
/////////////////////////////////////////////////////////////
function genKeyPressData(key) {
	return JSON.stringify({op: "key", d: key});
}

/////////////////////////////////////////////////////////////
// Application
/////////////////////////////////////////////////////////////
const KEYS = {
	122: 'l',  // h is left
	 99: 'r',  // c is rotate
	120: 'd',  // x is down
	115: 't'   // s is rotate(turn)
};

class Controller {
	constructor(board) {
		this.conn = new ServerConn(
			(d) => this.onServerCommand(d),
			(e) => this.onServerClosed(e)
		)
		this.board = board;
		this.run = false;
		this.isPlayer = false;
	}

	start() {
		if (!this.run) {
			this.conn.open(()=>logger.i("Server connected."));
			setInterval( () => this.onDraw(), 30 );
			this.run = true;
		}
		// register key events
		$(document.body).keypress((e) => {
			if (typeof KEYS[e.which] != 'undefined' ) {
				this.onKeyPress(KEYS[e.which]);
			}
		});
	}

	onServerCommand(arrayBuffer) {
		let view = new DataView(arrayBuffer);
		let op = view.getUint8(0);
		let dataHead = 1;
		let dataLen = arrayBuffer.byteLength - 1;

		switch (op) {
			case 0: // join
			  this.isPlayer = view.getUint8(dataHead) == 1 ? true : false;
				logger.i("join: is_player:" + this.isPlayer);
				break;
			case 1: // fin
				logger.i("disconnect");
				this.conn.close();
				break;
			case 2: // board
				let col = view.getUint8(dataHead);
				let row = view.getUint8(dataHead + 1);
				var newState = new Array(col);
				var offset = dataHead + 2;
				for (var x = 0; x < newState.length; ++x) {
					newState[x] = new Uint8Array(arrayBuffer, offset, row);
					offset += row;
				}
				logger.i(`updated board state. col=${col}, row=${row}`);
				this.board.setStates(newState, true);
				break;
			case 3: // game over
				logger.i("game over!!");
				this.conn.close();
				break;
			default:
				logger.e("unknown server data.");
				logger.e(op);
				break;
		}
	}

	onServerClosed(e) {
		logger.i("Closed.");
	}

	onDraw() {
		this.board.render();
	}

	onKeyPress(key) {
		this.conn.send(genKeyPressData(key));
	}
}


$(() => {
	let board = new Board(30, $('#board')[0]);
	let app = new Controller(board);
	app.start();
})
