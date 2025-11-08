const Upload_Button = document.getElementById("submit_button");
const File_Input = document.getElementById("fileUpload");
const File_Name = document.getElementById("fileName");

Upload_Button.addEventListener("click", processFile , false);

function processFile(e) {
  var file = File_Input.files[0];
  var size = file.size;
  var sliceSize = 256;
  var start = 0;

  setTimeout(loop, 1);

  function loop() {
    
    console.log("sending");
    var end = start + sliceSize;
    
    if (size - end < 0) {
      end = size;
    }
    
    var s = slice(file, start, end);

    send(s, start, end);

    if (end < size) {
      start += sliceSize;
      setTimeout(loop, 1);
    }
  }
}


function send(piece, start, end) {
  var formdata = new FormData();
  var xhr = new XMLHttpRequest();

  xhr.open('POST', '/upload_chunk', true);

  formdata.append('start', start);
  formdata.append('end', end);
  formdata.append('user_file', piece);

  xhr.send(formdata);
}

function slice(file, start, end) {
  var slice = file.mozSlice ? file.mozSlice :
              file.webkitSlice ? file.webkitSlice :
              file.slice ? file.slice : noop;
  
  return slice.bind(file)(start, end);
}

function noop() {
  
}


