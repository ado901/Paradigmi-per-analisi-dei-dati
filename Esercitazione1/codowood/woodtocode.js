const LOOP = 10,
      IF_LEFT = 6,
      IF_RIGHT = 7,
      IF_FORWARD = 8,
      GO_FORWARD = 1,
      ELSE = 4,
      TURN_LEFT = 2,
      TURN_RIGHT = 3,
      END_IF = 9,
      END_LOOP = 5;

/*
sequence = { instruction }
instruction = “go_forward” | “turn_left” | “turn_right” | loop | if
loop = “loop” sequence “end_loop”
if = ( “if_forward” | “if_left” | “if_right” ) sequence else_end
else_end = “else” sequence “end_if” | “end_if”
*/

function processImage(file) {
  document.getElementById("avviso").style.backgroundColor="#ff00ff";
  document.getElementById("avviso").innerHTML = "inizio riconoscimento";
  document.getElementById("loaderDiv").style.visibility="visible";
  
  var reader = new FileReader();

  reader.onload = function (e) {
    var img = new Image();

    img.onload = function () {
      var w = 1200, h = 1200;
      if (img.height >= img.width) {
        w = Math.round(img.width * h / img.height);
      } else {
        h = Math.round(img.height * w / img.width);
      }
      var c = document.createElement('canvas');
      c.width = w;
      c.height = h;
      var ctx = c.getContext('2d');
      ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, w, h);
      var imageData = ctx.getImageData(0, 0, w, h);

      var detector = new AR.Detector();
      var markers = detector.detect(imageData);
      markers.sort(compareMarkers);
      markers = removeDuplicates(markers);
      var instructions = [];
      for (k of markers) {
        instructions.push(k.id);
        console.log("instr: " + k.id);
        ctx.fillStyle = 'Lime';
        ctx.fillText(k.id, k.corners[0].x, k.corners[0].y);
        ctx.fillStyle = 'Cyan';
        ctx.fillText(k.id, k.corners[1].x, k.corners[1].y);
        ctx.stroke();
      }
      document.querySelector('#captured-image').src = c.toDataURL();

      // AF ***
      document.getElementById("avviso").style.backgroundColor="#00ff00";
      document.getElementById("avviso").innerHTML = "fine riconoscimento";
      document.getElementById("loaderDiv").style.visibility="hidden";
      // ******
      console.log(instructions);
      code = parseSequence(instructions);

      if (instructions.length > 0) code = 'error';
      console.log(code);
      if (code=='error') {
        document.getElementById("avviso").style.backgroundColor="#ff0000";
        document.getElementById("avviso").innerHTML = "errore nei blocchi";
      }
      code = '<xml xmlns="http://www.w3.org/1999/xhtml">' + code + '</xml>';
      Blockly.getMainWorkspace().clear();
      var xml = Blockly.Xml.textToDom(code);
      Blockly.Xml.domToWorkspace(xml, Blockly.getMainWorkspace());
    };

    img.src = e.target.result;
  };
  reader.readAsDataURL(file);
}

function removeDuplicates(markers) {
  var diagonal = 0;
  for (i of markers) {
    var dx = i.corners[0].x - i.corners[2].x, dy = i.corners[0].y - i.corners[2].y;
    diagonal += Math.sqrt(dx * dx + dy * dy);
    dx = i.corners[1].x - i.corners[3].x, dy = i.corners[1].y - i.corners[3].y;
    diagonal += Math.sqrt(dx * dx + dy * dy);
  }
  diagonal /= 2 * markers.length;

  var result = [];
  for (i of markers) {
    var found = false;
    for (j of result) {
      if (i !== j && i.id == j.id &&
          Math.abs(i.corners[0].x - j.corners[0].x) < diagonal &&
          Math.abs(i.corners[0].y - j.corners[0].y) < diagonal) {
        found = true;
      }
    }
    if (!found) result.push(i);
  }
  return result;
}

