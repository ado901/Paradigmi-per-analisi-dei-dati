var img_file = null;

function createCameraButtons() {
  var runButton = document.querySelector('#runButton');
  if (runButton == null) return setTimeout(createCameraButtons, 250);
  
  Blockly.getMainWorkspace().clear();

  var loaderDiv = document.createElement('div');
  loaderDiv.id = 'loaderDiv';
  loaderDiv.style.visibility="hidden";
  document.body.appendChild(loaderDiv);
  
  var modal = document.createElement('div');
  modal.id = 'modal-frame';
  var modalClose = document.createElement('div');
  modalClose.appendChild(document.createTextNode('×'));
  modalClose.id = 'modal-close';
  modal.appendChild(modalClose);
  modalClose.onclick = function () {
      modal.style.display = "none";
  }
  var modalImage = document.createElement('img');
  modalImage.id = 'modal-content';
  modal.appendChild(modalImage);
  document.querySelector('body').appendChild(modal);

  var input = document.createElement('input');
  input.type = "file";
  input.accept = "image/*";
  input.acquire = "";
  input.id = 'camera-button';
  input.onchange = function () {
    img_file = input.files[0];
    processImage(input.files[0]);
  };
  var img = document.createElement('img');
  img.src = 'camera.png';
  var label = document.createElement('label');
  label.appendChild(img);
  label.appendChild(input);
  label.htmlFor = 'camera-button';
  label.id = 'camera-label';

  var image = document.createElement('img');
  image.id = 'captured-image';
  image.src = 'common/1x1.gif';
  image.onclick = function () {
    modal.style.display = "block";
    modalImage.src = image.src;
  }

  var avviso = document.createElement("text");
  avviso.id = 'avviso';
  var t = document.createTextNode(".");
  avviso.appendChild(t);
  document.body.appendChild(avviso);

  runButton.parentNode.parentNode.querySelector('td:first-child').appendChild(label);
  runButton.parentNode.parentNode.querySelector('td:first-child').appendChild(image);

  runButton.addEventListener('click', function(event) {
      var xml = Blockly.Xml.workspaceToDom(Blockly.getMainWorkspace());
      var code = Blockly.Xml.domToText(xml);
      console.log('running: ' + code);

      var url = new URL(window.location.href);
      var player = url.searchParams.get("player");
      if (player) {
        bake_cookie('player',player,120);
      } else {
        player = read_cookie('player');
      }
      var level = url.searchParams.get("level");

      var form = new FormData(),
      xhr = new XMLHttpRequest();
      form.append('player', player);
      form.append('match',  "match_"+level);
      form.append('camera_input', img_file);
      form.append('solution_xml', code);
      xhr.open('post', '/upload_solution', true);
      xhr.send(form);
  }, false);
}

function bake_cookie(cookieName,cookieVal,cookieTime) {
  var deadline = new Date();
  var now = new Date();
  deadline.setTime(now.getTime() + (parseInt(cookieTime) * 60000));
  document.cookie = cookieName + '=' + escape(cookieVal) + '; expires=' + deadline.toGMTString() + '; path=/';
}

function read_cookie(cookieName) {
  if (document.cookie.length > 0) {
    var start = document.cookie.indexOf(cookieName + "=");
    if (start != -1) {
      start = start + cookieName.length + 1;
      var end = document.cookie.indexOf(";",start);
      if (end == -1) end = document.cookie.length;
      return unescape(document.cookie.substring(start,end));
    }
  }
  return "";
}

createCameraButtons();