function compareMarkers(a, b) {
  var a0 = a.corners[0],
      a1 = a.corners[1],
      b0 = b.corners[0],
      result = 0;
  if (Math.abs(a0.y - a1.y) <= Math.abs(a0.x - a1.x)) {
    result = a0.y - b0.y;
    if (a0.x > a1.x) result *= -1;
  } else {
    result = a0.x - b0.x;
    if (a0.y < a1.y) result *= -1;
  }
  return Math.sign(result);
}

function parseSequence(tokens) {
  var instructions = [];
  while (tokens.length > 0 && [LOOP,
                               IF_LEFT, IF_RIGHT, IF_FORWARD,
                               GO_FORWARD, TURN_LEFT, TURN_RIGHT].includes(tokens[0]) ) {
    var i = parseInstruction(tokens);
    if (i == 'error') return 'error';
    instructions.push(i);
  }
    
  var result = '';
  for (var i of instructions.reverse()) {
    if (result != '') {
      var index = i.lastIndexOf('<');
      result = i.slice(0, index) + '<next>' + result + '</next>' + i.slice(index);
    } else {
      result = i;
    }
  }
  return result;
}

function parseInstruction(tokens) {
  if (tokens[0] == LOOP) {
    return parseLoop(tokens);
  } else if ([IF_LEFT, IF_RIGHT, IF_FORWARD].includes(tokens[0])) {
    return parseIf(tokens);
  } else if (tokens[0] == GO_FORWARD) {
    tokens.splice(0, 1);
    return '<block type="maze_moveForward"></block>';
  } else if (tokens[0] == TURN_LEFT) {
    tokens.splice(0, 1);
    return '<block type="maze_turn"><field name="DIR">turnLeft</field></block>';
  } else if (tokens[0] == TURN_RIGHT) {
    tokens.splice(0, 1);
    return '<block type="maze_turn"><field name="DIR">turnRight</field></block>';
  }
  return 'error';
}

function parseLoop(tokens) {
  tokens.splice(0, 1);
  var inner = parseSequence(tokens);
  if (inner == 'error' || tokens[0] != END_LOOP) return 'error';
  tokens.splice(0, 1);
  return '<block type="maze_forever"><statement name="DO">' + inner + '</statement></block>';
}

function parseIf(tokens) {
  // TO DO 

  output='';
  direzione='';
  if (tokens[0]==IF_LEFT){
    
    direzione='isPathLeft';
    
  }
  if (tokens[0]==IF_FORWARD){
    direzione='isPathForward';
    
  }
  if (tokens[0]==IF_RIGHT){
    direzione='isPathRight';
    
  }
  tokens.splice(0, 1);
  output+='<field name="DIR">'+direzione+'</field><statement name="DO">'
  var inner = parseSequence(tokens);
  if (inner == 'error') return 'error';
  console.log('aaaa'+inner);
  if (tokens[0] ==ELSE|| tokens[0] == END_IF ){
    if (tokens[0]==END_IF) output='<block type="maze_if">'+ output + inner + '</statement>';
    else output='<block type="maze_ifElse">'+ output + inner + '</statement>';
    var outputElseEnd=parseElseEnd(tokens);
    console.log('bbbb'+outputElseEnd)
    if (outputElseEnd=='error') return 'error';
    console.log(output+outputElseEnd);
    return output+outputElseEnd;
  }
  
  

}

function parseElseEnd(tokens) {
  // TO DO
  if (tokens[0]==ELSE){
    tokens.splice(0, 1);
    var inner = parseSequence(tokens);
    if (inner == 'error') return 'error';
    console.log('cccc'+inner)
    if (tokens[0] != END_IF) return 'error';
    tokens.splice(0, 1);
    return '<statement name="ELSE">'+inner+'</statement></block>';
    }
  else if (tokens[0] == END_IF) {
    tokens.splice(0, 1)
    return '</block>';
  }
}



